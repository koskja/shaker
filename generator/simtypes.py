from comptypes import *

class DelegatedOption(IType): #`Switch` with `Some`/`None` variants, depending on an arbitrary field
    compare_to: str
    value: Switch.Field
    dty: IType
    _name: str

    def __init__(self, name: str, compare_to: str, value: Switch.Field, dty: IType) -> None:
        self.compare_to = compare_to
        self.value = value
        self.dty = dty
    def emit_extra(self) -> str:
        return ""
    def emit_de(self, previous: List[Tuple[str, 'IType']]) -> str:
        compare_path = [
            make_snakecase(a) if a != ".." else a for a in self.compare_to.split("/")
        ]
        prev = [a for a in previous if isinstance(a[1], Container)]
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
        l = literal_of(self.dty)
        m = f"""
            match {l.unwrap.format(compare_to)} {{
                {l.lit.format(self.value.discriminant)} => map({self.value.value.emit_de(previous)}, Some)(input),
                _ => Ok((input, None))
            }}"""
        return f"|input| {{ {m} }}"
    def emit_ser(self, val: str) -> str:
        return f"""
            let w = match &{val} {{
                Some(val) => {valued_ser(self.value.value, 'val')},
                None => w,
            }};
        """
    def name(self) -> str:
        return f"Option<{self.value.value.name()}>"

    def has_lifetime(self) -> bool:
        return self.value.value.has_lifetime()