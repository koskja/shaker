use std::{
    io::{Cursor, Write},
    marker::PhantomData,
    ops::Deref, fmt::Display,
};

use cookie_factory::{GenError, GenResult, WriteContext};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, map_opt, map_res},
    multi::{length_count, length_data, count},
    sequence::preceded,
    IResult,
};
pub use num_traits::{self, PrimInt, NumCast, Signed, Unsigned};
use protocol_derive::SerializeFn;

pub use super::varint::VInt;
use crate::Packet;
pub use uuid::Uuid;

// FIXME: Serialize/deserialize everything as signed, even unsigned types

#[derive(Debug)]
pub struct LimitedSlice<'a, const MAX: usize>(pub &'a [u8]);
impl<'t: 'a, 'a, const MAX: usize> Packet<'t> for LimitedSlice<'a, MAX> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        assert!(self.0.len() <= MAX);
        cookie_factory::sequence::pair(
            VInt(self.0.len() as u32),
            cookie_factory::combinator::slice(self.0),
        )(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        nom::combinator::map(
            nom::multi::length_data(nom::combinator::verify(
                VInt::<u32>::deserialize_prim::<usize>,
                |&x| x <= MAX,
            )),
            |x| Self(x),
        )(input)
    }
}
#[derive(Debug)]
pub struct LimitedString<'a, const MAX: usize>(pub &'a str);
impl<'t: 'a, 'a, const MAX: usize> Packet<'t> for LimitedString<'a, MAX> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        assert!(self.0.len() <= MAX);
        cookie_factory::sequence::pair(
            VInt(self.0.len() as u32),
            cookie_factory::combinator::string(self.0),
        )(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        nom::combinator::map_res(
            nom::multi::length_data(nom::combinator::verify(
                VInt::<u32>::deserialize_prim::<usize>,
                |&x| x <= MAX,
            )),
            |x| std::str::from_utf8(x).map(Self),
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeFn)]
pub struct Bool(pub bool);
impl<'a> Packet<'a> for Bool {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        cookie_factory::bytes::be_u8(if self.0 { 0x00 } else { 0x01 })(w)
    }

    fn deserialize(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        nom::combinator::map(nom::number::complete::be_u8, |x| Self(x != 0))(input)
    }
}
pub struct PrefixedBuffer<'a, T>(pub &'a [u8], pub PhantomData<T>);
impl<'t: 'a, 'a, T: Packet<'t> + PrimInt> Packet<'t> for PrefixedBuffer<'a, T> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        let length = <T as NumCast>::from(self.0.len()).ok_or_else(|| GenError::CustomError(0))?;
        let w = length.serialize(w)?;
        cookie_factory::combinator::slice(self.0)(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        map(
            length_data(map_opt(T::deserialize, |x| x.to_usize())),
            |x| Self(x, PhantomData),
        )(input)
    }
}
impl<'a, T> Deref for PrefixedBuffer<'a, T> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
pub struct PrefixedString<'a, T>(pub &'a str, pub PhantomData<T>);
impl<'t: 'a, 'a, T: Packet<'t> + PrimInt> Packet<'t> for PrefixedString<'a, T> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        PrefixedBuffer(self.0.as_bytes(), PhantomData::<T>).serialize(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        map(
            map_res(PrefixedBuffer::<T>::deserialize, |x| {
                std::str::from_utf8(x.0)
            }),
            |x| Self(x, PhantomData),
        )(input)
    }
}
impl<'a, T> Display for PrefixedString<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub struct PrefixedArray<T, U>(pub Vec<T>, pub PhantomData<U>);
impl<'t, T: Packet<'t>, U: Packet<'t> + PrimInt> Packet<'t>
    for PrefixedArray<T, U>
{
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        let length = <U as NumCast>::from(self.0.len()).ok_or_else(|| GenError::CustomError(0))?;
        let mut w = length.serialize(w)?;
        w = self.serialize_ext(w)?;
        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        map(
            length_count(map_opt(U::deserialize, |x| x.to_usize()), T::deserialize),
            |x| Self(x, PhantomData),
        )(input)
    }
}
impl<'t, T: Packet<'t>, U: Packet<'t> + PrimInt> PrefixedArray<T, U> {
    pub fn deserialize_ext(input: &'t [u8], size: U) -> IResult<&'t [u8], Self> {
        let size = size.to_usize().ok_or(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )))?;
        map(count(T::deserialize, size), |x| Self(x, PhantomData))(input)
    }
    pub fn serialize_ext<W: Write>(&self, mut w: WriteContext<W>) -> GenResult<W> {
        for i in &self.0 {
            w = i.serialize(w)?;
        }
        Ok(w)
    }
}
impl<T, U: PrimInt> PrefixedArray<T, U> {
    pub fn len(&self) -> U {
        U::from(self.0.len()).unwrap()
    }
}
impl<'t> Packet<'t> for uuid::Uuid {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        let (top, bot) = self.as_u64_pair();
        let w = cookie_factory::bytes::be_u64(top)(w)?;
        cookie_factory::bytes::be_u64(bot)(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        map_res(take(16usize), Self::from_slice)(input)
    }
}
impl<'t: 'a, 'a, T: Packet<'t> + 'a> Packet<'t> for Option<T> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        if let Some(value) = self {
            let w = cookie_factory::bytes::be_u8(0x01)(w)?;
            value.serialize(w)
        } else {
            cookie_factory::bytes::be_u8(0x00)(w)
        }
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        alt((
            map(tag([0x00]), |_| None),
            preceded(tag([0x01]), map(T::deserialize, Some)),
        ))(input)
    }
}
pub struct Void;
impl<'t> Packet<'t> for Void {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        GenResult::Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        Ok((input, Self))
    }
}
pub struct RestBuffer<'a>(pub &'a [u8]);
impl<'t: 'a, 'a> Packet<'t> for RestBuffer<'a> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        cookie_factory::combinator::slice(self.0)(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        Ok((&[], Self(input)))
    }
}

