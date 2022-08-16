
from abc import ABC, abstractmethod
from copy import deepcopy
import json
import re
from typing import Any, Callable, Dict, List, Optional, Set, Union

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
        def parse_external(s: str) -> Union['NativeType', 'Template']:
            if len(Template(s).generics()) == 0:
                return NativeType(s)
            else:
                return Template(s)
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
    
    def is_trivial(self) -> bool:
        return False

class NativeType(IType):
    _name = ""
    void = False

    def __init__(self, name: str) -> None:
        self._name = name

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
    
    def is_trivial(self) -> bool:
        return True

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

    def is_trivial(self) -> bool:
        return self.d.is_trivial()



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