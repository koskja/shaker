from abc import ABC, abstractmethod
from copy import deepcopy
import json
import re
import regex
from typing import Any, Callable, Dict, List, Optional, Set, Tuple, Union

from sympy import Integer

from helpers import *


class Context:
    types: Dict[str, "IType"]
    type_constructors: Dict[str, "ITypeConstructor"]
    native_typemap: Dict[str, str]
    used_idents: Set[str]

    def __init__(self, native: Dict[str, str]) -> None:
        self.types = {}
        self.type_constructors = {}
        self.used_idents = set()
        self.native_typemap = native

    def clone(self) -> "Context":
        return deepcopy(self)

    def insert(self, name: str, ty: Union["IType", "ITypeConstructor"]):
        if isinstance(ty, IType):
            if name in self.types:
                raise RuntimeError("Cannot override type")
            self.types[name] = ty
        elif isinstance(ty, ITypeConstructor):
            if name in self.type_constructors:
                self.type_constructors[name] = ConstructorList(
                    ty, self.type_constructors[name]
                )
            else:
                self.type_constructors[name] = ty
        else:
            raise RuntimeError("Invalid type")

    def contains(self, type: str) -> bool:
        return type in self.types or type in self.type_constructors

    def parse(self, name: str, type_def: str | List):
        if isinstance(type_def, str):
            if type_def == "native":
                if name in self.native_typemap:
                    self.insert(name, parse_external(self.native_typemap[name]))
                    if name == "void":
                        self.types["void"].void = True
                elif not self.contains(name):
                    raise RuntimeError("cringe")
            else:
                self.types[name] = TyAlias(make_camelcase(name), self.types[type_def])
        else:  # handle template
            assert isinstance(type_def, List)
            template, params = self.type_constructors[type_def[0]], type_def[1]
            if DelayedContructor.check(params):
                ty = DelayedContructor(template, params)
            else:
                ty = template.ctor(self, name, params)
            self.insert(name, ty)

    def parse_type(
        self,
        type_def: str | List,
        name: str,
        prefix=None,
        suffix=None,
        force_name=False,
    ) -> "IType":
        name = name if force_name else self.make_unique(name, prefix, suffix)
        if name not in self.types:
            if isinstance(type_def, str) and type_def in self.types:
                return self.types[type_def]
            self.parse(name, type_def)
        return self.types[name]

    def reserve_ident(self, name: str):
        self.used_idents.add(name)

    def make_unique(self, n: str, p: str | None, s: str | None):
        p = p or ""
        s = s or ""
        values = [[n], [p, n], [n, s], [p, n, s]]
        for segs in values:
            joined = "_".join(segs)
            if joined[0].isdigit():
                continue
            val = make_snakecase(joined)
            if val not in self.used_idents:
                last = ""
                while last != val:
                    last = val
                    opts = list(demangle_name(val, len(val.split("_"))))
                    new_opt = next(
                        filter(lambda x: x != val and x not in self.used_idents, opts),
                        None,
                    )
                    if new_opt is not None:
                        val = new_opt
                self.used_idents.add(val)
                return val
        return anon_ident()


def camelcased(func):
    """
    call `make_camelcase` on the second argument.
    """
    return lambda a, b, c: func(a, make_camelcase(b), c)


class IType(ABC):
    @abstractmethod
    def emit_ser(self, val: str) -> str:
        pass

    @abstractmethod
    def emit_de(self, previous: List[str]) -> str:
        pass

    @abstractmethod
    def emit_extra(self) -> str:
        pass

    @abstractmethod
    def name(self) -> str:
        pass

    def ty_name(self) -> str:
        return self.name().replace("<'a>", "").replace("<", "::<", 1)

    @abstractmethod
    def has_lifetime(self) -> bool:
        pass

    def discriminant_level(self) -> Integer:
        return 0


class ITypeConstructor(ABC):
    @abstractmethod
    def ctor(self, ctx: Context, name: str, params) -> IType:
        pass


