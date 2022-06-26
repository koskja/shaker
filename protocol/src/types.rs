use std::{
    io::Write,
    ops::{BitAnd, BitOr, Deref, Shl},
};

use cookie_factory::{SerializeFn, WriteContext};
use nom::{
    bits,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, map_res, verify},
    error::{ErrorKind, ParseError},
    multi::length_data,
    number::complete::be_u8,
    sequence::{preceded, tuple},
    IResult,
};
use num_traits::PrimInt;

pub struct VarInt;
impl VarInt {
    pub fn parse_i16(input: &[u8]) -> IResult<&[u8], i16> {
        varparse::<_, 3>(input)
    }
    pub fn parse_u16(input: &[u8]) -> IResult<&[u8], u16> {
        varparse::<_, 3>(input)
    }
    pub fn parse_i32(input: &[u8]) -> IResult<&[u8], i32> {
        varparse::<_, 5>(input)
    }
    pub fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
        varparse::<_, 5>(input)
    }
    pub fn parse_i64(input: &[u8]) -> IResult<&[u8], i64> {
        varparse::<_, 10>(input)
    }
    pub fn parse_u64(input: &[u8]) -> IResult<&[u8], u64> {
        varparse::<_, 10>(input)
    }
    pub fn write<T: PrimInt, W: std::io::Write>(val: T) -> impl SerializeFn<W> {
        varwrite(val)
    }
}
pub fn varparse<T, const L: usize>(input: &[u8]) -> IResult<&[u8], T>
where
    T: BitAnd<T, Output = T> + BitOr<T, Output = T> + From<u8> + PartialEq + Shl<usize, Output = T>,
{
    let mut result: T = 0u8.into();
    for (pos, &val) in input.iter().take(L).enumerate() {
        let trimmed_byte: T = (val & 0x7F).into();
        result = result | (trimmed_byte << (pos * 7));
        if val & 0x80 != 0x80 {
            return IResult::Ok((&input[pos + 1..], result));
        }
    }
    IResult::Err(nom::Err::Error(nom::error::Error::new(
        input,
        ErrorKind::TooLarge,
    )))
}
pub fn varwrite<T: PrimInt, W: std::io::Write>(val: T) -> impl SerializeFn<W> {
    move |mut w: WriteContext<W>| {
        let mut val = val;
        let mut quit = false;
        loop {
            let mut to_write = [(val & T::from(0x78u8).unwrap()).to_u8().unwrap()];
            val = val >> 7;
            if !val.is_zero() {
                to_write[0] |= 0x80;
            } else {
                quit = true;
            }
            w.write(&to_write[..])?;
            if quit {
                return Ok(w);
            }
        }
    }
}
pub fn bool(input: &[u8]) -> IResult<&[u8], bool> {
    map(be_u8, |x| x != 0)(input)
}
pub fn tagged_option<'a, F, O, E>(p: F) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Option<O>, E>
where
    F: FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>,
    E: ParseError<&'a [u8]>,
{
    alt((
        map(tag([0x00]), |_| None),
        map(preceded(tag([0x01]), p), Some),
    ))
}
pub fn remaining_slice(input: &[u8]) -> IResult<&[u8], &[u8]> {
    Ok((&[], input))
}
/// Reads a `VarInt` and takes that many bytes, turning them into a slice. Trying to read more than `max` bytes causes an `ErrorKind::Verify`.
pub fn limited_slice<'a>(max: u32) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    length_data(verify(VarInt::parse_u32, move |&x| x <= max))
}
/// Reads a `VarInt` and interprets that many bytes as a `str` slice. Trying to read more than `max` bytes or an invalid UTF-8 string causes an error.
pub fn limited_str<'a>(max: u32) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], &'a str> {
    map_res(limited_slice(max), |s| {
        std::str::from_utf8(s)
    })
}
pub fn prefixed_str(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(length_data(map(VarInt::parse_u32, |x| x as usize)), |x| {
        std::str::from_utf8(x)
    })(input)
}
pub fn write_str<'a, 'b: 'a, 'c: 'a, W: 'b + std::io::Write>(
    s: &'c str,
) -> impl SerializeFn<W> + 'a {
    cookie_factory::sequence::pair(
        VarInt::write(s.len()),
        cookie_factory::combinator::string(s),
    )
}
pub fn position(input: &[u8]) -> IResult<&[u8], (i32, i32, i32)> {
    use nom::bits::complete::take as take_bits;
    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(tuple((
        take_bits(26usize),
        take_bits(26usize),
        take_bits(12usize),
    )))(input)
}

pub fn write_slice<'a, W: 'a + std::io::Write, T: 'a + Deref<Target = [u8]>>(
    bytes: T,
) -> impl SerializeFn<W> + 'a {
    move |w: WriteContext<W>| { cookie_factory::combinator::slice(bytes.deref())(w) }
}
pub fn write_prefixed_slice<'a, W: 'a + std::io::Write, T: 'a + Deref<Target = [u8]>>(
    bytes: T,
) -> impl SerializeFn<W> + 'a {
    move |w: WriteContext<W>| { 
        let bytes = bytes.deref();
        cookie_factory::sequence::pair(
        VarInt::write(bytes.len()),
        cookie_factory::combinator::slice(bytes),
    )(w) }
}
