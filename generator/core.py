
from abc import ABC, abstractmethod
from copy import deepcopy
import json
import re
from typing import Any, Callable, Dict, List, Optional, Set, Tuple, Union

from sympy import Integer

from helpers import *


class Context:
    """ Stores the context for parsing types from the protocol. Stores already parsed types and constructors."""
    types: Dict[str, "IType"]
    type_constructors: Dict[str, "ITypeConstructor"]
    native_typemap: Dict[str, str]
    literals: Dict[str, 'Literal']
    used_idents: Set[str] # set of all names in the output program

    def __init__(self, native: Dict[str, str], literals: Dict[str, Dict[str, str]]) -> None:
        self.types = {}
        self.type_constructors = {}
        self.used_idents = set()
        self.native_typemap = native
        self.literals = { a: Literal(b['inner'], b['lit'], b['new'], b['unwrap']) for a, b in literals.items()}
        for ty in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64"]:
            if ty not in self.literals:
                self.literals[ty] = Literal(ty, f"{{}}{ty}", "{}", "{}")
        if 'bool' not in self.literals:
            self.literals["bool"] = Literal("bool", "{}", "{}", "{}")

    def clone(self) -> "Context":
        return deepcopy(self)

    def insert(self, name: str, ty: Union["IType", "ITypeConstructor"]):
        """ Insert a type or a type constructor."""
        if isinstance(ty, IType):
            if name in self.types:
                raise RuntimeError("Cannot override type")
            self.types[name] = TypeProxy(ty) # use a TypeProxy shim to allow replacing types
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
        """ Parse and store a `type_def`. `type_def` can be a type name or a template instantiation list (e.g. ['container', [...]]"""
        def parse_external(s: str) -> Union['NativeType', 'Template']:
            if len(Template(s).generics()) == 0:
                return NativeType(s)
            else:
                return Template(s)
        if isinstance(type_def, str):
            if type_def == "native":
                if name in self.native_typemap:
                    self.insert(name, parse_external(self.native_typemap[name]))
                    if name in self.literals:
                        self.types[name].__dict__['literal'] = self.literals[name]
                    if name == "void": 
                        self.types["void"].void = True
                elif not self.contains(name):
                    raise RuntimeError(f"Type {name} defined as `native` but not present in native typemap")
            else:
                self.types[name] = TyAlias(make_camelcase(name), self.types[type_def])
        else:  # handle template
            assert isinstance(type_def, List)
            template, params = self.type_constructors[type_def[0]], type_def[1]
            if DelayedConstructor.check(params):
                ty = DelayedConstructor(template, params)
            else:
                ty = template.ctor(self, name, params)
            if name in self.literals:
                ty.__dict__['literal'] = self.literals[name]
            self.insert(name, ty)

    def parse_type(
        self,
        type_def: str | List,
        name: str,
        prefix=None,
        suffix=None,
        force_name=False,
    ) -> "IType":
        """ Parse a `type_def` and return the resulting type. Returns already parsed types when passed a `str` for `type_def`."""
        if isinstance(type_def, str) and type_def in self.types:
            return self.types[type_def]
        name = name if force_name else self.make_unique(name, prefix, suffix) 
        self.parse(name, type_def)
        return self.types[name]

    def reserve_ident(self, name: str):
        self.used_idents.add(name)

    def make_unique(self, n: str, p: str | None, s: str | None):
        """ Produces a unique identifier(one that was not yet used in this context) by pre/appending `p` or `s`."""
        p = p or ""
        s = s or ""
        values = [[n], [p, n], [n, s], [p, n, s]]
        for segs in values:
            joined = "_".join(segs)
            if joined[0].isdigit():
                continue
            val = make_snakecase(joined)
            if val not in self.used_idents:
                val = demangle_name(val, len(val.split('_')), lambda x: x not in self.used_idents)
                self.used_idents.add(val)
                return val
        return anon_ident()


def camelcased(func):
    """
    call `make_camelcase` on the second argument.
    """
    return lambda a, b, c: func(a, make_camelcase(b), c)

class IType(ABC):
    """ A generic type. Can be serialized or deserialized. """
    @abstractmethod
    def emit_ser(self, val: str) -> str:
        """ Returns a piece of code that serializes `val` by writing to `w: WriteContext<W: Write>` - e.g. \\
        `let w = {val}.serialize(w)?;"""
        pass

    @abstractmethod
    def emit_de(self, previous: List[Tuple[str, 'IType']]) -> str:
        """ Returns a callable function of the type `Fn(&[u8]) -> nom::IResult<&[u8], Self>`. 
        This function may capture other variables for its execution. 

        `previous` serves as a way to 'track' the call stack, allowing types that require other fields to refer to them,\\
        even if they are higher up. The list consists of `(variable_name_prefix, variable_type)`.
        """
        pass

    @abstractmethod
    def emit_extra(self) -> str:
        """ Returns a list of top level definitions required for this type."""
        pass

    @abstractmethod
    def name(self) -> str:
        """ This type's output name."""
        pass

    def ty_name(self) -> str:
        """ This type's output name in turbofish format."""
        return self.name().replace("<'a>", "").replace("<", "::<", 1)

    @abstractmethod
    def has_lifetime(self) -> bool:
        """ Whether this type has a generic liftime parameter."""
        pass

    def discriminant_level(self) -> Integer:
        """ How many levels up in the `emit_de` call chain are required to produce a correct deserialization function. 
        """
        return 0
    
    def is_trivial(self) -> bool:
        """ This type implements `Packet` and thus has a `discriminant_level` of zero."""
        return False
    

