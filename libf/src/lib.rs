#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]
#![feature(int_roundings)]
#![feature(arbitrary_enum_discriminant)]
pub mod types;

use std::io::Write;

use cookie_factory::{GenResult, WriteContext};
use libf_derive::Packet;
use nom::IResult;
use types::{LimitedSlice, VarInt};

pub trait Packet<'a>: Sized {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W>;
    fn deserialize(input: &'a [u8]) -> IResult<&'a [u8], Self>;
}

#[derive(Debug, Packet)]
pub struct First {
    pub a: VarInt<u64>,
    pub b: VarInt<i32>,
}

#[derive(Debug)]
pub struct Second<'a> {
    pub a: VarInt<u32>,
    //#[borrow('a)]
    pub b: LimitedSlice<'a, 2>,
}
impl<'a, 'this: 'a> Packet<'this> for Second<'a> {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
        let w = <VarInt<u32> as Packet>::serialize(&self.a, w)?;
        let w = <LimitedSlice<'a, 2> as Packet>::serialize(&self.b, w)?;
        Ok(w)
    }
    fn deserialize(input: &'this [u8]) -> IResult<&'this [u8], Self> {
        nom::combinator::map(
            nom::sequence::tuple((
                <VarInt<u32> as Packet>::deserialize,
                <LimitedSlice<'a, 2> as Packet>::deserialize,
            )),
            |(a, b)| Self { a, b },
        )(input)
    }
}

#[derive(Debug, Packet)]
#[repr(u8)]
pub enum Login<'this> {
    First(First) = 0xF0,
    Second(Second<'this>) = 0x01,
    Third(Second<'this>),
}
