use std::io::Write;

use cookie_factory::{GenResult, WriteContext};
use nom::IResult;
use num_traits::{NumCast, PrimInt};
use protocol_derive::SerializeFn;

use crate::Packet;

// FIXME: Serialize/deserialize everything as signed, even unsigned types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeFn)]
pub struct VarInt<T: PrimInt + From<u8>>(pub T);
impl<'a, T: PrimInt + From<u8>> Packet<'a> for VarInt<T> {
    fn serialize<W: Write>(&self, mut w: WriteContext<W>) -> GenResult<W> {
        let mut val = self.0;
        loop {
            let mut bottom_byte = (val & (<T as From<u8>>::from(0xFFu8))).to_u8().unwrap();
            bottom_byte &= 0x7F;
            val = val.unsigned_shr(7);
            if val.is_zero() {
                w.write(&[bottom_byte])?;
                return Ok(w);
            }
            bottom_byte |= 0x80;
            w.write(&[bottom_byte])?;
        }
    }

    fn deserialize(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let max_len = {
            let max = T::max_value();
            max.count_ones().div_ceil(7)
        } as usize;
        let mut result: T = 0u8.into();
        for (pos, &val) in input.iter().take(max_len).enumerate() {
            let trimmed_byte: T = (val & 0x7F).into();
            result = result | (trimmed_byte.unsigned_shl(pos as u32 * 7));
            if val & 0x80 != 0x80 {
                return IResult::Ok((&input[pos + 1..], Self(result)));
            }
        }
        IResult::Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )))
    }
}
impl<T: PrimInt + From<u8>> VarInt<T> {
    pub fn deserialize_self<'a>(input: &'a [u8]) -> IResult<&'a [u8], T> {
        Self::deserialize(input).map(|(i, this)| (i, this.0))
    }
    pub fn deserialize_prim<'a, U: NumCast>(input: &'a [u8]) -> IResult<&'a [u8], U> {
        Self::deserialize(input).map(|(i, this)| (i, U::from(this.0).unwrap()))
    }
}
#[derive(Debug)]
pub struct LimitedSlice<'a, const MAX: usize>(pub &'a [u8]);
impl<'t: 'a, 'a, const MAX: usize> Packet<'t> for LimitedSlice<'a, MAX> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        assert!(self.0.len() <= MAX);
        cookie_factory::sequence::pair(
            VarInt(self.0.len() as u32),
            cookie_factory::combinator::slice(self.0),
        )(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        nom::combinator::map(
            nom::multi::length_data(nom::combinator::verify(
                VarInt::<u32>::deserialize_prim::<usize>,
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
            VarInt(self.0.len() as u32),
            cookie_factory::combinator::string(self.0),
        )(w)
    }

    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self> {
        nom::combinator::map_res(
            nom::multi::length_data(nom::combinator::verify(
                VarInt::<u32>::deserialize_prim::<usize>,
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
