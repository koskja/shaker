from core import *
from comptypes import *

def type_annotations(ctx: Context):
    def get_type(prev: List[Tuple[str, IType]], path: str) -> IType:
        filtered = [a[1] for a in prev if isinstance(a[1], Container)] 
        ty = None
        for seg in path.split('/'):
            if seg == '..':
                filtered = filtered[:-1]
            else:
                ty = ty or filtered[-1]
                ty = ty.get_field_ty(make_snakecase(seg))
        return ty

    def annotate(ty: IType, prev: List[Tuple[str, IType]]):
        if '_annotated' in ty.__dict__:
            return
        if isinstance(ty, Container):
            for a in ty.fields:
                annotate(a[1], prev+[(a[0], a[1])])
        elif isinstance(ty, Mapper):
            annotate(ty.match_ty, prev+[("", ty.match_ty)])
        elif isinstance(ty, Bitfield):
            pass
        elif isinstance(ty, ExternallyTaggedArray):
            if ty.count:
                ty.count_ty = get_type(prev, ty.count)
            annotate(ty.item, prev + [("", ty.item)])
        elif isinstance(ty, Switch):
            for a in ty.fields:
                if a.value:
                    annotate(a.value, prev+[(a.name, a.value)])
            ty.dty = get_type(prev, ty.compare_to)
        ty.__dict__['_annotated'] = True
    for ty in ctx.types.values():
        try:
            annotate(ty, [("", ty)])
        except:
            pass

def mapper_switch_reduce(ctx: Context):
    for ty in ctx.types.values():
        if isinstance(ty, Switch) and isinstance(ty.dty, Mapper):
            variants = { a.discriminant: a for a in ty.fields if a.discriminant }
            v2 = []
            for md, mv in ty.dty.arms.items(): 
                if mv in variants:
                    f = deepcopy(variants[mv])
                    f.discriminant = md
                    v2.append(f)
            v2.sort(key=lambda x: x.discriminant)
            default = { "xd": a for a in ty.fields if not a.discriminant }
            if default.get("xd"):
                v2.append(default["xd"])
            ty.dty.proxy_replace(ty.dty.match_ty)
            ty.fields = v2

def run_all(ctx: Context):
    type_annotations(ctx)
    mapper_switch_reduce(ctx)