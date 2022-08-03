from abc import ABC, abstractmethod
from copy import deepcopy
import json
import re
from typing import Any, Callable, Dict, List, Union

from sympy import Integer, public

from helpers import *


class IType(ABC):
    @abstractmethod
    def emit_ser(self, val) -> str:
        pass

    @abstractmethod
    def emit_de(self) -> str:
        pass

    @abstractmethod
    def emit_extra(self) -> str:
        pass

    @abstractmethod
    def name(self) -> str:
        pass

    @abstractmethod
    def has_lifetime(self) -> bool:
        pass


class ITypeConstructor(ABC):
    @abstractmethod
    def ctor(
        self, type_map: Dict[str, Union[IType, "ITypeConstructor"]], name: str, params
    ) -> IType:
        pass


class TypeConstructorDelegate(ITypeConstructor):
    func: Callable[[Dict[str, Union[IType, "ITypeConstructor"]], str, Any], IType] = None

    def __init__(self, func) -> None:
        self.func = func

    def ctor(
        self, type_map: Dict[str, Union[IType, "ITypeConstructor"]], name: str, params
    ) -> IType:
        return self.func(type_map, name, params)

class DelayedContructor(ITypeConstructor):
    inner: ITypeConstructor = None
    params: str = None
    def __init__(self, inner: ITypeConstructor, params: Any) -> None:
        self.inner = inner
        self.params = json.dumps(params)
    def check(value: Any) -> bool:
        return '$' in json.dumps(value)
    def ctor(self, type_map: Dict[str, Union[IType, "ITypeConstructor"]], name: str, params) -> IType:
        return self.inner.ctor(type_map, name, json.loads(Template(self.params).emit(params)))

class Container(IType):
    struct_name = "INVALID"
    fields = []

    def __init__(self, name, fields) -> None:
        self.struct_name = name
        self.fields = fields
        super().__init__()

    def emit_de(self) -> str:
        fields = "".join(
            map(lambda a: f"let (input, {a[0]}) = {a[1].emit_de()};\n", self.fields)
        )
        return (
            fields
            + "Ok((input, Self { "
            + "".join(map(lambda a: f"{a[0]}, ", self.fields))
            + " }))"
        )

    def emit_ser(self) -> str:
        pass

    def emit_extra(self) -> str:
        lifetime = ""
        if self.has_lifetime():
            lifetime = "<'a>"
        return (
            f"pub struct {self.struct_name}{lifetime} {{\n"
            + "".join(map(lambda a: f"{a[0]}: {a[1].name()}, \n", self.fields))
            + "}\n"
        )

    def name(self) -> str:
        return self.struct_name + ("<'a>" if self.has_lifetime() else "")

    def has_lifetime(self) -> bool:
        return any(map(lambda a: a[1].has_lifetime(), self.fields))

    def construct(type_map, name, params) -> "Container":
        fields = []
        for item in params:
            if "anon" in item:
                item["name"] = anon_ident()
            ty = parse2(make_unique(make_camelcase(item["name"]), prefix=name), item["type"], type_map)
            fields.append([make_snakecase(item["name"]), ty])
        return Container(name, fields)


class Mapper(IType):
    ty_name = ""
    match_ty = None
    arms = {}

    def __init__(self, ty_name, match_ty, arms) -> None:
        self.ty_name = ty_name
        self.match_ty = match_ty
        self.arms = arms
        super().__init__()

    def emit_extra(self) -> str:
        return ""

    def emit_de(self) -> str:
        a = f"""let branch_{self.ty_name} = {self.match_ty.emit_de()};\n
                let {self.ty_name} = match branch_{self.ty_name} {{\n"""
        a += "".join(
            map(
                lambda x: f'x if str::parse("{x[0]}").unwrap() == x => "{x[1]}", \n',
                self.arms.items(),
            )
        )
        a += "}"
        return a

    def emit_ser(self, val) -> str:
        return super().emit_ser(val)

    def name(self) -> str:
        return "&'static str"

    def has_lifetime(self) -> bool:
        return False

    def construct(type_map, name, params) -> "Mapper":
        match_ty = type_map[params["type"]].name()
        return Mapper(name, match_ty, params["mappings"])


