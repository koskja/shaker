#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]
#![feature(int_roundings)]
#![feature(arbitrary_enum_discriminant)]
pub mod types;

use std::io::Write;

use cookie_factory::{GenResult, WriteContext};
use nom::IResult;
use protocol_derive::Packet;
use types::{LimitedSlice, VarInt};
mod varint;

pub trait Packet<'t>: Sized {
    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W>;
    fn deserialize(input: &'t [u8]) -> IResult<&'t [u8], Self>;
}

#[derive(Debug, Packet)]
pub struct First {
    pub a: VarInt<u64>,
    pub b: VarInt<i32>,
}

#[derive(Debug, Packet)]
pub struct Second<'a> {
    pub a: VarInt<u32>,
    pub b: LimitedSlice<'a, 2>,
    pub c: LimitedSlice<'a, 3>,
}

#[derive(Debug, Packet)]
#[repr(u8)]
pub enum Login<'this> {
    First(First) = 0xF0,
    Second(Second<'this>) = 0x01,
    Tis { a: Second<'this>, b: First },
}
