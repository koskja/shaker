#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]
#![feature(int_roundings)]
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

#[derive(Debug, Packet)]
pub struct Second<'this> {
    pub a: VarInt<u32>,
    pub b: LimitedSlice<'this, 2>,
}