class TypeConstructorDelegate(ITypeConstructor):
    func: Callable[[Context, str, Any], IType] = None

    def __init__(self, func) -> None:
        self.func = func

    def ctor(self, ctx: Context, name: str, params) -> IType:
        return self.func(ctx, name, params)


class DelayedContructor(ITypeConstructor):
    inner: ITypeConstructor = None
    params: str = None

    def __init__(self, inner: ITypeConstructor, params: Any) -> None:
        self.inner = inner
        self.params = json.dumps(params)

    def check(value: Any) -> bool:
        return "$" in json.dumps(value)

    def ctor(self, ctx: Context, name: str, params) -> IType:
        return self.inner.ctor(
            ctx, name, json.loads(Template(self.params).emit(params))
        )


class Container(IType):
    struct_name: str = ""
    fields: List[Tuple[str, IType]] = []

    def __init__(self, name, fields) -> None:
        self.struct_name = name
        self.fields = fields

    def emit_de(self, previous: List[Tuple[str, IType]]) -> str:
        if self.discriminant_level() == 0:
            return f"{self.ty_name()}::deserialize"
        else:
            return self.true_de(previous)

    def emit_ser(self, val) -> str:
        if self.discriminant_level() == 0:
            return f"let w = {self.ty_name()}::serialize(&{val}, w)?;"
        else:
            return self.true_ser(val)

    def true_de(self, previous) -> str:
        last = previous[-1][0]
        field_varname = lambda x: f"{last}_{x}"
        field_de = lambda ty, varname: ty.emit_de(previous + [(varname, ty)])
        fields = [
            (lambda x: f"let (input, {x}) = ({field_de(a[1], x)})(input)?;\n")(
                field_varname(a[0])
            )
            for a in self.fields
        ]
        return (
            "|input| {"
            + "".join(fields)
            + f"Ok((input, {self.struct_name} {{ "
            + "".join([f"{a[0]}: {field_varname(a[0])}, \n" for a in self.fields])
            + " })) }"
        )

    def true_ser(self, val) -> str:
        o = ""
        for name, ty in self.fields:
            o += ty.emit_ser(f"{val}.{name}") + "\n"
        return o

    def emit_extra(self) -> str:
        lifetime = ""
        if self.has_lifetime():
            lifetime = "<'a>"
        derive = '#[derive(protocol_lib::Packet)]\n' if max([a[1].discriminant_level() for a in self.fields] or [0]) == 0 else ''
        return (
            derive + 
            f"pub struct {self.struct_name}{lifetime} {{\n"
            + "".join(map(lambda a: f"{a[0]}: {a[1].name()}, \n", self.fields))
            + "}\n"
            + ((
                make_impl(self, de=Container.true_de, ser=Container.true_ser)
                if self.discriminant_level() == 0
                else ""
            ) if max([a[1].discriminant_level() for a in self.fields] or [0]) != 0 else '')
        )

    def name(self) -> str:
        return self.struct_name + ("<'a>" if self.has_lifetime() else "")

    def has_lifetime(self) -> bool:
        return any(map(lambda a: a[1].has_lifetime(), self.fields))

    def discriminant_level(self) -> Integer:
        return max(max([a[1].discriminant_level() for a in self.fields] or [0]) - 1, 0)

    def get_field_ty(self, name: str) -> Optional[IType]:
        return next(iter([a[1] for a in self.fields if a[0] == name] or [None]))

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> "Container":
        fields = []
        for item in params:
            if "anon" in item:
                item["name"] = anon_ident()
            else:
                item["anon"] = False
            ty = ctx.parse_type(item["type"], item["name"], prefix=name)
            if item["anon"] and isinstance(ty, Container):
                fields += ty.fields
            else:
                fields.append((make_snakecase(item["name"]), ty))
        return Container(name, fields)


