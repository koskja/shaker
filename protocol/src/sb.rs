use cookie_factory::SerializeFn;
use nom::{
    combinator::map_opt,
    number::complete::{be_u16, be_u64},
    IResult,
};
use crate::{types::{write_prefixed_slice, write_slice, limited_str, limited_slice}, write_tagged_option};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::{
    enum_options, enum_parser_fn, struct_parse, struct_parser_fn,
    struct_write, struct_writer_fn, 
    types::{
        prefixed_str, remaining_slice, tagged_option, write_str,
        VarInt,
    },
    wf,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Handshake<'a> {
    pub protocol_id: u32,
    pub address: &'a str,
    pub port: u16,
    pub next: HandshakeState,
}
#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Eq)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
}
impl<'a> Handshake<'a> {
    struct_parser_fn! { 'a =>
        protocol_id(VarInt::parse_u32),
        address(prefixed_str),
        port(be_u16),
        next(map_opt(VarInt::parse_i32, HandshakeState::from_i32))
    }
    struct_writer_fn! {
        protocol_id(VarInt::write),
        address(write_str),
        port(cookie_factory::bytes::be_u16),
        next(ref {|x| VarInt::write(ToPrimitive::to_i32(x).unwrap())}),
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Ping {
    pub payload: u64,
}
impl Ping {
    struct_parser_fn! {
        payload(be_u64)
    }
    struct_writer_fn! {
        payload(cookie_factory::bytes::be_u64),
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct LoginStart<'a> {
    pub name: &'a str,
    pub sig_data: Option<LoginStartSigData<'a>>,
}
impl<'a> LoginStart<'a> {
    struct_parser_fn! { 'a =>
        name(limited_str(16)),
        sig_data(tagged_option(LoginStartSigData::parse))
    }
    struct_writer_fn! {
        name(ref cookie_factory::combinator::string),
        sig_data(ref write_tagged_option!(ref LoginStartSigData::write)),
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct LoginStartSigData<'a> {
    pub timestamp: u64,
    pub public_key: &'a [u8],
    pub signature: &'a [u8],
}
impl<'a> LoginStartSigData<'a> {
    struct_parser_fn! { 'a =>
        timestamp(be_u64),
        public_key(limited_slice(0xFFFF)),
        signature(limited_slice(0xFFFF)),
    }
    struct_writer_fn! {
        timestamp(cookie_factory::bytes::be_u64),
        public_key(deref write_prefixed_slice),
        signature(deref write_prefixed_slice),
    }
}

pub enum StatusPacket {
    Request,
    Ping(Ping),
}
impl StatusPacket {
    enum_parser_fn! {
        [0x00 => empty => Self::Request],
        [0x01 => Ping]
    }
    wf! {
        [0x00 => Request => empty],
        [0x01 => Ping]
    }
}
pub struct EncryptionResponse<'a> {
    pub shared_secret: &'a [u8],
    pub verify_token: &'a [u8],
}
impl<'a> EncryptionResponse<'a> {
    struct_parser_fn! { 'a => 
        shared_secret(limited_slice(0xFFFF)),
        verify_token(limited_slice(0xFFFF)),
    }
    struct_writer_fn! {
        shared_secret(deref write_prefixed_slice),
        verify_token(deref write_prefixed_slice),
    }
}
pub struct LoginPluginResponse<'a> {
    pub message_id: u32,
    pub data: Option<&'a [u8]>,
}
impl<'a> LoginPluginResponse<'a> {
    struct_parser_fn! { 'a =>
        message_id(VarInt::parse_u32),
        data(tagged_option(remaining_slice)),
    }
    struct_writer_fn! {
        message_id(cookie_factory::bytes::be_u32),
        data(ref write_tagged_option!(deref write_slice))
    }
}
pub enum LoginPacket<'a> {
    LoginStart(LoginStart<'a>),
    EncryptionResponse(EncryptionResponse<'a>),
    LoginPluginResponse(LoginPluginResponse<'a>),
}
impl<'a> LoginPacket<'a> {
    enum_parser_fn! { 'a =>
        [0x00 => LoginStart],
        [0x01 => EncryptionResponse],
    }
    wf! {
        [0x00 => LoginStart],
        [0x01 => EncryptionResponse],
        [0x02 => LoginPluginResponse],
    }
}
