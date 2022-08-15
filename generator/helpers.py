from copy import deepcopy
import re
from typing import List

import sys


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def split_words(s: str) -> List[str]:
    flat_map = lambda f, xs: (y for ys in xs for y in f(ys))
    return list(flat_map(lambda x: re.findall("[a-zA-Z0-9][^A-Z]*", x), s.split("_")))


def demangle_name(s: str, level):
    rules = [
        [r"ies_item", r"y"],
        [r"s_item", r""],
        [r"minecraft", r""],
        [r"brigadier", r""],
    ]
    if level > 2:
        rules.append([r"^packet_", r""])
    if level > 3:
        rules += [[r"attribute", r"attr"]]
    for find, replace in rules:
        v = re.sub(find, replace, s)
        yield v


def make_camelcase(s: str) -> str:
    return "".join(map(str.capitalize, make_ident(s)))


def make_snakecase(s: str) -> str:
    return "_".join(map(str.lower, make_ident(s)))


def anon_ident() -> str:
    if "counter" not in anon_ident.__dict__:
        anon_ident.counter = 0
    retval = f"Ident{anon_ident.counter}"
    anon_ident.counter += 1
    return retval


def make_unique(s: str, prefix: str | None = None, suffix: str | None = None) -> str:
    if "used" not in make_unique.__dict__:
        make_unique.used = set()
    used = make_unique.used
    if s not in used:
        used.add(s)
        return deepcopy(s)
    if isinstance(prefix, str) and (prefix + s) not in used:
        used.add(prefix + s)
        return prefix + s
    if isinstance(suffix, str) and (s + suffix) not in used:
        used.add(s + suffix)
        return s + suffix
    if (
        isinstance(prefix, str)
        and isinstance(suffix, str)
        and (prefix + s + suffix) not in used
    ):
        used.add(prefix + s + suffix)
        return prefix + s + suffix
    retval = anon_ident()
    used.add(retval)
    eprint(
        "cannot make unique ident from [",
        prefix,
        ", ",
        s,
        ", ",
        suffix,
        "], using ",
        retval,
    )
    return retval


def make_ident(s: str) -> List[str]:
    s = s.replace(":", "_")
    if s[0].isdigit():
        # breakpoint()
        s = "F" + s
    elif s in keywords:
        s = "r_" + s
    return split_words(s)


keywords = [
    "as",
    "use",
    "extern crate",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "if",
    "if let",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "for",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "impl",
    "ref",
    "return",
    "Self",
    "self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "alignof",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pure",
    "sizeof",
    "typeof",
    "unsized",
    "virtual",
    "yield",
]