class Mapper(IType):  # TODO: make this not suck - somehow skip intermediate str
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

    def emit_de(self, previous) -> str:
        arms = "".join(map(lambda x: f'"{x[0]}" => "{x[1]}",\n', self.arms.items()))
        if len(self.arms) == 0:
            return '|x| Ok((x, ""))'
        x = f"""
        let (input, x) = ({self.match_ty.emit_de(previous)})(input)?;
        let x = format!("{{x}}");
        let val = match &x[..] {{
            {arms}
            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
        }};
        Ok((input, val))
        """
        return f"|input| {{ {x} }}"

    def emit_ser(self, val) -> str:
        arms = "".join(map(lambda x: f'"{x[1]}" => "{x[0]}",\n', self.arms.items()))
        if len(self.arms) == 0:
            return ""
        return f""" 
        let tag = match &{val}[..] {{
            {arms}
            _ => panic!("invalid value")
            }};
        let tag2 = str::parse(tag).unwrap();
        {self.match_ty.emit_ser('tag2')}
        """

    def name(self) -> str:
        return "&'static str"

    def has_lifetime(self) -> bool:
        return False

    @camelcased
    def construct(ctx: Context, name: str, params: Context) -> "Mapper":
        match_ty = ctx.types[params["type"]]
        return Mapper(name, match_ty, params["mappings"])


