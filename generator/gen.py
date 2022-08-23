from core import *
from comptypes import *
import regex

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
            w = item.discriminant().serialize(w)?;
            w = 
                {valued_ser(self.item.get_field_ty('value'), 'item')}
                
            
        }}
        """

    def name(self) -> str:
        return f"Vec<{self.item.get_field_ty('value').name()}>"

    def has_lifetime(self) -> bool:
        return self.item.has_lifetime()
    
    def discriminant_level(self) -> Integer:
        return 0

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
        return 0

    @camelcased
    def construct(ctx: Context, name: str, params: Any) -> IType:
        container = params["type"][1]
        return TopbitTerminated(
            ctx.types[container[0]["type"]], ctx.types[container[1]["type"]]
        )



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
ctx = Context(external_mapping["native"], external_mapping["literals"])

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
import optpass
optpass.run_all(ctx)
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
        optpass.run_all(ctx)
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
