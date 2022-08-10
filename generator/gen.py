from abc import ABC, abstractmethod
from copy import deepcopy
import json
import re
from typing import Any, Callable, Dict, List, Set, Union

from sympy import Integer, public

from helpers import *

class Context:
    types: Dict[str, 'IType']
    type_constructors: Dict[str, 'ITypeConstructor']
    native_typemap: Dict[str, str]
    used_idents: Set[str]

    def __init__(self, native: Dict[str, str]) -> None:
        self.types = {}
        self.type_constructors = {}
        self.used_idents = set()
        self.native_typemap = native
    
    def clone(self) -> 'Context':
        return deepcopy(self)
    
    def insert(self, name: str, ty: Union['IType',  'ITypeConstructor']):
        if isinstance(ty, IType):
            if name in self.types:
                raise RuntimeError('Cannot override type')
            self.types[name] = ty
        elif isinstance(ty, ITypeConstructor):
            if name in self.type_constructors:
                self.type_constructors[name] = ConstructorList(ty, self.type_constructors[name])
            else:
                self.type_constructors[name] = ty
        else:
            raise RuntimeError('Invalid type')
    
    def contains(self, type: str) -> bool:
        return type in self.types or type in self.type_constructors
    
    def parse(self, name: str, type_def: str | List):
        if isinstance(type_def, str):
            if type_def == 'native':
                if name in self.native_typemap:
                    self.insert(name, parse_external(self.native_typemap[name]))
                    assert name == make_unique(name)
                elif not self.contains(name):
                    raise RuntimeError('cringe')
            else:
                self.types[name] = TyAlias(name, self.types[type_def])
        else: # handle template
            assert isinstance(type_def, List)
            template, params = self.type_constructors[type_def[0]], type_def[1]
            if DelayedContructor.check(params):
                ty = DelayedContructor(template, params)
            else:
                ty = template.ctor(self, name, params)
            self.insert(name, ty)

    def parse_type(self, type_def: str | List, name: str, prefix = None, suffix = None) -> 'IType':
        name = self.__make_unique(name, prefix, suffix)
        if name not in self.types:
            if isinstance(type_def, str) and type_def in self.types:
                return self.types[type_def]
            self.parse(name, type_def)
        return self.types[name]

    def reserve_ident(self, name: str):
        self.used_idents.add(name)
    
    def __make_unique(self, n: str, p: str | None, s: str | None):
        p = p or ''
        s = s or ''
        values = [n, p + n, n + s, p + n + s]
        for val in values:
            if val not in self.used_idents:
                self.used_idents.add(val)
                return val
        return anon_ident()
        
def camelcased(func):
    return lambda a, b, c: func(a, make_camelcase(b), c)


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
        self, ctx: Context, name: str, params
    ) -> IType:
        pass


class TypeConstructorDelegate(ITypeConstructor):
    func: Callable[[Context, str, Any], IType] = None

    def __init__(self, func) -> None:
        self.func = func

    def ctor(
        self, ctx: Context, name: str, params
    ) -> IType:
        return self.func(ctx, name, params)

class DelayedContructor(ITypeConstructor):
    inner: ITypeConstructor = None
    params: str = None
    def __init__(self, inner: ITypeConstructor, params: Any) -> None:
        self.inner = inner
        self.params = json.dumps(params)
    def check(value: Any) -> bool:
        return '$' in json.dumps(value)
    def ctor(self, ctx: Context, name: str, params) -> IType:
        return self.inner.ctor(ctx, name, json.loads(Template(self.params).emit(params)))

class Container(IType):
    struct_name = "INVALID"
    fields = []

    def __init__(self, name, fields) -> None:
        self.struct_name = name
        self.fields = fields

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

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> "Container":
        fields = []
        for item in params:
            if "anon" in item:
                item["name"] = anon_ident()
            ty = ctx.parse_type(item["type"], item["name"], prefix=name)
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

    @camelcased
    def construct(ctx: Context, name: str, params: Context) -> "Mapper":
        match_ty = ctx.types[params["type"]].name()
        return Mapper(name, match_ty, params["mappings"])


