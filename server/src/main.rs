#![feature(type_name_of_val)]
use std::any::{type_name, type_name_of_val};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

use libf::types::{LimitedSlice, VarInt};
use libf::Packet;
fn main() -> Result<(), std::io::Error> {
    /*let l = libf::First {
        a: VarInt(69420),
        b: VarInt(-6554423),
    }; */
    let l = libf::Second {
        a: VarInt(568),
        b: LimitedSlice(&[83, 54, 33]),
    };
    println!("{}", type_name_of_val(&l));
    let mut buf = vec![];
    cookie_factory::gen(|x| l.serialize(x), &mut buf).unwrap();
    let res = libf::Second::deserialize(&buf).unwrap();
    println!("{:?}", res);
    /*let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
    let listener = TcpListener::bind(socket)?;
    for con in listener.incoming() {
        let mut _con = con?;
    }*/
    Ok(())
}