pub struct Nbt(pub quartz_nbt::NbtCompound, pub String);
impl<'t> Packet<'t> for Nbt {
    fn serialize<W: Write>(&self, mut w: WriteContext<W>) -> GenResult<W> {
        quartz_nbt::io::write_nbt(
            &mut w,
            Some(&self.1),
            &self.0,
            quartz_nbt::io::Flavor::Uncompressed,
        )
        .map_err(|_| GenError::CustomError(4))?;
        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        let mut c = Cursor::new(input);
        let x = quartz_nbt::io::read_nbt(&mut c, quartz_nbt::io::Flavor::Uncompressed).map_err(
            |_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)),
        )?;
        Ok((&input[c.position() as usize..], Self(x.0, x.1)))
    }
}

pub struct OptionalNbt(pub Option<Nbt>);
impl<'t> Packet<'t> for OptionalNbt {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        match self.0 {
            Some(ref x) => x.serialize(w),
            None => cookie_factory::bytes::be_u8(0x00)(w),
        }
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        alt((
            map(tag([0x00]), |_| Self(None)),
            map(Nbt::deserialize, |x| Self(Some(x))),
        ))(input)
    }
}

impl<'t> Packet<'t> for bool {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        if *self { 1u8 } else { 0 }.serialize(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        map_res(u8::deserialize, |x| match x { 0 => Ok(false), 1 => Ok(true), _ => Err(nom::error::ErrorKind::Alt) })(input)
    }
}

pub fn parse_bits_signed<T: PrimInt + Signed>(len: usize) -> impl Fn((&[u8], usize)) -> nom::IResult<(&[u8], usize), T> {
    move |input| {
        nom::combinator::map(nom::bits::complete::take(len), parse_signed_be(len))(input)
    }
}
pub fn parse_bits_unsigned<T: PrimInt + Unsigned>(len: usize) -> impl Fn((&[u8], usize)) -> nom::IResult<(&[u8], usize), T> {
    move |input| {
        nom::combinator::map(nom::bits::complete::take(len), parse_unsigned_be(len))(input)
    }
}
pub fn write_bits<W: Write>(values: &[(u64, usize)], mut w: WriteContext<W>) -> GenResult<W> {
    let mut accum = vec![];
    for (value, len) in values {
        let mut value = *value;
        for _ in 0..*len {
            accum.push((value & 0x1) as u8);
            value = value.unsigned_shl(1);
            if accum.len() == 8 {
                w = cookie_factory::bytes::be_u8(accum.drain(..).fold(0, |a, b| (a >> 1) | (b << 7)))(w)?;
            }
        }
    }
    Ok(w)
}

pub fn parse_signed_be<T: PrimInt + Signed>(len: usize) -> impl Fn(u64) -> T {
    move |mut val: u64| {
        let mut accum = T::zero();
        for _ in 0..len {
            accum = accum.unsigned_shl(1);
            accum = accum | T::from::<u8>((val & 1) as u8).unwrap();
            val >>= 1;
        }
        accum = accum.unsigned_shl(64 - len as u32).signed_shr(64 - len as u32); // sign extend
        accum.to_le()
    }
}

pub fn parse_unsigned_be<T: PrimInt + Unsigned>(len: usize) -> impl Fn(u64) -> T {
    move |mut val: u64| {
        let mut accum = T::zero();
        for _ in 0..len {
            accum = accum.unsigned_shl(1);
            accum = accum | T::from::<u8>((val & 1) as u8).unwrap();
            val >>= 1;
        }
        accum.to_le()
    }
}

macro_rules! packet_primitive {
    ($inner:ty, $s:path, $d:path) => {
        impl<'a> $crate::Packet<'a> for $inner {
            fn serialize<W: ::std::io::Write>(
                &self,
                w: ::cookie_factory::WriteContext<W>,
            ) -> ::cookie_factory::GenResult<W> {
                $s(*self)(w)
            }

            fn deserialize(input: &'a [u8]) -> ::nom::IResult<&'a [u8], Self> {
                $d(input)
            }
        }
    };
}
mod primitives {
    use cookie_factory::bytes as ser;
    use nom::number::complete as de;
    packet_primitive!(u8, ser::be_u8, de::be_u8);
    packet_primitive!(i8, ser::be_i8, de::be_i8);
    packet_primitive!(u16, ser::be_u16, de::be_u16);
    packet_primitive!(i16, ser::be_i16, de::be_i16);
    packet_primitive!(u32, ser::be_u32, de::be_u32);
    packet_primitive!(i32, ser::be_i32, de::be_i32);
    packet_primitive!(u64, ser::be_u64, de::be_u64);
    packet_primitive!(i64, ser::be_i64, de::be_i64);
    packet_primitive!(f32, ser::be_f32, de::be_f32);
    packet_primitive!(f64, ser::be_f64, de::be_f64);
}