class Switch(IType):
    def __init__(self, name, compare_to, fields, default) -> None:
        self._name = make_camelcase(name)
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
                lambda x: f"{make_camelcase(x[0])}"
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

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        if "default" not in params:
            params["default"] = "void"
        # breakpoint()
        fields = dict(
            map(
                lambda x: (
                    x[0],
                    ctx.parse_type(x[1], x[0], prefix=name),
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
            ctx.parse_type(params["default"], f"Default", prefix=name),
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

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        return Bitfield(name, params)


class EntityMetadataLoop(IType):
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

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        return EntityMetadataLoop(ctx.parse_type(params["type"][1][1]["type"], f'{name}Item'))


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

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        container = params["type"][1]
        return TopbitTerminated(
            ctx.types[container[0]["type"]], ctx.types[container[1]["type"]]
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

    @camelcased
    def construct(
        ctx: Context, name: str, params
    ) -> IType:
        return ExternallyTaggedArray(
            ctx.parse_type(params["type"], name, suffix='Item'), params["count"]
        )

class TyAlias(IType):
    def __init__(self, name: str, alias: IType) -> None:
        self.n = name
        self.d = alias

    def emit_extra(self) -> str:
        return f'type {self.n} = {self.d.name()};'

    def emit_de(self) -> str:
        return super().emit_de()

    def emit_ser(self, val) -> str:
        return super().emit_ser(val)

    def name(self) -> str:
        return self.n

    def has_lifetime(self) -> bool:
        return self.d.has_lifetime()

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

    def ctor(self, ctx: Context, name: str, params: Any) -> IType:
        try:
            return self.a.ctor(ctx, name, params)
        except:
            return self.b.ctor(ctx, name, params)

class Template(ITypeConstructor):
    value = ""

    def __init__(self, s: str) -> None:
        self.value = s

    def ctor(self, ctx: Context, name, params) -> NativeType:
        params = deepcopy(params)
        if isinstance(params, dict):
            for k, v in params.items():
                if not isinstance(v, str):
                    params[k] = ctx.parse_type(v, f'{name}Ty').name()
                else:
                    if v in ctx.types:
                        params[k] = ctx.types[v].name()
        else:
            params = ctx.parse_type(params, name).name()

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


protocol = json.loads(getVer("1.18"))
external_mapping = json.load(open("/home/koskja/shaker/type_mapping.json"))
ctx = Context(external_mapping['native'])

specials = {
"container": Container,
"mapper": Mapper,
"switch": Switch,
"bitfield": Bitfield,
"entityMetadataLoop": EntityMetadataLoop,
"topBitSetTerminatedArray": 
    TopbitTerminated,
"array":ExternallyTaggedArray
}
for name, ty in specials.items():
    ctx.insert(name, TypeConstructorDelegate(getattr(ty, 'construct')))

print("\n".join(external_mapping["prelude"]["global"]) + "\n")

for name in protocol["types"].keys():
    ctx.reserve_ident(name)

for name, df in protocol["types"].items():
    ctx.parse(name, df)
for n, l in ctx.types.items():
    print(l.emit_extra())

o = ""
protocol.pop("types")
base_ctx = ctx.clone()
base_idents = deepcopy(make_unique.used)

for i, j in protocol.items():
    o += f"pub mod {i} {{\n"
    for k, l in j.items():
        o += f"pub mod {k} {{\n"
        o += "\n".join(external_mapping["prelude"]["all"]) + "\n"
        for name in l["types"].keys():
            ctx.reserve_ident(name)
        for name, df in l["types"].items():
            ctx.parse(name, df)
        for m in filter(lambda x: x not in base_ctx.types.keys(), ctx.types.keys()):
            xdd = ctx.types[m].emit_extra()
            o += xdd
        ctx = base_ctx.clone()
        make_unique.used = deepcopy(base_idents)
        o += "}\n"

    o += "}\n"
print(o)
