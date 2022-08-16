from typing import Tuple
from core import *

class Container(IType):
    struct_name: str = ""
    fields: List[Tuple[str, IType]] = []

    def __init__(self, name, fields) -> None:
        self.struct_name = name
        self.fields = fields

    def emit_de(self, previous) -> str:
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

    def emit_ser(self, val) -> str:
        o = ""
        for name, ty in self.fields:
            o += ty.emit_ser(f"{val}.{name}") + "\n"
        return o

    def emit_extra(self) -> str:
        return (
            f"""
            pub struct {self.name()} {{
                {"".join([f"{a[0]}: {a[1].name()}, \n" for a in self.fields])}
            }}"""
        )

    def name(self) -> str:
        return self.struct_name + ("<'a>" if self.has_lifetime() else "")

    def has_lifetime(self) -> bool:
        return any(map(lambda a: a[1].has_lifetime(), self.fields))

    def discriminant_level(self) -> Integer:
        return max(max([a[1].discriminant_level() for a in self.fields] or [0]) - 1, 0)

    def get_field_ty(self, name: str) -> Optional[IType]:
        return next(iter([a[1] for a in self.fields if a[0] == name] or [None]))

    def is_trivial(self) -> bool:
        return max([a[1].discriminant_level() for a in self.fields] or [0]) == 0

    def specialize(self) -> Union['Container' , 'SufficientContainer' , 'TrivialContainer']:
        if self.discriminant_level() == 0:
            if self.is_trivial():
                return TrivialContainer(self.struct_name, self.fields)
            return SufficientContainer(self.struct_name, self.fields)
        else:
            return Container(self.struct_name, self.fields)

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
        return Container(name, fields).specialize()

class SufficientContainer(Container):
    def emit_extra(self) -> str:
        return Container.emit_extra(self) + make_impl(self, de=Container.emit_de, ser=Container.emit_ser)
    def emit_ser(self, val: str) -> str:
        return f'let w = {self.ty_name()}::serialize(&{val}, w)?;'
    def emit_de(self, previous) -> str:
        return f'{self.ty_name()}::deserialize'

class TrivialContainer(SufficientContainer):
    def emit_extra(self) -> str:
        return '#[derive(protocol_lib::Packet)]' + Container.emit_extra(self)

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

    def is_trivial(self) -> bool:
        return True

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        params = deepcopy(params)
        for x in params:
            x["name"] = make_snakecase(x["name"])
            x["ty"] = Bitfield.nearest_type(x["size"], x["signed"])
        return Bitfield(name, params)

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

    def is_trivial(self) -> bool:
        return not self.count and not self.discriminant_level()

    @camelcased
    def construct(ctx: Context, name: str, params: Dict) -> IType:
        return ExternallyTaggedArray(
            ctx.parse_type(params["type"], name, suffix="Item"),
            params.get("count"),
            ctx.parse_type(params.get("countType") or "varint", name, suffix="Count"),
        )
