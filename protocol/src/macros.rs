/// Generate a struct-constructing closure that uses a tuple.
///
/// ```
/// #[derive(Debug, PartialEq)]
/// struct Position {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
/// fn main() {
///     use protocol::tuple_to_struct;
///     let constructor = tuple_to_struct![(x, y, z) -> Position];
///     let input = (0.0, 3.1, 4.2);
///     let output = constructor(input);
///     assert_eq!(output, Position { x: 0.0, y: 3.1, z: 4.2 });
/// }
/// ```
#[macro_export]
macro_rules! tuple_to_struct {
    (($($y:ident),+) -> $s:ident) => {
        |($($y),+,)| {$s { $($y),+ }}
    };
}
/// Creates a `nom::alt` combinator from its parameters.
/// A branch of the form `[$tag => $parse => $map]` outputs `map(preceded(tag([$tag]), $parse), $map)`.
/// The individual branches are then combined in a `nom::branch::alt` call.
/// ```ignore
/// enum_options!(
///     [0x3E => empty => Self::NoValue], // Produces `Self::NoValue` if the tag matches
///     [0xA5 => FooBar], //  <---------------------|
///     [0xA5 => FooBar::parse], //  <--------------|            
///     [0xA5 => FooBar => Self::FooBar], //  <-----|
///     [0xA5 => FooBar::parse => Self::FooBar], // These four produce identical code
///     [0x77 => Quag::mire => (|x: Quag| Self::Extreme(x.left, x.right))], // The map function can be arbitrary
///     [0x02 => guest::parse_function => Self::Special], // The parsing function can also be arbitrary
/// )
/// ```
#[macro_export]
macro_rules! enum_options {
    ($tag:literal => empty => $($m: tt)*) => {
        nom::combinator::map(nom::bytes::complete::tag([$tag]), |_| $($m)*)
    };
    ($tag:literal => $p:path => $($m: tt)*) => {
        nom::combinator::map(nom::sequence::preceded(nom::bytes::complete::tag([$tag]), $p), $($m)*)
    };
    ($tag:literal => $t:ident => $($m: tt)*) => {
        enum_options!($tag => $t::parse => $($m)*)
    };
    ($tag:literal => $t:ident::$f:ident) => {
        enum_options!($tag => $t::$f => Self::$t)
    };
    ($tag:literal => $t:ident) => {
        enum_options!($tag => $t::parse => Self::$t)
    };

    ($([$($t:tt)*]),* $(,)*) => {
        nom::branch::alt((
            $(
                enum_options!($($t)*)
            ),*
        ))
    };
}
/// Creates a struct parser from a list of its fields and their respective parsers.
///
/// This is done by splitting the fields and parsers, putting the field parsers into a `nom::tuple` parser and mapping `tuple_to_struct!` over the resulting tuple.
///
/// ```
/// let usage = struct_parse!(
///     prefix(digit0),
///     text(preceded(tag("chc"), alpha0))
/// );
/// let expanded = map(
///     tuple((
///         digit0,
///         preceded(tag("chc"), alpha0)
///     )),
///     |(prefix, text)| Self { prefix, text }
/// );
/// ```
#[macro_export]
macro_rules! struct_parse {
    (@accum ($field:ident($($p:tt)*), $($rem:tt)*) -> ($($parsers:tt)*) -> ($($fields:tt)*)) => {
        struct_parse!(@accum ($($rem)*) -> ($($parsers)* , [$($p)*]) -> ($($fields)* , $field))
    };
    (@accum ($field:ident($($p:tt)*)) -> ($($parsers:tt)*) -> ($($fields:tt)*)) => {
        struct_parse!(@accum () -> ($($parsers)* , [$($p)*]) -> ($($fields)* , $field))
    };
    (@accum () -> ($(,)* $([$($parsers:tt)*]),* ) -> ($(,)* $($fields:ident),*)) => {
        nom::combinator::map(
            nom::sequence::tuple((
                $(
                    $($parsers)*
                ),*,
            )),
            $crate::tuple_to_struct!(($($fields),*) -> Self)
        )
    };
    ($($t:tt)*) => {
        struct_parse!(@accum ($($t)*) -> () -> ())
    };
}
/// Produces a `parse` function with a `fn(&[u8]) -> IResult<&[u8], Self>` signature. The resuling function gives its `input: &[u8]` to this macro's parameter.
/// Conveniently combines `struct_parse!` and `parser_fn!`.
#[macro_export]
macro_rules! struct_parser_fn {
    ($life:lifetime => $($t:tt)*) => {
        pub fn parse(input: &$life[u8]) -> IResult<&$life[u8], Self> {
            struct_parse!($($t)*)(input)
        }
    };
    ($($t:tt)*) => {
        pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
            struct_parse!($($t)*)(input)
        }
    };
}
/// Conveniently combines `enum_options!` and `parser_fn!`.
#[macro_export]
macro_rules! enum_parser_fn {
    ($life:lifetime => $($t:tt)*) => {
        pub fn parse(input: &$life [u8]) -> IResult<&$life [u8], Self> {
            enum_options!($($t)*)(input)
        }
    };
    ($($t:tt)*) => {
        pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
            enum_options!($($t)*)(input)
        }
    };
}
#[macro_export]
macro_rules! struct_write {
    (@write $s:ident $field:ident(deref $($ser:tt)*)) => {
        $($ser)*(&*$s.$field)
    };
    (@write $s:ident $field:ident(ref $($ser:tt)*)) => {
        $($ser)*(&$s.$field)
    };
    (@write $s:ident $field:ident($($ser:tt)*)) => {
        $($ser)*($s.$field)
    };
    (@accum $s:ident () -> ($([$($t:tt)*])*)) => {
        cookie_factory::sequence::tuple(($($($t)*),*,))
    };
    (@accum $s:ident ($field:ident($($ser:tt)*), $($rem:tt)*) -> ($($body:tt)*)) => {
        struct_write!(@accum $s ($($rem)*) -> ([struct_write!(@write $s $field($($ser)*))] $($body)*))
    };
    ($s:ident -> $($field:ident($($ser:tt)*)),* $(,)*) => {
        struct_write!(@accum $s ($(
            $field($($ser)*)
        ),*   ,) -> ())
    };
}
#[macro_export]
macro_rules! struct_writer_fn {
    ($life:lifetime => $($t:tt)*) => {
        pub fn write<'__a, W: '__a + std::io::Write>(&$life self) -> impl SerializeFn<W> + '__a where $life: '__a {
            struct_write!(self -> $($t)*)
        }
    };
    ($($t:tt)*) => {
        pub fn write<'__a, W: '__a + std::io::Write>(&'__a self) -> impl SerializeFn<W> + '__a {
            struct_write!(self -> $($t)*)
        }
    };
}
#[macro_export]
macro_rules! wf {
    (@accum ($s:ident, $w:ident) -> ([$tag:literal => $field:ident => empty => $($fun:tt)*], $($rem:tt)*) -> ($($body:tt)*)) => {
        wf!(@accum ($s, $w) -> ($($rem)*) -> ([
            Self::$field => cookie_factory::sequence::pair(cookie_factory::bytes::be_u8($tag), $($fun)*())($w)
        ], $($body)*))
    };
    (@accum ($s:ident, $w:ident) -> ([$tag:literal => $field:ident => empty], $($rem:tt)*) -> ($($body:tt)*)) => {
        wf!(@accum ($s, $w) -> ($($rem)*) -> ([
            Self::$field => cookie_factory::bytes::be_u8($tag)($w)
        ], $($body)*))
    };
    (@accum ($s:ident, $w:ident) -> ([$tag:literal => $field:ident], $($rem:tt)*) -> ($($body:tt)*)) => {
        wf!(@accum ($s, $w) -> ($($rem)*) -> ([
            Self::$field(ref x) => cookie_factory::sequence::pair(cookie_factory::bytes::be_u8($tag), $field::write(x))($w)
        ], $($body)*))
    };
    (@accum ($s:ident, $w:ident) -> ([$tag:literal => $field:ident => $($fun:tt)*], $($rem:tt)*) -> ($($body:tt)*)) => {
        wf!(@accum ($s, $w) -> ($($rem)*) -> ([
            Self::$field(x) => cookie_factory::sequence::pair(cookie_factory::bytes::be_u8($tag), $($fun)*(x))($w)
        ], $($body)*))
    };
    (@accum ($s:ident, $w:ident) -> ([$tag:literal => ref $field:ident => $($fun:tt)*], $($rem:tt)*) -> ($($body:tt)*)) => {
        wf!(@accum ($s, $w) -> ($($rem)*) -> ([
            Self::$field(ref x) => cookie_factory::sequence::pair(cookie_factory::bytes::be_u8($tag), $($fun)*(x))($w)
        ], $($body)*))
    };

    (@accum ($s:ident, $w:ident) -> () -> ($([$($t:tt)*]),* $(,)*)) => {
        return match *$s {
            $($($t)*),*
        };
    };
    ($([$($t:tt)*]),* $(,)*) => {
        pub fn write<'__a, W: '__a + std::io::Write>(&'__a self) -> impl cookie_factory::SerializeFn<W> + '__a {
            move |w: cookie_factory::WriteContext<W>| {
                wf!(@accum (self, w) -> ($([$($t)*]),* ,) -> ());
            }
        }
    };
}
/// Writes an `Option<T>` by outputting `[0x01, T]` for `Some(T)` or `[0x00]` for `None`.
/// 
/// Syntax: `write_tagged_option!([value | ref | deref] ser_fn)`, where `ser_fn: impl FnOnce(_) -> impl SerializeFn<_>`
/// 
/// The first argument determines the way `T` should be passed to `ser_fn`.
/// - `value` - function takes a copied `T`
/// - `ref` - function takes a `&T`
/// - `deref` - function takes the deref target of `T` - `&<T as Deref>::Target`.
/// 
/// ```
/// use cookie_factory::*;
/// use protocol::write_tagged_option;
/// struct Large(pub [u8; 1024 * 1024]);
/// fn serialize_large<'a, W: 'a + std::io::Write>(x: &'a Large) -> impl SerializeFn<W> + 'a {
///     combinator::slice(&x.0)
/// }
/// fn write_to_slice(x: &mut [u8]) {
///     let by_value: Option<u16> = Some(0xBEEF);
///     let by_ref: Option<Large> = Some(Large([2u8; 1024 * 1024]));
///     let by_deref: Option<Vec<u8>> = Some(vec![42u8]);
///     gen(
///         sequence::tuple((
///             write_tagged_option!(value bytes::be_u16)(by_value),
///             write_tagged_option!(ref serialize_large)(&by_ref),
///             write_tagged_option!(deref combinator::slice)(&by_deref),
///         )),
///         x
///     ).unwrap();
/// }
/// ```
#[macro_export]
macro_rules! write_tagged_option {
    (@fin ($($f:tt)*) $($t:tt)*) => {
        move |x| { move |w: cookie_factory::WriteContext<_>| {
            if let Some(x) = $($f)*(x) {
                cookie_factory::sequence::pair(
                    cookie_factory::bytes::be_u8(0x01),
                    $($t)*(x)
                )(w)
            } else {
                cookie_factory::bytes::be_u8(0x01)(w)
            }
        } }
    };
    (ref $($t:tt)*) => { write_tagged_option!(@fin (Option::as_ref) $($t)*)};
    (deref $($t:tt)*) => { write_tagged_option!(@fin (Option::as_deref) $($t)*)};
    (value $($t:tt)*) => { write_tagged_option!(@fin ((|identity| identity)) $($t)*)};
}