class Switch(IType):
    def __init__(self, name, compare_to, fields, default) -> None:
        self._name = name
        self.compare_to = compare_to
        self.fields = fields
        self.default = default
        super().__init__()

    def emit_extra(self) -> str:
        lifetime = ""
        if self.has_lifetime():
            lifetime = "<'a>"
        a = f"pub enum {self._name}{lifetime} {{\n"
        a += "".join(
            map(
                lambda x: f"{x[0]}"
                + (
                    ""
                    if isinstance(x[1], NativeType) and x[1].void
                    else f"({x[1].name()})"
                )
                + ", \n",
                self.fields.items(),
            )
        )
        a += 'Default' + ('' if isinstance(self.default, NativeType)
        and self.default.void else f'({self.default.name()})') + ', \n'
        a += "}\n"
        return a

    def emit_de(self) -> str:
        return super().emit_de()

    def emit_ser(self, val) -> str:
        return super().emit_ser(val)

    def name(self) -> str:
        return self._name + ("<'a>" if self.has_lifetime() else "")

    def has_lifetime(self) -> bool:
        return any(map(lambda x: x.has_lifetime(), self.fields.values())) | self.default.has_lifetime()

    def construct(type_map, name, params) -> IType:
        if "default" not in params:
            params["default"] = "void"
        # breakpoint()
        fields = dict(
            map(
                lambda x: (
                    make_camelcase(x[0]),
                    parse2(make_unique(make_camelcase(x[0]), prefix=name), x[1], type_map),
                ),
                params["fields"].items(),
            )
        )
        # fv = list(fields.values())
        # if len(fv) == 2:
        #    if (isinstance(fv[0], NativeType) and fv[0].void):
        #        return NativeType(f'Option<{fv[1].name()}>')
        #    elif (isinstance(fv[1], NativeType) and fv[1].void):
        #        return NativeType(f'Option<{fv[0].name()}>')
        return Switch(
            name,
            params["compareTo"],
            fields,
            parse2(make_unique(f"Default", prefix=name), params["default"], type_map),
        )


# class DelegatedOption: `Switch` with `Some`/`None` variants, depending on an arbitrary field


class Bitfield(IType):
    _name = ""
    fields = []

    def __init__(self, name, fields) -> None:
        self._name = name
        self.fields = fields
        super().__init__()

    def name(self) -> str:
        return self._name

    def next_power_of_2(x):
        return 1 if x == 0 else 2 ** (x - 1).bit_length()

    def nearest_type(bits: Integer, signed: bool) -> str:
        return ("i" if signed else "u") + str(max(Bitfield.next_power_of_2(bits), 8))

    def emit_extra(self) -> str:
        a = f"pub struct {self.name()} {{\n"
        a += "".join(
            map(
                lambda x: f"{x['name']}: {Bitfield.nearest_type(x['size'], x['signed'])}, \n",
                self.fields,
            )
        )
        a += "}\n"
        return a

    def emit_de(self) -> str:
        return super().emit_de()

    def emit_ser(self, val) -> str:
        return super().emit_ser(val)

    def has_lifetime(self) -> bool:
        return False

    def construct(type_map, name, params) -> IType:
        a = Bitfield(name, params)
        return a


class EntityMetadata(IType):
    item = None

    def __init__(self, item: IType) -> None:
        self.item = item

    def emit_extra(self) -> str:
        return ""

    def emit_de(self) -> str:
        return super().emit_de()

    def emit_ser(self, val) -> str:
        return super().emit_ser(val)

    def name(self) -> str:
        return f"Vec<{self.item.name()}>"

    def has_lifetime(self) -> bool:
        return self.item.has_lifetime()

    def construct(type_map, name, params) -> IType:
        return EntityMetadata(parse2(make_unique(name), params["type"][1][1]["type"], type_map))


class TopbitTerminated(IType):
    key = None
    value = None

    def __init__(self, key: IType, item: IType) -> None:
        self.key = key
        self.item = item

    def emit_extra(self) -> str:
        return ""

    def emit_de(self) -> str:
        return super().emit_de()

    def emit_ser(self, val) -> str:
        return super().emit_ser(val)

    def name(self) -> str:
        return f"std::collections::HashMap<{self.key.name()}, {self.item.name()}>"

    def has_lifetime(self) -> bool:
        return self.item.has_lifetime()

    def construct(type_map, name, params) -> IType:
        container = params["type"][1]
        return TopbitTerminated(
            type_map[container[0]["type"]], type_map[container[1]["type"]]
        )


class ExternallyTaggedArray(IType):
    item: IType = None
    count: str = ""

    def __init__(self, item: IType, count: str) -> None:
        self.item = item
        self.count = count

    def emit_extra(self) -> str:
        return ""

    def emit_de(self) -> str:
        return super().emit_de()

    def emit_ser(self, val) -> str:
        return super().emit_ser(val)

    def name(self) -> str:
        return f"Vec<{self.item.name()}>"

    def has_lifetime(self) -> bool:
        return self.item.has_lifetime()

    def construct(
        type_map: Dict[str, Union[IType, "ITypeConstructor"]], name: str, params
    ) -> IType:
        return ExternallyTaggedArray(
            parse2(make_unique(name, suffix='Item'), params["type"], type_map), params["count"]
        )


class NativeType(IType):
    _name = ""
    void = False

    def __init__(self, name: str) -> None:
        self._name = name
        super().__init__()

    def emit_de(self) -> str:
        return f"{self._name}::deserialize(input)?"

    def emit_ser(self, val) -> str:
        return f"{self._name}::serialize({val})(w)"

    def emit_extra(self) -> str:
        return ""

    def name(self) -> str:
        return self._name

    def has_lifetime(self) -> bool:
        return "'a" in self._name

class ConstructorList(ITypeConstructor):
    a: ITypeConstructor = None
    b: ITypeConstructor = None

    def __init__(self, a: ITypeConstructor, b: ITypeConstructor) -> None:
        self.a = a
        self.b = b

    def ctor(self, type_map, name, params) -> IType:
        try:
            return self.a.ctor(type_map, name, params)
        except:
            return self.b.ctor(type_map, name, params)