class Switch(IType):
    def __init__(self, name, compare_to, fields, default) -> None:
        self._name = name
        self.compare_to: str = compare_to
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
                + ("" if is_void(x[1]) else f"({x[1].name()})")
                + ", \n",
                self.fields.values(),
            )
        )
        a += (
            "Default"
            + ("" if is_void(self.default) else f"({self.default.name()})")
            + ", \n"
        )
        a += "}\n"
        b = f"""
        impl{("<'a>" if self.has_lifetime() else "")} {self.name()} {{
            pub fn discriminant(&self) -> &'static str {{
                match self {{
                    {
                        ''.join(
                            [f'{self.ty_name()}::{a[1][0]}{"" if is_void(a[1][1]) else "(_)"} => "{a[0]}", ' for a in self.fields.items()]
                        )
                    }
                    _ => ""
                }}
            }}
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {{
                {self.true_ser('self')}
                Ok(w)
            }}
        }}
        """
        return a + b

    def emit_de(self, previous: List[Tuple[str, IType]]) -> str:
        compare_path = [
            make_snakecase(a) if a != ".." else a for a in self.compare_to.split("/")
        ]
        prev = [a for a in previous if not isinstance(a[1], Switch)]
        separator = "_"
        for seg in compare_path:
            if seg == "..":
                prev = prev[:-1]
            else:
                seg_ty = prev[-1][1].get_field_ty(seg)
                prev += [(prev[-1][0] + separator + seg, seg_ty)]
                if seg_ty.discriminant_level() == 0:
                    separator = "."
        compare_to = prev[-1][0]
        m = f'match &format!("{{}}", {compare_to})[..] {{\n'
        arms = "".join(
            [
                f'"{a[0]}" => nom::combinator::map({a[1][1].emit_de(previous+[(previous[-1][0], a[1][1])]) or "()"}, {self.ty_name()}::{a[1][0]})(input),\n'
                if not is_void(a[1][1])
                else f'"{a[0]}" => Ok((input ,{self.ty_name()}::{a[1][0]})),\n'
                for a in self.fields.items()
            ]
        )
        return (
            f"|input| {{ {m}{arms}"
            + (
                f"_ => nom::combinator::map({self.default.emit_de(previous+[(previous[-1][0], self.default)])}, {self.ty_name()}::Default)(input)"
                if not is_void(self.default)
                else f"_ => Ok((input, {self.ty_name()}::Default)),\n"
            )
            + "} }"
        )

    def emit_ser(self, val) -> str:
        return f"let w = {self.ty_name()}::serialize(&{val}, w)?;"

    def true_ser(self, val) -> str:
        arms = "".join(
            map(
                lambda x: f"{self.ty_name()}::{x[1][0]}"
                + (
                    f"(val) => {valued_ser(x[1][1], 'val')}, \n"
                    if not is_void(x[1][1])
                    else " => w,\n"
                ),
                self.fields.items(),
            )
        )
        x = f"""
        let w = match &{val} {{ {arms} {self.ty_name()}::Default{'(val)' if not is_void(self.default) else ''} => {f'{self.default.ty_name()}::serialize(val, w)?' if not is_void(self.default) else 'w'} }};
        """
        return x

    def name(self) -> str:
        return self._name + ("<'a>" if self.has_lifetime() else "")

    def has_lifetime(self) -> bool:
        return (
            any(map(lambda x: x[1].has_lifetime(), self.fields.values()))
            | self.default.has_lifetime()
        )

    def discriminant_level(self) -> Integer:
        levels = [a[1][1].discriminant_level() for a in self.fields.items()] or [0]
        return max(max(levels) - 1, self.compare_to.count("..") + 1)

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        if "default" not in params:
            params["default"] = "void"
        # breakpoint()
        fields = dict(
            map(
                lambda x: (
                    x[0],
                    (
                        lambda y: (
                            make_camelcase(y)
                            if x[0].isnumeric()
                            else make_camelcase(y).replace(name, ""),
                            ctx.parse_type(x[1], y, force_name=True),
                        )
                    )(ctx.make_unique(x[0], name, None)),
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
        return a + make_impl(self, Bitfield.true_de, Bitfield.true_ser)

    def emit_de(self, previous) -> str:
        return f"{self.ty_name()}::deserialize"

    def true_de(self, previous) -> str:
        name_list = "".join([a["name"] + ", " for a in self.fields])
        parse_list = "".join(
            [
                f"{'parse_bits_signed' if a['signed'] else 'parse_bits_unsigned'}({a['size']}), "
                for a in self.fields
            ]
        )
        return f"nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(nom::sequence::tuple(({parse_list})), |({name_list})| {self.name()} {{ {name_list} }}))"

    def emit_ser(self, val) -> str:
        return f"let w = {self.ty_name()}::serialize(&{val}, w)?;"

    def true_ser(self, val) -> str:
        fields = []
        for field in self.fields:
            if field["signed"]:
                fields.append(
                    f'unsafe {{ core::mem::transmute({val}.{field["name"]} as i64) }}'
                )
            else:
                fields.append(f'{val}.{field["name"]} as u64')
            fields[-1] = f'({fields[-1]}, {field["size"]}), '
        return f"""
        let w = write_bits(&[{''.join(fields)}], w)?;
        """

    def has_lifetime(self) -> bool:
        return False

    def get_field_ty(self, name: str) -> Optional[IType]:
        return next(
            iter(
                [NativeType(x["ty"]) for x in self.fields if x["name"] == name]
                or [None]
            )
        )

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        params = deepcopy(params)
        for x in params:
            x["name"] = make_snakecase(x["name"])
            x["ty"] = Bitfield.nearest_type(x["size"], x["signed"])
        return Bitfield(name, params)


class EntityMetadataLoop(IType):
    item: Container = None
    end_val: Integer

    def __init__(self, item: IType, end_val: Integer) -> None:
        self.item = item
        self.end_val = end_val

    def emit_extra(self) -> str:
        return ""

    def emit_de(self, previous) -> str:
        return f"""
            |mut input| {{
                let mut accum = vec![];
                loop {{
                    let (i, item) = {self.item.emit_de(previous)}(input)?;
                    input = i;
                    let index = item.key;
                    accum.push(item.value);
                    if index == 0xFF {{
                        break;
                    }}
                }}
                Ok((input, accum))
            }}
        """

    def emit_ser(self, val) -> str:
        return f"""
        let mut w = w;
        for (index, item) in {val}.iter().enumerate() {{
            w = u8::serialize(&if index == {val}.len() - 1 {{ 255 }} else {{ index as u8 }}, w)?;
            w = str::parse::<{self.item.get_field_ty('r_type').name()}>(item.discriminant()).unwrap().serialize(w)?;
            w = 
                {valued_ser(self.item.get_field_ty('value'), 'item')}
                
            
        }}
        """

    def name(self) -> str:
        return f"Vec<{self.item.get_field_ty('value').name()}>"

    def has_lifetime(self) -> bool:
        return self.item.has_lifetime()
    
    def discriminant_level(self) -> Integer:
        return 1000000

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        value = ctx.parse_type(params["type"], name + "Wrapper", suffix="Item")
        return EntityMetadataLoop(value, params["endVal"])


class TopbitTerminated(IType):
    key = None
    value = None

    def __init__(self, key: IType, item: IType) -> None:
        self.key = key
        self.item = item

    def emit_extra(self) -> str:
        return ""

    def emit_de(self, previous) -> str:
        return f""" |mut input| {{
            let mut val = std::collections::HashMap::new();
            loop {{
                let (i, (k_, v)) = nom::sequence::tuple((i8::deserialize, {self.item.ty_name()}::deserialize))(input)?;
                input = i;
                let k = k_ & 0x7F;
                val.insert(k, v);
                if k != k_ {{
                    break
                }}
            }}
            Ok((input, val)) }}
        """

    def emit_ser(self, val) -> str:
        assert self.key.name() == "i8"
        return f"""
            let mut w = w;
            for (i, (k, v)) in {val}.iter().enumerate() {{
                let k = if i == {val}.len() - 1 {{
                    *k | (1i8 << 7)
                }} else {{
                    *k
                }};
                let ww = i8::serialize(&k, w)?;
                w = v.serialize(ww)?;
            }}
        """

    def name(self) -> str:
        return f"std::collections::HashMap<{self.key.name()}, {self.item.name()}>"

    def has_lifetime(self) -> bool:
        return self.item.has_lifetime()

    def discriminant_level(self) -> Integer:
        return 1000000

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        container = params["type"][1]
        return TopbitTerminated(
            ctx.types[container[0]["type"]], ctx.types[container[1]["type"]]
        )


class ExternallyTaggedArray(IType):
    item: IType = None
    count: Optional[str] = ""
    count_ty: IType

    def __init__(self, item: IType, count: Optional[str], count_ty: IType) -> None:
        self.item = item
        self.count = count
        self.count_ty = count_ty

    def emit_extra(self) -> str:
        return ""

    def emit_de(self, previous) -> str:
        if not self.count:
            if not self.discriminant_level():
                return f"""
                    PrefixedArray::<{self.item.name()}, {self.count_ty.name()}>::deserialize
                """
            else:
                return f"""
                    |input| {{
                        let (input, len) = {self.count_ty.emit_de(previous)}(input)?;
                        let len = protocol_lib::types::num_traits::ToPrimitive::to_usize(&len).ok_or(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::TooLarge,
                        )))?;
                        nom::combinator::map(nom::multi::count({self.item.emit_de(previous)}, len), |x| {self.ty_name()}(x, core::marker::PhantomData))(input)
                    }}
                """

        else:
            return f"""
                |input| {{
                    let len = {previous[-2][0] + '_' + self.count};
                    let len = protocol_lib::types::num_traits::ToPrimitive::to_usize(&len).ok_or(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::TooLarge,
                    )))?;
                    nom::multi::count({self.item.emit_de(previous)}, len)(input)
                }}
            """

    def emit_ser(self, val) -> str:
        return (
            (
                f"""
            let w = {self.ty_name()}::len(&{val}).serialize(w)?;
        """
                if not self.count
                else ""
            )
            + f"""
            let mut w = w;
            let items = {val if self.count else val+'.0'}.iter();
            for i in items {{
                w = 
                    {valued_ser(self.item, 'i')}
            }}
        """
        )

    def name(self) -> str:
        return (
            f"Vec<{self.item.name()}>"
            if self.count
            else f"PrefixedArray<{self.item.name()}, {self.count_ty.name()}>"
        )

    def has_lifetime(self) -> bool:
        return self.item.has_lifetime()

    def discriminant_level(self) -> Integer:
        return max((1 if self.count else 0), self.item.discriminant_level())

    @camelcased
    def construct(ctx: Context, name: str, params: Dict) -> IType:
        return ExternallyTaggedArray(
            ctx.parse_type(params["type"], name, suffix="Item"),
            params.get("count"),
            ctx.parse_type(params.get("countType") or "varint", name, suffix="Count"),
        )


class TyAlias(IType):
    def __init__(self, name: str, alias: IType) -> None:
        self.n = name
        self.d = alias

    def emit_extra(self) -> str:
        return f"type {self.n} = {self.d.name()};"

    def emit_de(self, previous) -> str:
        return self.d.emit_de(previous)

    def emit_ser(self, val) -> str:
        return self.d.emit_ser(val)

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

    def emit_de(self, previous) -> str:
        return f"{self.ty_name()}::deserialize"

    def emit_ser(self, val) -> str:
        return f"let w = {self.ty_name()}::serialize(&{val}, w)?;"

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
        except ValueError:
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
                    params[k] = ctx.parse_type(v, name, suffix="Item").name()
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


def make_impl(
    ty: IType,
    de: Optional[Callable[[IType, List[str]], str]] = None,
    ser: Optional[Callable[[IType, str], str]] = None,
) -> str:
    de = de or ty.emit_de
    ser = ser or ty.emit_ser
    impl = (
        f"impl<'t: 'a, 'a> protocol_lib::Packet<'t>"
        if ty.has_lifetime()
        else f"impl<'t> protocol_lib::Packet<'t>"
    )
    return f"""
{impl} for {ty.name()} {{\n
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {{\n
        {ser(ty, 'self')}\n
        Ok(w)
    }}\n
\n
    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {{\n
        ({de(ty, [('self', ty)])})(input)\n
    }}\n
}}\n
    """


def is_void(ty) -> bool:
    return isinstance(ty, NativeType) and ty.void


def valued_ser(ty: IType, val: str) -> str:
    s = ty.emit_ser(val)
    if len(s.split('\n')) == 1: # let w = ...;
        return s[7:-1]
    else:
        return f'{{ {s} w }}'


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
ctx = Context(external_mapping["native"])

specials = {
    "container": Container,
    "mapper": Mapper,
    "switch": Switch,
    "bitfield": Bitfield,
    "entityMetadataLoop": EntityMetadataLoop,
    "topBitSetTerminatedArray": TopbitTerminated,
    "array": ExternallyTaggedArray,
}
for name, ty in specials.items():
    ctx.insert(name, TypeConstructorDelegate(getattr(ty, "construct")))

o = ""

print("\n".join(external_mapping["prelude"]["global"]) + "\n")

for name in protocol["types"].keys():
    ctx.reserve_ident(name)

for name, df in protocol["types"].items():
    ctx.parse(name, df)
for n, l in ctx.types.items():
    o += l.emit_extra()

protocol.pop("types")
base_ctx = ctx.clone()

for i, j in protocol.items():
    o += f"pub mod {i} {{\n"
    for k, l in j.items():
        o += f"pub mod {k} {{\n"
        o += "\n".join(external_mapping["prelude"]["all"]) + "\n"
        for name in l["types"].keys():
            ctx.reserve_ident(name)
        for name, df in l["types"].items():
            ctx.parse(name if name != 'packet' else k, df)
        for m in filter(lambda x: x not in base_ctx.types.keys(), ctx.types.keys()):
            xdd = ctx.types[m].emit_extra()
            o += xdd
        ctx = base_ctx.clone()
        o += "}\n"

    o += "}\n"

for [a, b] in external_mapping["regex"]:
    while True:
        n = regex.sub(a, b, o, flags=regex.MULTILINE)
        if o == n:
            break
        o = n

print(o)