class NativeType(IType):
    """ Represents the simplest, no-strings-attached type. 
    Defined intrinsically or explicitly by type mappings. 
    Represented as `native` in the protocol."""
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
    """ Constructs a type in a given context, given some instance-specific parameters."""
    @abstractmethod
    def ctor(self, ctx: Context, name: str, params: Any) -> IType:
        pass


class TypeConstructorDelegate(ITypeConstructor):
    """ Wraps a callable object and delegates its `ctor` to it."""
    func: Callable[[Context, str, Any], IType] = None

    def __init__(self, func) -> None:
        self.func = func

    def ctor(self, ctx: Context, name: str, params) -> IType:
        return self.func(ctx, name, params)


class DelayedConstructor(ITypeConstructor):
    """ 'Delays' a type's construction until its template parameters are provided. 
        Can be used for template switch types with `compareTo` as their template param."""
    inner: ITypeConstructor = None
    params: str = None

    def __init__(self, inner: ITypeConstructor, params: Any) -> None:
        self.inner = inner
        self.params = json.dumps(params)

    def check(value: Any) -> bool:
        """ Checks whether a type's definition contains any `$`, indicating template parameters"""
        return "$" in json.dumps(value)

    def ctor(self, ctx: Context, name: str, params) -> IType:
        return self.inner.ctor(
            ctx, name, json.loads(Template(self.params).emit(params))
        )

class TyAlias(IType):
    """ Serves as a transparent type alias."""
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
    """ Uses a fallback type constructor if the first throws a `ValueError`. 
    Useful for templated types with optional parameters."""
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
    """ The 'simplest' type constructor. Does a find-and-replace 
    of `$ty1`-style template parameters in a given string and declares the result a `NativeType`."""
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
        """ Replace all template arguments according to ``generics``."""
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
    """ Create a `Packet` implementation using `ty: IType`. Serialization and deserialization can be overriden with `de` and `ser`."""
    de = de or ty.emit_de
    ser = ser or ty.emit_ser
    impl_params = (
        f"'t: 'a, 'a"
        if ty.has_lifetime()
        else f"'t"
    )
    return f"""
        impl<{impl_params}> Packet<'t> for {ty.name()} {{
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {{
                {ser(ty, 'self')}
                Ok(w)
            }}

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {{
                ({de(ty, [('self', ty)])})(input)
            }}
        }}
    """

class Literal:
    inner: str
    lit: str
    new: str
    unwrap: str
    def __init__(self, inner: str, lit: str, new: str, unwrap: str) -> None:
        self.inner = inner
        self.lit = lit
        self.new = new
        self.unwrap = unwrap

def is_void(ty) -> bool:
    return isinstance(ty, NativeType) and ty.void

def literal_of(ty: IType) -> Optional[Literal]:
    if 'literal' in ty.__dict__:
        return ty.literal
    else:
        return None

def valued_ser(ty: IType, val: str) -> str:
    """ Outputs a serialization expression block that returns w."""
    s = ty.emit_ser(val)
    if len(s.split('\n')) == 1: # Optimize single-line serialization
        return s[7:-1] # let w = ...;
    else:
        return f'{{ {s} w }}'

class TypeProxy:
    __slots__ = ["_obj", "__weakref__"]

    def __init__(self, obj: IType) -> None:
        object.__setattr__(self, "_obj", obj)
    
    def __getattribute__(self, name):
        if name == 'proxy_replace': # delegate all attributes, except proxy_replace, to the proxied type
            return lambda obj: object.__setattr__(self, "_obj", obj) # proxy_replace(self, ty) replaces the inner type
        else:
            return getattr(object.__getattribute__(self, "_obj"), name)
    def __delattr__(self, name):
        delattr(object.__getattribute__(self, "_obj"), name)
    def __setattr__(self, name, value):
        setattr(object.__getattribute__(self, "_obj"), name, value)
    
    def __nonzero__(self):
        return bool(object.__getattribute__(self, "_obj"))
    def __str__(self):
        return str(object.__getattribute__(self, "_obj"))
    def __repr__(self):
        return repr(object.__getattribute__(self, "_obj"))
    
    @property
    def __class__(self): # shamelessly lie about what class this is
        return self._obj.__class__