class Template(ITypeConstructor):
    value = ""

    def __init__(self, s: str) -> None:
        self.value = s

    def ctor(self, type_map, name, params) -> NativeType:
        if isinstance(params, dict):
            for k, v in params.items():
                if not isinstance(v, str):
                    params[k] = parse2(make_unique(name), v, type_map).name()
                else:
                    if v in type_map:
                        params[k] = type_map[v].name()
        else:
            params = parse2(make_unique(name), params, type_map).name()

        return NativeType(self.emit(params))

    def emit(self, generics: str | Dict[str, str]) -> str:
        if isinstance(generics, str):  # handle funny `option` type
            return self.value.replace("$0", generics)

        if set(self.generics()) != set(generics.keys()):
            raise ValueError(
                f"""The provided template parameters <{list(generics.keys())}>
                     do not match this template <{self.generics()}>"""
            )
        output = self.value
        for k, v in generics.items():
            if isinstance(v, IType):
                output = output.replace("$" + k, v.name())
            else:
                output = output.replace("$" + k, v)
        return output

    def generics(self) -> List[str]:
        return list(
            map(lambda x: x.group()[1:], re.finditer(r"\$[a-zA-Z0-9_]+", self.value))
        )


def parse_external(s: str) -> NativeType | Template:
    if len(Template(s).generics()) == 0:
        return NativeType(s)
    else:
        return Template(s)


def getVer(version: str):
    with open("/home/koskja/shaker/minecraft-data/data/dataPaths.json") as f:
        s = json.load(f)
        s = s["pc"][version]["protocol"]
        with open(
            "/home/koskja/shaker/minecraft-data/data/" + s + "/protocol.json"
        ) as g:
            return g.read()


def parse2(name, type_def, type_map) -> IType | ITypeConstructor:
    if isinstance(type_def, str):
        if type_def in type_map:
            return type_map[type_def]
        raise RuntimeError("bullshit")

    assert isinstance(type_def, list)  # this is a templated type
    template_name, params = type_def[0], type_def[1]

    if DelayedContructor.check(params):
        new_type = DelayedContructor(type_map[template_name], params)
    else:
        new_type = type_map[template_name].ctor(type_map, make_camelcase(name), params)
    if name in type_map:
        if isinstance(type_map[name], ITypeConstructor):
            type_map[name] = ConstructorList(type_map[name], new_type)
        else:
            raise RuntimeError('cannot override type')
    else:
        type_map[name] = new_type
    return type_map[name]


def parse_type(definition, type_map, external_mapping):
    name, type_def = definition
    if type_def == "native":
        if name not in external_mapping:
            if name in type_map:
                return
            raise RuntimeError(
                f"""protocol declares type {name} as `native`
                 but it isn't present in external mappings"""
            )
        else:  # save templated types unexpanded
            ty = parse_external(external_mapping[name])
            if name == "void":
                ty.void = True

        if name in type_map and isinstance(type_map[name], ITypeConstructor):
            type_map[name] = ConstructorList(type_map[name], ty)
        else:
            type_map[name] = ty
        return

    if isinstance(type_def, str):
        assert type_def in type_map
        type_map[name] = type_map[type_def]
        return

    type_map[name] = parse2(make_unique(name), type_def, type_map)


protocol = json.loads(getVer("1.18"))
external_mapping = json.load(open("/home/koskja/shaker/type_mapping.json"))
type_map: Dict[str, Union[IType, ITypeConstructor]] = {}
type_map["container"] = TypeConstructorDelegate(Container.construct)
type_map["mapper"] = TypeConstructorDelegate(Mapper.construct)
type_map["switch"] = TypeConstructorDelegate(Switch.construct)
type_map["bitfield"] = TypeConstructorDelegate(Bitfield.construct)
type_map["entityMetadataLoop"] = TypeConstructorDelegate(EntityMetadata.construct)
type_map["topBitSetTerminatedArray"] = TypeConstructorDelegate(
    TopbitTerminated.construct
)
type_map["array"] = TypeConstructorDelegate(ExternallyTaggedArray.construct)

print("\n".join(external_mapping["prelude"]["global"]) + "\n")

for l in protocol["types"].items():
    parse_type(l, type_map, external_mapping["native"])
for l in type_map.values():
    if isinstance(l, IType):
        print(l.emit_extra())

o = ""
protocol.pop("types")
base_type_map = deepcopy(type_map)
base_idents = deepcopy(make_unique.used)

for i, j in protocol.items():
    o += f"pub mod {i} {{\n"
    for k, l in j.items():
        o += f"pub mod {k} {{\n"
        o += "\n".join(external_mapping["prelude"]["all"]) + "\n"
        for m in l["types"].items():
            parse_type(m, type_map, external_mapping)
        for m in filter(lambda x: x not in base_type_map.keys(), type_map.keys()):
            xdd = type_map[m].emit_extra()
            o += xdd
        type_map = deepcopy(base_type_map)
        make_unique.used = deepcopy(base_idents)
        o += "}\n"

    o += "}\n"
print(o)
