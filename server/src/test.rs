
use nom::{combinator::map, sequence::tuple};
#[allow(unused_imports)]
use protocol_lib::types::*;
type VarInt = VInt<i32>;
type VarLong = VInt<i64>;
type VarString<'a> = PrefixedString<'a, VarInt>;
type VarStringArray<'a> = PrefixedArray<PrefixedString<'a, VarInt>, VarInt>;
type VarArray<T> = PrefixedArray<T, VarInt>;
type VarBuffer<'a> = PrefixedBuffer<'a, VarInt>;

type Optvarint = VarInt;
pub struct Position {
    x: i32,
    z: i32,
    y: i16,
}

impl<'t> protocol_lib::Packet<'t> for Position {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[
                (unsafe { core::mem::transmute(self.x as i64) }, 26),
                (unsafe { core::mem::transmute(self.z as i64) }, 26),
                (unsafe { core::mem::transmute(self.y as i64) }, 12),
            ],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(tuple((parse_bits_signed(26), parse_bits_signed(26), parse_bits_signed(12))), |(x, z, y)| Position { x, z, y })))(input)
    }
}

pub struct RTrue {
    item_id: VarInt,
    item_count: i8,
    nbt_data: OptionalNbt,
}

impl<'t> protocol_lib::Packet<'t> for RTrue {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.item_id, w)?;
        let w = i8::serialize(&self.item_count, w)?;
        let w = OptionalNbt::serialize(&self.nbt_data, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((VarInt::deserialize, i8::deserialize, OptionalNbt::deserialize)), |(item_id, item_count, nbt_data)| RTrue {
            item_id,
            item_count,
            nbt_data,
        }))(input)
    }
}

pub enum Ident0 {
    RFalse,
    RTrue(RTrue),
    Default,
}

impl Ident0 {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Ident0::RFalse => "false",
            Ident0::RTrue(_) => "true",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            Ident0::RFalse => w,
            Ident0::RTrue(val) => RTrue::serialize(&val, w)?,
            Ident0::Default => w,
        };

        Ok(w)
    }
}
pub struct Slot {
    present: bool,
    ident0: Ident0,
}

impl<'t> protocol_lib::Packet<'t> for Slot {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = bool::serialize(&self.present, w)?;
        let w = Ident0::serialize(&self.ident0, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_present) = (bool::deserialize)(input)?;
            let (input, self_ident0) = (|input| match &format!("{}", self_present)[..] {
                "false" => Ok((input, Ident0::RFalse)),
                "true" => map(RTrue::deserialize, Ident0::RTrue)(input),
                _ => Ok((input, Ident0::Default)),
            })(input)?;
            Ok((
                input,
                Slot {
                    present: self_present,
                    ident0: self_ident0,
                },
            ))
        })(input)
    }
}

pub struct Data2 {
    block_state: VarInt,
}

impl<'t> protocol_lib::Packet<'t> for Data2 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.block_state, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((VarInt::deserialize,)), |(block_state,)| Data2 { block_state }))(input)
    }
}

pub struct Data3 {
    block_state: VarInt,
}

impl<'t> protocol_lib::Packet<'t> for Data3 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.block_state, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((VarInt::deserialize,)), |(block_state,)| Data3 { block_state }))(input)
    }
}

pub struct Data14 {
    red: f32,
    green: f32,
    blue: f32,
    scale: f32,
}

impl<'t> protocol_lib::Packet<'t> for Data14 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = f32::serialize(&self.red, w)?;
        let w = f32::serialize(&self.green, w)?;
        let w = f32::serialize(&self.blue, w)?;
        let w = f32::serialize(&self.scale, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((f32::deserialize, f32::deserialize, f32::deserialize, f32::deserialize)), |(red, green, blue, scale)| Data14 {
            red,
            green,
            blue,
            scale,
        }))(input)
    }
}

pub struct Data15 {
    from_red: f32,
    from_green: f32,
    from_blue: f32,
    scale: f32,
    to_red: f32,
    to_green: f32,
    to_blue: f32,
}

impl<'t> protocol_lib::Packet<'t> for Data15 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = f32::serialize(&self.from_red, w)?;
        let w = f32::serialize(&self.from_green, w)?;
        let w = f32::serialize(&self.from_blue, w)?;
        let w = f32::serialize(&self.scale, w)?;
        let w = f32::serialize(&self.to_red, w)?;
        let w = f32::serialize(&self.to_green, w)?;
        let w = f32::serialize(&self.to_blue, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(
            tuple((
                f32::deserialize,
                f32::deserialize,
                f32::deserialize,
                f32::deserialize,
                f32::deserialize,
                f32::deserialize,
                f32::deserialize,
            )),
            |(from_red, from_green, from_blue, scale, to_red, to_green, to_blue)| Data15 {
                from_red,
                from_green,
                from_blue,
                scale,
                to_red,
                to_green,
                to_blue,
            },
        ))(input)
    }
}

pub struct Data24 {
    block_state: VarInt,
}

impl<'t> protocol_lib::Packet<'t> for Data24 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.block_state, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((VarInt::deserialize,)), |(block_state,)| Data24 { block_state }))(input)
    }
}

pub struct Data35 {
    item: Slot,
}

impl<'t> protocol_lib::Packet<'t> for Data35 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = Slot::serialize(&self.item, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((Slot::deserialize,)), |(item,)| Data35 { item }))(input)
    }
}

pub enum Destination {
    Block(Position),
    Entity(VarInt),
    Default,
}

impl Destination {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Destination::Block(_) => "minecraft:block",
            Destination::Entity(_) => "minecraft:entity",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            Destination::Block(val) => Position::serialize(&val, w)?,
            Destination::Entity(val) => VarInt::serialize(&val, w)?,
            Destination::Default => w,
        };

        Ok(w)
    }
}
pub struct Data36<'a> {
    origin: Position,
    position_type: VarString<'a>,
    destination: Destination,
    ticks: VarInt,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Data36<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = Position::serialize(&self.origin, w)?;
        let w = PrefixedString::<'a, VarInt>::serialize(&self.position_type, w)?;
        let w = Destination::serialize(&self.destination, w)?;
        let w = VarInt::serialize(&self.ticks, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_origin) = (Position::deserialize)(input)?;
            let (input, self_position_type) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
            let (input, self_destination) = (|input| match &format!("{}", self_position_type)[..] {
                "minecraft:block" => map(Position::deserialize, Destination::Block)(input),
                "minecraft:entity" => map(VarInt::deserialize, Destination::Entity)(input),
                _ => Ok((input, Destination::Default)),
            })(input)?;
            let (input, self_ticks) = (VarInt::deserialize)(input)?;
            Ok((
                input,
                Data36 {
                    origin: self_origin,
                    position_type: self_position_type,
                    destination: self_destination,
                    ticks: self_ticks,
                },
            ))
        })(input)
    }
}

pub enum Data<'a> {
    Data2(Data2),
    Data3(Data3),
    Data14(Data14),
    Data15(Data15),
    Data24(Data24),
    Data35(Data35),
    Data36(Data36<'a>),
    Default,
}

impl<'a> Data<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Data::Data2(_) => "2",
            Data::Data3(_) => "3",
            Data::Data14(_) => "14",
            Data::Data15(_) => "15",
            Data::Data24(_) => "24",
            Data::Data35(_) => "35",
            Data::Data36(_) => "36",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            Data::Data2(val) => Data2::serialize(&val, w)?,
            Data::Data3(val) => Data3::serialize(&val, w)?,
            Data::Data14(val) => Data14::serialize(&val, w)?,
            Data::Data15(val) => Data15::serialize(&val, w)?,
            Data::Data24(val) => Data24::serialize(&val, w)?,
            Data::Data35(val) => Data35::serialize(&val, w)?,
            Data::Data36(val) => Data36::serialize(&val, w)?,
            Data::Default => w,
        };

        Ok(w)
    }
}
pub struct Particle<'a> {
    particle_id: VarInt,
    data: Data<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Particle<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.particle_id, w)?;
        let w = Data::serialize(&self.data, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_particle_id) = (VarInt::deserialize)(input)?;
            let (input, self_data) = (|input| match &format!("{}", self_particle_id)[..] {
                "2" => map(Data2::deserialize, Data::Data2)(input),
                "3" => map(Data3::deserialize, Data::Data3)(input),
                "14" => map(Data14::deserialize, Data::Data14)(input),
                "15" => map(Data15::deserialize, Data::Data15)(input),
                "24" => map(Data24::deserialize, Data::Data24)(input),
                "35" => map(Data35::deserialize, Data::Data35)(input),
                "36" => map(Data36::deserialize, Data::Data36)(input),
                _ => Ok((input, Data::Default)),
            })(input)?;
            Ok((
                input,
                Particle {
                    particle_id: self_particle_id,
                    data: self_data,
                },
            ))
        })(input)
    }
}

pub struct Ident1 {
    key: u8,
    r_type: VarInt,
}

impl<'t> protocol_lib::Packet<'t> for Ident1 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = u8::serialize(&self.key, w)?;
        let w = VarInt::serialize(&self.r_type, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((u8::deserialize, VarInt::deserialize)), |(key, r_type)| Ident1 { key, r_type }))(input)
    }
}

pub struct Value8 {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl<'t> protocol_lib::Packet<'t> for Value8 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = f32::serialize(&self.pitch, w)?;
        let w = f32::serialize(&self.yaw, w)?;
        let w = f32::serialize(&self.roll, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((f32::deserialize, f32::deserialize, f32::deserialize)), |(pitch, yaw, roll)| Value8 { pitch, yaw, roll }))(input)
    }
}

pub struct Value16 {
    villager_type: VarInt,
    villager_profession: VarInt,
    level: VarInt,
}

impl<'t> protocol_lib::Packet<'t> for Value16 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.villager_type, w)?;
        let w = VarInt::serialize(&self.villager_profession, w)?;
        let w = VarInt::serialize(&self.level, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((VarInt::deserialize, VarInt::deserialize, VarInt::deserialize)), |(villager_type, villager_profession, level)| {
            Value16 {
                villager_type,
                villager_profession,
                level,
            }
        }))(input)
    }
}

pub enum EntityMetadata<'a> {
    Value0(i8),
    Value1(VarInt),
    Value2(f32),
    Value3(VarString<'a>),
    Value4(VarString<'a>),
    Value5(Option<VarString<'a>>),
    Value6(Slot),
    Value7(bool),
    Value8(Value8),
    Value9(Position),
    Value10(Option<Position>),
    Value11(VarInt),
    Value12(Option<Uuid>),
    Value13(VarInt),
    Value14(Nbt),
    Value15(Particle<'a>),
    Value16(Value16),
    Value17(Optvarint),
    Value18(VarInt),
    Default,
}

impl<'a> EntityMetadata<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            EntityMetadata::Value0(_) => "0",
            EntityMetadata::Value1(_) => "1",
            EntityMetadata::Value2(_) => "2",
            EntityMetadata::Value3(_) => "3",
            EntityMetadata::Value4(_) => "4",
            EntityMetadata::Value5(_) => "5",
            EntityMetadata::Value6(_) => "6",
            EntityMetadata::Value7(_) => "7",
            EntityMetadata::Value8(_) => "8",
            EntityMetadata::Value9(_) => "9",
            EntityMetadata::Value10(_) => "10",
            EntityMetadata::Value11(_) => "11",
            EntityMetadata::Value12(_) => "12",
            EntityMetadata::Value13(_) => "13",
            EntityMetadata::Value14(_) => "14",
            EntityMetadata::Value15(_) => "15",
            EntityMetadata::Value16(_) => "16",
            EntityMetadata::Value17(_) => "17",
            EntityMetadata::Value18(_) => "18",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            EntityMetadata::Value0(val) => i8::serialize(&val, w)?,
            EntityMetadata::Value1(val) => VarInt::serialize(&val, w)?,
            EntityMetadata::Value2(val) => f32::serialize(&val, w)?,
            EntityMetadata::Value3(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
            EntityMetadata::Value4(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
            EntityMetadata::Value5(val) => Option::<VarString<'a>>::serialize(&val, w)?,
            EntityMetadata::Value6(val) => Slot::serialize(&val, w)?,
            EntityMetadata::Value7(val) => bool::serialize(&val, w)?,
            EntityMetadata::Value8(val) => Value8::serialize(&val, w)?,
            EntityMetadata::Value9(val) => Position::serialize(&val, w)?,
            EntityMetadata::Value10(val) => Option::<Position>::serialize(&val, w)?,
            EntityMetadata::Value11(val) => VarInt::serialize(&val, w)?,
            EntityMetadata::Value12(val) => Option::<Uuid>::serialize(&val, w)?,
            EntityMetadata::Value13(val) => VarInt::serialize(&val, w)?,
            EntityMetadata::Value14(val) => Nbt::serialize(&val, w)?,
            EntityMetadata::Value15(val) => Particle::serialize(&val, w)?,
            EntityMetadata::Value16(val) => Value16::serialize(&val, w)?,
            EntityMetadata::Value17(val) => VarInt::serialize(&val, w)?,
            EntityMetadata::Value18(val) => VarInt::serialize(&val, w)?,
            EntityMetadata::Default => w,
        };

        Ok(w)
    }
}
pub struct EntityMetadataWrapper<'a> {
    key: u8,
    r_type: VarInt,
    value: EntityMetadata<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for EntityMetadataWrapper<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = u8::serialize(&self.key, w)?;
        let w = VarInt::serialize(&self.r_type, w)?;
        let w = EntityMetadata::serialize(&self.value, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_key) = (u8::deserialize)(input)?;
            let (input, self_r_type) = (VarInt::deserialize)(input)?;
            let (input, self_value) = (|input| match &format!("{}", self_r_type)[..] {
                "0" => map(i8::deserialize, EntityMetadata::Value0)(input),
                "1" => map(VarInt::deserialize, EntityMetadata::Value1)(input),
                "2" => map(f32::deserialize, EntityMetadata::Value2)(input),
                "3" => map(PrefixedString::<'a, VarInt>::deserialize, EntityMetadata::Value3)(input),
                "4" => map(PrefixedString::<'a, VarInt>::deserialize, EntityMetadata::Value4)(input),
                "5" => map(Option::<VarString<'a>>::deserialize, EntityMetadata::Value5)(input),
                "6" => map(Slot::deserialize, EntityMetadata::Value6)(input),
                "7" => map(bool::deserialize, EntityMetadata::Value7)(input),
                "8" => map(Value8::deserialize, EntityMetadata::Value8)(input),
                "9" => map(Position::deserialize, EntityMetadata::Value9)(input),
                "10" => map(Option::<Position>::deserialize, EntityMetadata::Value10)(input),
                "11" => map(VarInt::deserialize, EntityMetadata::Value11)(input),
                "12" => map(Option::<Uuid>::deserialize, EntityMetadata::Value12)(input),
                "13" => map(VarInt::deserialize, EntityMetadata::Value13)(input),
                "14" => map(Nbt::deserialize, EntityMetadata::Value14)(input),
                "15" => map(Particle::deserialize, EntityMetadata::Value15)(input),
                "16" => map(Value16::deserialize, EntityMetadata::Value16)(input),
                "17" => map(VarInt::deserialize, EntityMetadata::Value17)(input),
                "18" => map(VarInt::deserialize, EntityMetadata::Value18)(input),
                _ => Ok((input, EntityMetadata::Default)),
            })(input)?;
            Ok((
                input,
                EntityMetadataWrapper {
                    key: self_key,
                    r_type: self_r_type,
                    value: self_value,
                },
            ))
        })(input)
    }
}

pub struct MinecraftSmeltingFormat<'a> {
    group: VarString<'a>,
    ingredient: VarArray<Slot>,
    result: Slot,
    experience: f32,
    cook_time: VarInt,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for MinecraftSmeltingFormat<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;

        let w = PrefixedArray::<Slot, VarInt>::len(&self.ingredient).serialize(w)?;

        let mut w = w;
        let items = self.ingredient.0.iter();
        for i in items {
            w = Slot::serialize(&i, w)?
        }

        let w = Slot::serialize(&self.result, w)?;
        let w = f32::serialize(&self.experience, w)?;
        let w = VarInt::serialize(&self.cook_time, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(
            tuple((
                PrefixedString::<'a, VarInt>::deserialize,
                PrefixedArray::<Slot, VarInt>::deserialize,
                Slot::deserialize,
                f32::deserialize,
                VarInt::deserialize,
            )),
            |(group, ingredient, result, experience, cook_time)| MinecraftSmeltingFormat {
                group,
                ingredient,
                result,
                experience,
                cook_time,
            },
        ))(input)
    }
}

pub struct Tag<'a> {
    tag_name: VarString<'a>,
    entries: VarArray<VarInt>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Tag<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.tag_name, w)?;

        let w = PrefixedArray::<VarInt, VarInt>::len(&self.entries).serialize(w)?;

        let mut w = w;
        let items = self.entries.0.iter();
        for i in items {
            w = VarInt::serialize(&i, w)?
        }

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(
            tuple((PrefixedString::<'a, VarInt>::deserialize, PrefixedArray::<VarInt, VarInt>::deserialize)),
            |(tag_name, entries)| Tag { tag_name, entries },
        ))(input)
    }
}

pub struct Ident5 {
    x: u8,
    z: u8,
}

impl<'t> protocol_lib::Packet<'t> for Ident5 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.x as u64, 4), (self.z as u64, 4)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(tuple((parse_bits_unsigned(4), parse_bits_unsigned(4))), |(x, z)| Ident5 { x, z })))(input)
    }
}

pub struct ChunkBlockEntity {
    ident5: Ident5,
    y: i16,
    r_type: VarInt,
    nbt_data: OptionalNbt,
}

impl<'t> protocol_lib::Packet<'t> for ChunkBlockEntity {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = Ident5::serialize(&self.ident5, w)?;
        let w = i16::serialize(&self.y, w)?;
        let w = VarInt::serialize(&self.r_type, w)?;
        let w = OptionalNbt::serialize(&self.nbt_data, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(
            tuple((Ident5::deserialize, i16::deserialize, VarInt::deserialize, OptionalNbt::deserialize)),
            |(ident5, y, r_type, nbt_data)| ChunkBlockEntity { ident5, y, r_type, nbt_data },
        ))(input)
    }
}

pub struct Flags {
    unused: u8,
    has_custom_suggestions: u8,
    has_redirect_node: u8,
    has_command: u8,
    command_node_type: u8,
}

impl<'t> protocol_lib::Packet<'t> for Flags {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[
                (self.unused as u64, 3),
                (self.has_custom_suggestions as u64, 1),
                (self.has_redirect_node as u64, 1),
                (self.has_command as u64, 1),
                (self.command_node_type as u64, 2),
            ],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(
            tuple((parse_bits_unsigned(3), parse_bits_unsigned(1), parse_bits_unsigned(1), parse_bits_unsigned(1), parse_bits_unsigned(2))),
            |(unused, has_custom_suggestions, has_redirect_node, has_command, command_node_type)| Flags {
                unused,
                has_custom_suggestions,
                has_redirect_node,
                has_command,
                command_node_type,
            },
        )))(input)
    }
}

pub enum RedirectNode {
    RedirectNode1(VarInt),
    Default,
}

impl RedirectNode {
    pub fn discriminant(&self) -> &'static str {
        match self {
            RedirectNode::RedirectNode1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            RedirectNode::RedirectNode1(val) => VarInt::serialize(&val, w)?,
            RedirectNode::Default => w,
        };

        Ok(w)
    }
}
pub struct ExtraNodeData1<'a> {
    name: VarString<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for ExtraNodeData1<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(name,)| ExtraNodeData1 { name }))(input)
    }
}

pub struct FloatFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

impl<'t> protocol_lib::Packet<'t> for FloatFlags {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.unused as u64, 6), (self.max_present as u64, 1), (self.min_present as u64, 1)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(
            tuple((parse_bits_unsigned(6), parse_bits_unsigned(1), parse_bits_unsigned(1))),
            |(unused, max_present, min_present)| FloatFlags { unused, max_present, min_present },
        )))(input)
    }
}

pub enum Min {
    Min1(f32),
    Default,
}

impl Min {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Min::Min1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            Min::Min1(val) => f32::serialize(&val, w)?,
            Min::Default => w,
        };

        Ok(w)
    }
}
pub enum Max {
    Max1(f32),
    Default,
}

impl Max {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Max::Max1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            Max::Max1(val) => f32::serialize(&val, w)?,
            Max::Default => w,
        };

        Ok(w)
    }
}
pub struct Float {
    flags: FloatFlags,
    min: Min,
    max: Max,
}

impl<'t> protocol_lib::Packet<'t> for Float {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = FloatFlags::serialize(&self.flags, w)?;
        let w = Min::serialize(&self.min, w)?;
        let w = Max::serialize(&self.max, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (FloatFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => map(f32::deserialize, Min::Min1)(input),
                _ => Ok((input, Min::Default)),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => map(f32::deserialize, Max::Max1)(input),
                _ => Ok((input, Max::Default)),
            })(input)?;
            Ok((
                input,
                Float {
                    flags: self_flags,
                    min: self_min,
                    max: self_max,
                },
            ))
        })(input)
    }
}

pub struct DoubleFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

impl<'t> protocol_lib::Packet<'t> for DoubleFlags {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.unused as u64, 6), (self.max_present as u64, 1), (self.min_present as u64, 1)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(
            tuple((parse_bits_unsigned(6), parse_bits_unsigned(1), parse_bits_unsigned(1))),
            |(unused, max_present, min_present)| DoubleFlags { unused, max_present, min_present },
        )))(input)
    }
}

pub enum DoubleMin {
    DoubleMin1(f64),
    Default,
}

impl DoubleMin {
    pub fn discriminant(&self) -> &'static str {
        match self {
            DoubleMin::DoubleMin1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            DoubleMin::DoubleMin1(val) => f64::serialize(&val, w)?,
            DoubleMin::Default => w,
        };

        Ok(w)
    }
}
pub enum DoubleMax {
    DoubleMax1(f64),
    Default,
}

impl DoubleMax {
    pub fn discriminant(&self) -> &'static str {
        match self {
            DoubleMax::DoubleMax1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            DoubleMax::DoubleMax1(val) => f64::serialize(&val, w)?,
            DoubleMax::Default => w,
        };

        Ok(w)
    }
}
pub struct Double {
    flags: DoubleFlags,
    min: DoubleMin,
    max: DoubleMax,
}

impl<'t> protocol_lib::Packet<'t> for Double {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = DoubleFlags::serialize(&self.flags, w)?;
        let w = DoubleMin::serialize(&self.min, w)?;
        let w = DoubleMax::serialize(&self.max, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (DoubleFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => map(f64::deserialize, DoubleMin::DoubleMin1)(input),
                _ => Ok((input, DoubleMin::Default)),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => map(f64::deserialize, DoubleMax::DoubleMax1)(input),
                _ => Ok((input, DoubleMax::Default)),
            })(input)?;
            Ok((
                input,
                Double {
                    flags: self_flags,
                    min: self_min,
                    max: self_max,
                },
            ))
        })(input)
    }
}

pub struct IntegerFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

impl<'t> protocol_lib::Packet<'t> for IntegerFlags {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.unused as u64, 6), (self.max_present as u64, 1), (self.min_present as u64, 1)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(
            tuple((parse_bits_unsigned(6), parse_bits_unsigned(1), parse_bits_unsigned(1))),
            |(unused, max_present, min_present)| IntegerFlags { unused, max_present, min_present },
        )))(input)
    }
}

pub enum IntegerMin {
    IntegerMin1(i32),
    Default,
}

impl IntegerMin {
    pub fn discriminant(&self) -> &'static str {
        match self {
            IntegerMin::IntegerMin1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            IntegerMin::IntegerMin1(val) => i32::serialize(&val, w)?,
            IntegerMin::Default => w,
        };

        Ok(w)
    }
}
pub enum IntegerMax {
    IntegerMax1(i32),
    Default,
}

impl IntegerMax {
    pub fn discriminant(&self) -> &'static str {
        match self {
            IntegerMax::IntegerMax1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            IntegerMax::IntegerMax1(val) => i32::serialize(&val, w)?,
            IntegerMax::Default => w,
        };

        Ok(w)
    }
}
pub struct Integer {
    flags: IntegerFlags,
    min: IntegerMin,
    max: IntegerMax,
}

impl<'t> protocol_lib::Packet<'t> for Integer {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = IntegerFlags::serialize(&self.flags, w)?;
        let w = IntegerMin::serialize(&self.min, w)?;
        let w = IntegerMax::serialize(&self.max, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (IntegerFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => map(i32::deserialize, IntegerMin::IntegerMin1)(input),
                _ => Ok((input, IntegerMin::Default)),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => map(i32::deserialize, IntegerMax::IntegerMax1)(input),
                _ => Ok((input, IntegerMax::Default)),
            })(input)?;
            Ok((
                input,
                Integer {
                    flags: self_flags,
                    min: self_min,
                    max: self_max,
                },
            ))
        })(input)
    }
}

pub struct LongFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

impl<'t> protocol_lib::Packet<'t> for LongFlags {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.unused as u64, 6), (self.max_present as u64, 1), (self.min_present as u64, 1)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(
            tuple((parse_bits_unsigned(6), parse_bits_unsigned(1), parse_bits_unsigned(1))),
            |(unused, max_present, min_present)| LongFlags { unused, max_present, min_present },
        )))(input)
    }
}

pub enum LongMin {
    LongMin1(i64),
    Default,
}

impl LongMin {
    pub fn discriminant(&self) -> &'static str {
        match self {
            LongMin::LongMin1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            LongMin::LongMin1(val) => i64::serialize(&val, w)?,
            LongMin::Default => w,
        };

        Ok(w)
    }
}
pub enum LongMax {
    LongMax1(i64),
    Default,
}

impl LongMax {
    pub fn discriminant(&self) -> &'static str {
        match self {
            LongMax::LongMax1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            LongMax::LongMax1(val) => i64::serialize(&val, w)?,
            LongMax::Default => w,
        };

        Ok(w)
    }
}
pub struct Long {
    flags: LongFlags,
    min: LongMin,
    max: LongMax,
}

impl<'t> protocol_lib::Packet<'t> for Long {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = LongFlags::serialize(&self.flags, w)?;
        let w = LongMin::serialize(&self.min, w)?;
        let w = LongMax::serialize(&self.max, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (LongFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => map(i64::deserialize, LongMin::LongMin1)(input),
                _ => Ok((input, LongMin::Default)),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => map(i64::deserialize, LongMax::LongMax1)(input),
                _ => Ok((input, LongMax::Default)),
            })(input)?;
            Ok((
                input,
                Long {
                    flags: self_flags,
                    min: self_min,
                    max: self_max,
                },
            ))
        })(input)
    }
}

pub struct MinecraftEntity {
    unused: u8,
    only_allow_players: u8,
    only_allow_entities: u8,
}

impl<'t> protocol_lib::Packet<'t> for MinecraftEntity {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.unused as u64, 6), (self.only_allow_players as u64, 1), (self.only_allow_entities as u64, 1)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(
            tuple((parse_bits_unsigned(6), parse_bits_unsigned(1), parse_bits_unsigned(1))),
            |(unused, only_allow_players, only_allow_entities)| MinecraftEntity {
                unused,
                only_allow_players,
                only_allow_entities,
            },
        )))(input)
    }
}

pub struct ScoreHolder {
    unused: u8,
    allow_multiple: u8,
}

impl<'t> protocol_lib::Packet<'t> for ScoreHolder {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.unused as u64, 7), (self.allow_multiple as u64, 1)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(tuple((parse_bits_unsigned(7), parse_bits_unsigned(1))), |(unused, allow_multiple)| ScoreHolder {
            unused,
            allow_multiple,
        })))(input)
    }
}

pub struct Range {
    allow_decimals: bool,
}

impl<'t> protocol_lib::Packet<'t> for Range {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = bool::serialize(&self.allow_decimals, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((bool::deserialize,)), |(allow_decimals,)| Range { allow_decimals }))(input)
    }
}

pub struct ResourceOrTag<'a> {
    registry: VarString<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for ResourceOrTag<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.registry, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(registry,)| ResourceOrTag { registry }))(input)
    }
}

pub struct Resource<'a> {
    registry: VarString<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Resource<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.registry, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(registry,)| Resource { registry }))(input)
    }
}

pub enum Properties<'a> {
    Bool,
    Float(Float),
    Double(Double),
    Integer(Integer),
    Long(Long),
    String(&'static str),
    MinecraftEntity(MinecraftEntity),
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    Component,
    Message,
    Nbt,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Angle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder(ScoreHolder),
    Swizzle,
    Team,
    ItemSlot,
    ResourceLocation,
    MobEffect,
    Function,
    EntityAnchor,
    Range(Range),
    IntRange,
    FloatRange,
    ItemEnchantment,
    EntitySummon,
    Dimension,
    NbtCompoundTag,
    Time,
    ResourceOrTag(ResourceOrTag<'a>),
    Resource(Resource<'a>),
    Uuid,
    Default,
}

impl<'a> Properties<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Properties::Bool => "brigadier:bool",
            Properties::Float(_) => "brigadier:float",
            Properties::Double(_) => "brigadier:double",
            Properties::Integer(_) => "brigadier:integer",
            Properties::Long(_) => "brigadier:long",
            Properties::String(_) => "brigadier:string",
            Properties::MinecraftEntity(_) => "minecraft:entity",
            Properties::GameProfile => "minecraft:game_profile",
            Properties::BlockPos => "minecraft:block_pos",
            Properties::ColumnPos => "minecraft:column_pos",
            Properties::Vec3 => "minecraft:vec3",
            Properties::Vec2 => "minecraft:vec2",
            Properties::BlockState => "minecraft:block_state",
            Properties::BlockPredicate => "minecraft:block_predicate",
            Properties::ItemStack => "minecraft:item_stack",
            Properties::ItemPredicate => "minecraft:item_predicate",
            Properties::Color => "minecraft:color",
            Properties::Component => "minecraft:component",
            Properties::Message => "minecraft:message",
            Properties::Nbt => "minecraft:nbt",
            Properties::NbtPath => "minecraft:nbt_path",
            Properties::Objective => "minecraft:objective",
            Properties::ObjectiveCriteria => "minecraft:objective_criteria",
            Properties::Operation => "minecraft:operation",
            Properties::Particle => "minecraft:particle",
            Properties::Angle => "minecraft:angle",
            Properties::Rotation => "minecraft:rotation",
            Properties::ScoreboardSlot => "minecraft:scoreboard_slot",
            Properties::ScoreHolder(_) => "minecraft:score_holder",
            Properties::Swizzle => "minecraft:swizzle",
            Properties::Team => "minecraft:team",
            Properties::ItemSlot => "minecraft:item_slot",
            Properties::ResourceLocation => "minecraft:resource_location",
            Properties::MobEffect => "minecraft:mob_effect",
            Properties::Function => "minecraft:function",
            Properties::EntityAnchor => "minecraft:entity_anchor",
            Properties::Range(_) => "minecraft:range",
            Properties::IntRange => "minecraft:int_range",
            Properties::FloatRange => "minecraft:float_range",
            Properties::ItemEnchantment => "minecraft:item_enchantment",
            Properties::EntitySummon => "minecraft:entity_summon",
            Properties::Dimension => "minecraft:dimension",
            Properties::NbtCompoundTag => "minecraft:nbt_compound_tag",
            Properties::Time => "minecraft:time",
            Properties::ResourceOrTag(_) => "minecraft:resource_or_tag",
            Properties::Resource(_) => "minecraft:resource",
            Properties::Uuid => "minecraft:uuid",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            Properties::Bool => w,
            Properties::Float(val) => Float::serialize(&val, w)?,
            Properties::Double(val) => Double::serialize(&val, w)?,
            Properties::Integer(val) => Integer::serialize(&val, w)?,
            Properties::Long(val) => Long::serialize(&val, w)?,
            Properties::String(val) => {
                let tag = match &val[..] {
                    "SINGLE_WORD" => "0",
                    "QUOTABLE_PHRASE" => "1",
                    "GREEDY_PHRASE" => "2",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;
                w
            }
            Properties::MinecraftEntity(val) => MinecraftEntity::serialize(&val, w)?,
            Properties::GameProfile => w,
            Properties::BlockPos => w,
            Properties::ColumnPos => w,
            Properties::Vec3 => w,
            Properties::Vec2 => w,
            Properties::BlockState => w,
            Properties::BlockPredicate => w,
            Properties::ItemStack => w,
            Properties::ItemPredicate => w,
            Properties::Color => w,
            Properties::Component => w,
            Properties::Message => w,
            Properties::Nbt => w,
            Properties::NbtPath => w,
            Properties::Objective => w,
            Properties::ObjectiveCriteria => w,
            Properties::Operation => w,
            Properties::Particle => w,
            Properties::Angle => w,
            Properties::Rotation => w,
            Properties::ScoreboardSlot => w,
            Properties::ScoreHolder(val) => ScoreHolder::serialize(&val, w)?,
            Properties::Swizzle => w,
            Properties::Team => w,
            Properties::ItemSlot => w,
            Properties::ResourceLocation => w,
            Properties::MobEffect => w,
            Properties::Function => w,
            Properties::EntityAnchor => w,
            Properties::Range(val) => Range::serialize(&val, w)?,
            Properties::IntRange => w,
            Properties::FloatRange => w,
            Properties::ItemEnchantment => w,
            Properties::EntitySummon => w,
            Properties::Dimension => w,
            Properties::NbtCompoundTag => w,
            Properties::Time => w,
            Properties::ResourceOrTag(val) => ResourceOrTag::serialize(&val, w)?,
            Properties::Resource(val) => Resource::serialize(&val, w)?,
            Properties::Uuid => w,
            Properties::Default => w,
        };

        Ok(w)
    }
}
pub enum SuggestionType<'a> {
    SuggestionType1(VarString<'a>),
    Default,
}

impl<'a> SuggestionType<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            SuggestionType::SuggestionType1(_) => "1",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            SuggestionType::SuggestionType1(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
            SuggestionType::Default => w,
        };

        Ok(w)
    }
}
pub struct ExtraNodeData2<'a> {
    name: VarString<'a>,
    parser: VarString<'a>,
    properties: Properties<'a>,
    suggestion_type: SuggestionType<'a>,
}
pub enum ExtraNodeData<'a> {
    ExtraNodeData0,
    ExtraNodeData1(ExtraNodeData1<'a>),
    ExtraNodeData2(ExtraNodeData2<'a>),
    Default,
}

impl<'a> ExtraNodeData<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            ExtraNodeData::ExtraNodeData0 => "0",
            ExtraNodeData::ExtraNodeData1(_) => "1",
            ExtraNodeData::ExtraNodeData2(_) => "2",
            _ => "",
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        use protocol_lib::Packet;

        let w = match &self {
            ExtraNodeData::ExtraNodeData0 => w,
            ExtraNodeData::ExtraNodeData1(val) => ExtraNodeData1::serialize(&val, w)?,
            ExtraNodeData::ExtraNodeData2(val) => {
                let w = PrefixedString::<'a, VarInt>::serialize(&val.name, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&val.parser, w)?;
                let w = Properties::serialize(&val.properties, w)?;
                let w = SuggestionType::serialize(&val.suggestion_type, w)?;
                w
            }
            ExtraNodeData::Default => w,
        };

        Ok(w)
    }
}
pub struct CommandNode<'a> {
    flags: Flags,
    children: VarArray<VarInt>,
    redirect_node: RedirectNode,
    extra_node_data: ExtraNodeData<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for CommandNode<'a> {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = Flags::serialize(&self.flags, w)?;

        let w = PrefixedArray::<VarInt, VarInt>::len(&self.children).serialize(w)?;

        let mut w = w;
        let items = self.children.0.iter();
        for i in items {
            w = VarInt::serialize(&i, w)?
        }

        let w = RedirectNode::serialize(&self.redirect_node, w)?;
        let w = ExtraNodeData::serialize(&self.extra_node_data, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (Flags::deserialize)(input)?;
            let (input, self_children) = (PrefixedArray::<VarInt, VarInt>::deserialize)(input)?;
            let (input, self_redirect_node) = (|input| match &format!("{}", self_flags.has_redirect_node)[..] {
                "1" => map(VarInt::deserialize, RedirectNode::RedirectNode1)(input),
                _ => Ok((input, RedirectNode::Default)),
            })(input)?;
            let (input, self_extra_node_data) = (|input| match &format!("{}", self_flags.command_node_type)[..] {
                "0" => Ok((input, ExtraNodeData::ExtraNodeData0)),
                "1" => map(ExtraNodeData1::deserialize, ExtraNodeData::ExtraNodeData1)(input),
                "2" => map(
                    |input| {
                        let (input, self_extra_node_data_name) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                        let (input, self_extra_node_data_parser) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                        let (input, self_extra_node_data_properties) = (|input| match &format!("{}", self_extra_node_data_parser)[..] {
                            "brigadier:bool" => Ok((input, Properties::Bool)),
                            "brigadier:float" => map(Float::deserialize, Properties::Float)(input),
                            "brigadier:double" => map(Double::deserialize, Properties::Double)(input),
                            "brigadier:integer" => map(Integer::deserialize, Properties::Integer)(input),
                            "brigadier:long" => map(Long::deserialize, Properties::Long)(input),
                            "brigadier:string" => map(
                                |input| {
                                    let (input, x) = (VarInt::deserialize)(input)?;
                                    let x = format!("{x}");
                                    let val = match &x[..] {
                                        "0" => "SINGLE_WORD",
                                        "1" => "QUOTABLE_PHRASE",
                                        "2" => "GREEDY_PHRASE",

                                        _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                                    };
                                    Ok((input, val))
                                },
                                Properties::String,
                            )(input),
                            "minecraft:entity" => map(MinecraftEntity::deserialize, Properties::MinecraftEntity)(input),
                            "minecraft:game_profile" => Ok((input, Properties::GameProfile)),
                            "minecraft:block_pos" => Ok((input, Properties::BlockPos)),
                            "minecraft:column_pos" => Ok((input, Properties::ColumnPos)),
                            "minecraft:vec3" => Ok((input, Properties::Vec3)),
                            "minecraft:vec2" => Ok((input, Properties::Vec2)),
                            "minecraft:block_state" => Ok((input, Properties::BlockState)),
                            "minecraft:block_predicate" => Ok((input, Properties::BlockPredicate)),
                            "minecraft:item_stack" => Ok((input, Properties::ItemStack)),
                            "minecraft:item_predicate" => Ok((input, Properties::ItemPredicate)),
                            "minecraft:color" => Ok((input, Properties::Color)),
                            "minecraft:component" => Ok((input, Properties::Component)),
                            "minecraft:message" => Ok((input, Properties::Message)),
                            "minecraft:nbt" => Ok((input, Properties::Nbt)),
                            "minecraft:nbt_path" => Ok((input, Properties::NbtPath)),
                            "minecraft:objective" => Ok((input, Properties::Objective)),
                            "minecraft:objective_criteria" => Ok((input, Properties::ObjectiveCriteria)),
                            "minecraft:operation" => Ok((input, Properties::Operation)),
                            "minecraft:particle" => Ok((input, Properties::Particle)),
                            "minecraft:angle" => Ok((input, Properties::Angle)),
                            "minecraft:rotation" => Ok((input, Properties::Rotation)),
                            "minecraft:scoreboard_slot" => Ok((input, Properties::ScoreboardSlot)),
                            "minecraft:score_holder" => map(ScoreHolder::deserialize, Properties::ScoreHolder)(input),
                            "minecraft:swizzle" => Ok((input, Properties::Swizzle)),
                            "minecraft:team" => Ok((input, Properties::Team)),
                            "minecraft:item_slot" => Ok((input, Properties::ItemSlot)),
                            "minecraft:resource_location" => Ok((input, Properties::ResourceLocation)),
                            "minecraft:mob_effect" => Ok((input, Properties::MobEffect)),
                            "minecraft:function" => Ok((input, Properties::Function)),
                            "minecraft:entity_anchor" => Ok((input, Properties::EntityAnchor)),
                            "minecraft:range" => map(Range::deserialize, Properties::Range)(input),
                            "minecraft:int_range" => Ok((input, Properties::IntRange)),
                            "minecraft:float_range" => Ok((input, Properties::FloatRange)),
                            "minecraft:item_enchantment" => Ok((input, Properties::ItemEnchantment)),
                            "minecraft:entity_summon" => Ok((input, Properties::EntitySummon)),
                            "minecraft:dimension" => Ok((input, Properties::Dimension)),
                            "minecraft:nbt_compound_tag" => Ok((input, Properties::NbtCompoundTag)),
                            "minecraft:time" => Ok((input, Properties::Time)),
                            "minecraft:resource_or_tag" => map(ResourceOrTag::deserialize, Properties::ResourceOrTag)(input),
                            "minecraft:resource" => map(Resource::deserialize, Properties::Resource)(input),
                            "minecraft:uuid" => Ok((input, Properties::Uuid)),
                            _ => Ok((input, Properties::Default)),
                        })(input)?;
                        let (input, self_extra_node_data_suggestion_type) = (|input| match &format!("{}", self_flags.has_custom_suggestions)[..] {
                            "1" => map(PrefixedString::<'a, VarInt>::deserialize, SuggestionType::SuggestionType1)(input),
                            _ => Ok((input, SuggestionType::Default)),
                        })(input)?;
                        Ok((
                            input,
                            ExtraNodeData2 {
                                name: self_extra_node_data_name,
                                parser: self_extra_node_data_parser,
                                properties: self_extra_node_data_properties,
                                suggestion_type: self_extra_node_data_suggestion_type,
                            },
                        ))
                    },
                    ExtraNodeData::ExtraNodeData2,
                )(input),
                _ => Ok((input, ExtraNodeData::Default)),
            })(input)?;
            Ok((
                input,
                CommandNode {
                    flags: self_flags,
                    children: self_children,
                    redirect_node: self_redirect_node,
                    extra_node_data: self_extra_node_data,
                },
            ))
        })(input)
    }
}

pub mod handshaking {
    pub mod clientbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub enum Params {
            Default,
        }

        impl Params {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet {
            name: &'static str,
            params: Params,
        }

        impl<'t> protocol_lib::Packet<'t> for Packet {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|x| Ok((x, "")))(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub struct PacketSetProtocol<'a> {
            protocol_version: VarInt,
            server_host: VarString<'a>,
            server_port: u16,
            next_state: VarInt,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSetProtocol<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.protocol_version, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.server_host, w)?;
                let w = u16::serialize(&self.server_port, w)?;
                let w = VarInt::serialize(&self.next_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, PrefixedString::<'a, VarInt>::deserialize, u16::deserialize, VarInt::deserialize)),
                    |(protocol_version, server_host, server_port, next_state)| PacketSetProtocol {
                        protocol_version,
                        server_host,
                        server_port,
                        next_state,
                    },
                ))(input)
            }
        }

        pub struct PacketLegacyServerListPing {
            payload: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketLegacyServerListPing {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.payload, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize,)), |(payload,)| PacketLegacyServerListPing { payload }))(input)
            }
        }

        pub enum Params<'a> {
            SetProtocol(PacketSetProtocol<'a>),
            LegacyServerListPing(PacketLegacyServerListPing),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::SetProtocol(_) => "set_protocol",
                    Params::LegacyServerListPing(_) => "legacy_server_list_ping",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::SetProtocol(val) => PacketSetProtocol::serialize(&val, w)?,
                    Params::LegacyServerListPing(val) => PacketLegacyServerListPing::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "set_protocol" => "0x00",
                    "legacy_server_list_ping" => "0xfe",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|input| {
                        let (input, x) = (VarInt::deserialize)(input)?;
                        let x = format!("{x}");
                        let val = match &x[..] {
                            "0x00" => "set_protocol",
                            "0xfe" => "legacy_server_list_ping",

                            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "set_protocol" => map(PacketSetProtocol::deserialize, Params::SetProtocol)(input),
                        "legacy_server_list_ping" => map(PacketLegacyServerListPing::deserialize, Params::LegacyServerListPing)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
pub mod status {
    pub mod clientbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub struct PacketServerInfo<'a> {
            response: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketServerInfo<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.response, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(response,)| PacketServerInfo { response }))(input)
            }
        }

        pub struct PacketPing {
            time: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPing {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i64::deserialize,)), |(time,)| PacketPing { time }))(input)
            }
        }

        pub enum Params<'a> {
            ServerInfo(PacketServerInfo<'a>),
            Ping(PacketPing),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::ServerInfo(_) => "server_info",
                    Params::Ping(_) => "ping",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::ServerInfo(val) => PacketServerInfo::serialize(&val, w)?,
                    Params::Ping(val) => PacketPing::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "server_info" => "0x00",
                    "ping" => "0x01",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|input| {
                        let (input, x) = (VarInt::deserialize)(input)?;
                        let x = format!("{x}");
                        let val = match &x[..] {
                            "0x00" => "server_info",
                            "0x01" => "ping",

                            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "server_info" => map(PacketServerInfo::deserialize, Params::ServerInfo)(input),
                        "ping" => map(PacketPing::deserialize, Params::Ping)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub struct PacketPingStart {}

        impl<'t> protocol_lib::Packet<'t> for PacketPingStart {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((|i| Ok((i, ())),)), |_| PacketPingStart {}))(input)
            }
        }

        pub struct PacketPing {
            time: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPing {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i64::deserialize,)), |(time,)| PacketPing { time }))(input)
            }
        }

        pub enum Params {
            PingStart(PacketPingStart),
            Ping(PacketPing),
            Default,
        }

        impl Params {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::PingStart(_) => "ping_start",
                    Params::Ping(_) => "ping",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::PingStart(val) => PacketPingStart::serialize(&val, w)?,
                    Params::Ping(val) => PacketPing::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet {
            name: &'static str,
            params: Params,
        }

        impl<'t> protocol_lib::Packet<'t> for Packet {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "ping_start" => "0x00",
                    "ping" => "0x01",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|input| {
                        let (input, x) = (VarInt::deserialize)(input)?;
                        let x = format!("{x}");
                        let val = match &x[..] {
                            "0x00" => "ping_start",
                            "0x01" => "ping",

                            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "ping_start" => map(PacketPingStart::deserialize, Params::PingStart)(input),
                        "ping" => map(PacketPing::deserialize, Params::Ping)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
pub mod login {
    pub mod clientbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub struct PacketDisconnect<'a> {
            reason: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDisconnect<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.reason, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(reason,)| PacketDisconnect { reason }))(input)
            }
        }

        pub struct PacketEncryptionBegin<'a> {
            server_id: VarString<'a>,
            public_key: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEncryptionBegin<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.server_id, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.public_key, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.verify_token, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedBuffer::<'a, VarInt>::deserialize,
                        PrefixedBuffer::<'a, VarInt>::deserialize,
                    )),
                    |(server_id, public_key, verify_token)| PacketEncryptionBegin { server_id, public_key, verify_token },
                ))(input)
            }
        }

        pub struct PacketSuccess<'a> {
            uuid: Uuid,
            username: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSuccess<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.uuid, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.username, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Uuid::deserialize, PrefixedString::<'a, VarInt>::deserialize)), |(uuid, username)| PacketSuccess {
                    uuid,
                    username,
                }))(input)
            }
        }

        pub struct PacketCompress {
            threshold: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCompress {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.threshold, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(threshold,)| PacketCompress { threshold }))(input)
            }
        }

        pub struct PacketLoginPluginRequest<'a> {
            message_id: VarInt,
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketLoginPluginRequest<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.message_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.channel, w)?;
                let w = RestBuffer::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, PrefixedString::<'a, VarInt>::deserialize, RestBuffer::deserialize)),
                    |(message_id, channel, data)| PacketLoginPluginRequest { message_id, channel, data },
                ))(input)
            }
        }

        pub enum Params<'a> {
            Disconnect(PacketDisconnect<'a>),
            EncryptionBegin(PacketEncryptionBegin<'a>),
            Success(PacketSuccess<'a>),
            Compress(PacketCompress),
            LoginPluginRequest(PacketLoginPluginRequest<'a>),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::Disconnect(_) => "disconnect",
                    Params::EncryptionBegin(_) => "encryption_begin",
                    Params::Success(_) => "success",
                    Params::Compress(_) => "compress",
                    Params::LoginPluginRequest(_) => "login_plugin_request",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::Disconnect(val) => PacketDisconnect::serialize(&val, w)?,
                    Params::EncryptionBegin(val) => PacketEncryptionBegin::serialize(&val, w)?,
                    Params::Success(val) => PacketSuccess::serialize(&val, w)?,
                    Params::Compress(val) => PacketCompress::serialize(&val, w)?,
                    Params::LoginPluginRequest(val) => PacketLoginPluginRequest::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "disconnect" => "0x00",
                    "encryption_begin" => "0x01",
                    "success" => "0x02",
                    "compress" => "0x03",
                    "login_plugin_request" => "0x04",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|input| {
                        let (input, x) = (VarInt::deserialize)(input)?;
                        let x = format!("{x}");
                        let val = match &x[..] {
                            "0x00" => "disconnect",
                            "0x01" => "encryption_begin",
                            "0x02" => "success",
                            "0x03" => "compress",
                            "0x04" => "login_plugin_request",

                            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "disconnect" => map(PacketDisconnect::deserialize, Params::Disconnect)(input),
                        "encryption_begin" => map(PacketEncryptionBegin::deserialize, Params::EncryptionBegin)(input),
                        "success" => map(PacketSuccess::deserialize, Params::Success)(input),
                        "compress" => map(PacketCompress::deserialize, Params::Compress)(input),
                        "login_plugin_request" => map(PacketLoginPluginRequest::deserialize, Params::LoginPluginRequest)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub struct PacketLoginStart<'a> {
            username: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketLoginStart<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.username, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(username,)| PacketLoginStart { username }))(input)
            }
        }

        pub struct PacketEncryptionBegin<'a> {
            shared_secret: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEncryptionBegin<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.shared_secret, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.verify_token, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((PrefixedBuffer::<'a, VarInt>::deserialize, PrefixedBuffer::<'a, VarInt>::deserialize)),
                    |(shared_secret, verify_token)| PacketEncryptionBegin { shared_secret, verify_token },
                ))(input)
            }
        }

        pub struct PacketLoginPluginResponse<'a> {
            message_id: VarInt,
            data: Option<RestBuffer<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketLoginPluginResponse<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.message_id, w)?;
                let w = Option::<RestBuffer>::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, Option::<RestBuffer>::deserialize)), |(message_id, data)| PacketLoginPluginResponse {
                    message_id,
                    data,
                }))(input)
            }
        }

        pub enum Params<'a> {
            LoginStart(PacketLoginStart<'a>),
            EncryptionBegin(PacketEncryptionBegin<'a>),
            LoginPluginResponse(PacketLoginPluginResponse<'a>),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::LoginStart(_) => "login_start",
                    Params::EncryptionBegin(_) => "encryption_begin",
                    Params::LoginPluginResponse(_) => "login_plugin_response",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::LoginStart(val) => PacketLoginStart::serialize(&val, w)?,
                    Params::EncryptionBegin(val) => PacketEncryptionBegin::serialize(&val, w)?,
                    Params::LoginPluginResponse(val) => PacketLoginPluginResponse::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "login_start" => "0x00",
                    "encryption_begin" => "0x01",
                    "login_plugin_response" => "0x02",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|input| {
                        let (input, x) = (VarInt::deserialize)(input)?;
                        let x = format!("{x}");
                        let val = match &x[..] {
                            "0x00" => "login_start",
                            "0x01" => "encryption_begin",
                            "0x02" => "login_plugin_response",

                            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "login_start" => map(PacketLoginStart::deserialize, Params::LoginStart)(input),
                        "encryption_begin" => map(PacketEncryptionBegin::deserialize, Params::EncryptionBegin)(input),
                        "login_plugin_response" => map(PacketLoginPluginResponse::deserialize, Params::LoginPluginResponse)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
pub mod play {
    pub mod clientbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub struct PacketSpawnEntity {
            entity_id: VarInt,
            object_uuid: Uuid,
            r_type: VarInt,
            x: f64,
            y: f64,
            z: f64,
            pitch: i8,
            yaw: i8,
            object_data: i32,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSpawnEntity {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = Uuid::serialize(&self.object_uuid, w)?;
                let w = VarInt::serialize(&self.r_type, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = i8::serialize(&self.pitch, w)?;
                let w = i8::serialize(&self.yaw, w)?;
                let w = i32::serialize(&self.object_data, w)?;
                let w = i16::serialize(&self.velocity_x, w)?;
                let w = i16::serialize(&self.velocity_y, w)?;
                let w = i16::serialize(&self.velocity_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        Uuid::deserialize,
                        VarInt::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        i32::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                    )),
                    |(entity_id, object_uuid, r_type, x, y, z, pitch, yaw, object_data, velocity_x, velocity_y, velocity_z)| PacketSpawnEntity {
                        entity_id,
                        object_uuid,
                        r_type,
                        x,
                        y,
                        z,
                        pitch,
                        yaw,
                        object_data,
                        velocity_x,
                        velocity_y,
                        velocity_z,
                    },
                ))(input)
            }
        }

        pub struct PacketSpawnEntityExperienceOrb {
            entity_id: VarInt,
            x: f64,
            y: f64,
            z: f64,
            count: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSpawnEntityExperienceOrb {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = i16::serialize(&self.count, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, f64::deserialize, f64::deserialize, f64::deserialize, i16::deserialize)),
                    |(entity_id, x, y, z, count)| PacketSpawnEntityExperienceOrb { entity_id, x, y, z, count },
                ))(input)
            }
        }

        pub struct PacketSpawnEntityLiving {
            entity_id: VarInt,
            entity_uuid: Uuid,
            r_type: VarInt,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
            head_pitch: i8,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSpawnEntityLiving {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = Uuid::serialize(&self.entity_uuid, w)?;
                let w = VarInt::serialize(&self.r_type, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = i8::serialize(&self.yaw, w)?;
                let w = i8::serialize(&self.pitch, w)?;
                let w = i8::serialize(&self.head_pitch, w)?;
                let w = i16::serialize(&self.velocity_x, w)?;
                let w = i16::serialize(&self.velocity_y, w)?;
                let w = i16::serialize(&self.velocity_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        Uuid::deserialize,
                        VarInt::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                    )),
                    |(entity_id, entity_uuid, r_type, x, y, z, yaw, pitch, head_pitch, velocity_x, velocity_y, velocity_z)| PacketSpawnEntityLiving {
                        entity_id,
                        entity_uuid,
                        r_type,
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                        head_pitch,
                        velocity_x,
                        velocity_y,
                        velocity_z,
                    },
                ))(input)
            }
        }

        pub struct PacketSpawnEntityPainting {
            entity_id: VarInt,
            entity_uuid: Uuid,
            title: VarInt,
            location: Position,
            direction: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSpawnEntityPainting {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = Uuid::serialize(&self.entity_uuid, w)?;
                let w = VarInt::serialize(&self.title, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = u8::serialize(&self.direction, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, Uuid::deserialize, VarInt::deserialize, Position::deserialize, u8::deserialize)),
                    |(entity_id, entity_uuid, title, location, direction)| PacketSpawnEntityPainting {
                        entity_id,
                        entity_uuid,
                        title,
                        location,
                        direction,
                    },
                ))(input)
            }
        }

        pub struct PacketNamedEntitySpawn {
            entity_id: VarInt,
            player_uuid: Uuid,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketNamedEntitySpawn {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = Uuid::serialize(&self.player_uuid, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = i8::serialize(&self.yaw, w)?;
                let w = i8::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        Uuid::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                    )),
                    |(entity_id, player_uuid, x, y, z, yaw, pitch)| PacketNamedEntitySpawn {
                        entity_id,
                        player_uuid,
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                    },
                ))(input)
            }
        }

        pub struct PacketAnimation {
            entity_id: VarInt,
            animation: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAnimation {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = u8::serialize(&self.animation, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, u8::deserialize)), |(entity_id, animation)| PacketAnimation { entity_id, animation }))(input)
            }
        }

        pub struct StatisticsEntry {
            category_id: VarInt,
            statistic_id: VarInt,
            value: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for StatisticsEntry {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.category_id, w)?;
                let w = VarInt::serialize(&self.statistic_id, w)?;
                let w = VarInt::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, VarInt::deserialize, VarInt::deserialize)), |(category_id, statistic_id, value)| {
                    StatisticsEntry { category_id, statistic_id, value }
                }))(input)
            }
        }

        pub struct PacketStatistics {
            entries: VarArray<StatisticsEntry>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketStatistics {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<StatisticsEntry, VarInt>::len(&self.entries).serialize(w)?;

                let mut w = w;
                let items = self.entries.0.iter();
                for i in items {
                    w = StatisticsEntry::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedArray::<StatisticsEntry, VarInt>::deserialize,)), |(entries,)| PacketStatistics { entries }))(input)
            }
        }

        pub struct Ident8Flags {
            unused: u32,
            hidden: u8,
            show_toast: u8,
            has_background_texture: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for Ident8Flags {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = write_bits(
                    &[(self.unused as u64, 29), (self.hidden as u64, 1), (self.show_toast as u64, 1), (self.has_background_texture as u64, 1)],
                    w,
                )?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(
                    tuple((parse_bits_unsigned(29), parse_bits_unsigned(1), parse_bits_unsigned(1), parse_bits_unsigned(1))),
                    |(unused, hidden, show_toast, has_background_texture)| Ident8Flags {
                        unused,
                        hidden,
                        show_toast,
                        has_background_texture,
                    },
                )))(input)
            }
        }

        pub enum BackgroundTexture<'a> {
            BackgroundTexture1(VarString<'a>),
            Default,
        }

        impl<'a> BackgroundTexture<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    BackgroundTexture::BackgroundTexture1(_) => "1",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    BackgroundTexture::BackgroundTexture1(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    BackgroundTexture::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Ident8<'a> {
            title: VarString<'a>,
            description: VarString<'a>,
            icon: Slot,
            frame_type: VarInt,
            flags: Ident8Flags,
            background_texture: BackgroundTexture<'a>,
            x_cord: f32,
            y_cord: f32,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Ident8<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.title, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.description, w)?;
                let w = Slot::serialize(&self.icon, w)?;
                let w = VarInt::serialize(&self.frame_type, w)?;
                let w = Ident8Flags::serialize(&self.flags, w)?;
                let w = BackgroundTexture::serialize(&self.background_texture, w)?;
                let w = f32::serialize(&self.x_cord, w)?;
                let w = f32::serialize(&self.y_cord, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_title) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_description) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_icon) = (Slot::deserialize)(input)?;
                    let (input, self_frame_type) = (VarInt::deserialize)(input)?;
                    let (input, self_flags) = (Ident8Flags::deserialize)(input)?;
                    let (input, self_background_texture) = (|input| match &format!("{}", self_flags.has_background_texture)[..] {
                        "1" => map(PrefixedString::<'a, VarInt>::deserialize, BackgroundTexture::BackgroundTexture1)(input),
                        _ => Ok((input, BackgroundTexture::Default)),
                    })(input)?;
                    let (input, self_x_cord) = (f32::deserialize)(input)?;
                    let (input, self_y_cord) = (f32::deserialize)(input)?;
                    Ok((
                        input,
                        Ident8 {
                            title: self_title,
                            description: self_description,
                            icon: self_icon,
                            frame_type: self_frame_type,
                            flags: self_flags,
                            background_texture: self_background_texture,
                            x_cord: self_x_cord,
                            y_cord: self_y_cord,
                        },
                    ))
                })(input)
            }
        }

        pub struct CriteriaItem<'a> {
            key: VarString<'a>,
            value: Void,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for CriteriaItem<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;
                let w = Void::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, Void::deserialize)), |(key, value)| CriteriaItem { key, value }))(input)
            }
        }

        pub struct AdvancementMappingItemValue<'a> {
            parent_id: Option<VarString<'a>>,
            display_data: Option<Ident8<'a>>,
            criteria: VarArray<CriteriaItem<'a>>,
            requirements: VarArray<VarStringArray<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for AdvancementMappingItemValue<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Option::<VarString<'a>>::serialize(&self.parent_id, w)?;
                let w = Option::<Ident8>::serialize(&self.display_data, w)?;

                let w = PrefixedArray::<CriteriaItem, VarInt>::len(&self.criteria).serialize(w)?;

                let mut w = w;
                let items = self.criteria.0.iter();
                for i in items {
                    w = CriteriaItem::serialize(&i, w)?
                }

                let w = PrefixedArray::<VarStringArray<'a>, VarInt>::len(&self.requirements).serialize(w)?;

                let mut w = w;
                let items = self.requirements.0.iter();
                for i in items {
                    w = {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::len(&i).serialize(w)?;

                        let mut w = w;
                        let items = i.0.iter();
                        for i in items {
                            w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                        }
                        w
                    }
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        Option::<VarString<'a>>::deserialize,
                        Option::<Ident8>::deserialize,
                        PrefixedArray::<CriteriaItem<'a>, VarInt>::deserialize,
                        PrefixedArray::<VarStringArray<'a>, VarInt>::deserialize,
                    )),
                    |(parent_id, display_data, criteria, requirements)| AdvancementMappingItemValue {
                        parent_id,
                        display_data,
                        criteria,
                        requirements,
                    },
                ))(input)
            }
        }

        pub struct AdvancementMappingItem<'a> {
            key: VarString<'a>,
            value: AdvancementMappingItemValue<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for AdvancementMappingItem<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;
                let w = AdvancementMappingItemValue::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, AdvancementMappingItemValue::deserialize)), |(key, value)| {
                    AdvancementMappingItem { key, value }
                }))(input)
            }
        }

        pub struct ProgressMappingItemValueItem<'a> {
            criterion_identifier: VarString<'a>,
            criterion_progress: Option<i64>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for ProgressMappingItemValueItem<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.criterion_identifier, w)?;
                let w = Option::<i64>::serialize(&self.criterion_progress, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((PrefixedString::<'a, VarInt>::deserialize, Option::<i64>::deserialize)),
                    |(criterion_identifier, criterion_progress)| ProgressMappingItemValueItem {
                        criterion_identifier,
                        criterion_progress,
                    },
                ))(input)
            }
        }

        pub struct ProgressMappingItem<'a> {
            key: VarString<'a>,
            value: VarArray<ProgressMappingItemValueItem<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for ProgressMappingItem<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;

                let w = PrefixedArray::<ProgressMappingItemValueItem, VarInt>::len(&self.value).serialize(w)?;

                let mut w = w;
                let items = self.value.0.iter();
                for i in items {
                    w = ProgressMappingItemValueItem::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((PrefixedString::<'a, VarInt>::deserialize, PrefixedArray::<ProgressMappingItemValueItem<'a>, VarInt>::deserialize)),
                    |(key, value)| ProgressMappingItem { key, value },
                ))(input)
            }
        }

        pub struct PacketAdvancements<'a> {
            reset: bool,
            advancement_mapping: VarArray<AdvancementMappingItem<'a>>,
            identifiers: VarStringArray<'a>,
            progress_mapping: VarArray<ProgressMappingItem<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketAdvancements<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.reset, w)?;

                let w = PrefixedArray::<AdvancementMappingItem, VarInt>::len(&self.advancement_mapping).serialize(w)?;

                let mut w = w;
                let items = self.advancement_mapping.0.iter();
                for i in items {
                    w = AdvancementMappingItem::serialize(&i, w)?
                }

                let w = PrefixedArray::<VarString<'a>, VarInt>::len(&self.identifiers).serialize(w)?;

                let mut w = w;
                let items = self.identifiers.0.iter();
                for i in items {
                    w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                }

                let w = PrefixedArray::<ProgressMappingItem, VarInt>::len(&self.progress_mapping).serialize(w)?;

                let mut w = w;
                let items = self.progress_mapping.0.iter();
                for i in items {
                    w = ProgressMappingItem::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        bool::deserialize,
                        PrefixedArray::<AdvancementMappingItem<'a>, VarInt>::deserialize,
                        PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                        PrefixedArray::<ProgressMappingItem<'a>, VarInt>::deserialize,
                    )),
                    |(reset, advancement_mapping, identifiers, progress_mapping)| PacketAdvancements {
                        reset,
                        advancement_mapping,
                        identifiers,
                        progress_mapping,
                    },
                ))(input)
            }
        }

        pub struct PacketBlockBreakAnimation {
            entity_id: VarInt,
            location: Position,
            destroy_stage: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockBreakAnimation {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = i8::serialize(&self.destroy_stage, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, Position::deserialize, i8::deserialize)), |(entity_id, location, destroy_stage)| {
                    PacketBlockBreakAnimation { entity_id, location, destroy_stage }
                }))(input)
            }
        }

        pub struct PacketTileEntityData {
            location: Position,
            action: VarInt,
            nbt_data: OptionalNbt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketTileEntityData {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.action, w)?;
                let w = OptionalNbt::serialize(&self.nbt_data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Position::deserialize, VarInt::deserialize, OptionalNbt::deserialize)), |(location, action, nbt_data)| {
                    PacketTileEntityData { location, action, nbt_data }
                }))(input)
            }
        }

        pub struct PacketBlockAction {
            location: Position,
            byte1: u8,
            byte2: u8,
            block_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockAction {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = u8::serialize(&self.byte1, w)?;
                let w = u8::serialize(&self.byte2, w)?;
                let w = VarInt::serialize(&self.block_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((Position::deserialize, u8::deserialize, u8::deserialize, VarInt::deserialize)),
                    |(location, byte1, byte2, block_id)| PacketBlockAction { location, byte1, byte2, block_id },
                ))(input)
            }
        }

        pub struct PacketBlockChange {
            location: Position,
            r_type: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockChange {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.r_type, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Position::deserialize, VarInt::deserialize)), |(location, r_type)| PacketBlockChange { location, r_type }))(input)
            }
        }

        pub enum BossBarTitle<'a> {
            BossBarTitle0(VarString<'a>),
            BossBarTitle3(VarString<'a>),
            Default,
        }

        impl<'a> BossBarTitle<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    BossBarTitle::BossBarTitle0(_) => "0",
                    BossBarTitle::BossBarTitle3(_) => "3",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    BossBarTitle::BossBarTitle0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    BossBarTitle::BossBarTitle3(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    BossBarTitle::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Health {
            Health0(f32),
            Health2(f32),
            Default,
        }

        impl Health {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Health::Health0(_) => "0",
                    Health::Health2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Health::Health0(val) => f32::serialize(&val, w)?,
                    Health::Health2(val) => f32::serialize(&val, w)?,
                    Health::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Color {
            Color0(VarInt),
            Color4(VarInt),
            Default,
        }

        impl Color {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Color::Color0(_) => "0",
                    Color::Color4(_) => "4",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Color::Color0(val) => VarInt::serialize(&val, w)?,
                    Color::Color4(val) => VarInt::serialize(&val, w)?,
                    Color::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Dividers {
            Dividers0(VarInt),
            Dividers4(VarInt),
            Default,
        }

        impl Dividers {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Dividers::Dividers0(_) => "0",
                    Dividers::Dividers4(_) => "4",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Dividers::Dividers0(val) => VarInt::serialize(&val, w)?,
                    Dividers::Dividers4(val) => VarInt::serialize(&val, w)?,
                    Dividers::Default => w,
                };

                Ok(w)
            }
        }
        pub enum BossBarFlags {
            BossBarFlags0(u8),
            BossBarFlags5(u8),
            Default,
        }

        impl BossBarFlags {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    BossBarFlags::BossBarFlags0(_) => "0",
                    BossBarFlags::BossBarFlags5(_) => "5",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    BossBarFlags::BossBarFlags0(val) => u8::serialize(&val, w)?,
                    BossBarFlags::BossBarFlags5(val) => u8::serialize(&val, w)?,
                    BossBarFlags::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketBossBar<'a> {
            entity_uuid: Uuid,
            action: VarInt,
            title: BossBarTitle<'a>,
            health: Health,
            color: Color,
            dividers: Dividers,
            flags: BossBarFlags,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketBossBar<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.entity_uuid, w)?;
                let w = VarInt::serialize(&self.action, w)?;
                let w = BossBarTitle::serialize(&self.title, w)?;
                let w = Health::serialize(&self.health, w)?;
                let w = Color::serialize(&self.color, w)?;
                let w = Dividers::serialize(&self.dividers, w)?;
                let w = BossBarFlags::serialize(&self.flags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_entity_uuid) = (Uuid::deserialize)(input)?;
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_title) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, BossBarTitle::BossBarTitle0)(input),
                        "3" => map(PrefixedString::<'a, VarInt>::deserialize, BossBarTitle::BossBarTitle3)(input),
                        _ => Ok((input, BossBarTitle::Default)),
                    })(input)?;
                    let (input, self_health) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(f32::deserialize, Health::Health0)(input),
                        "2" => map(f32::deserialize, Health::Health2)(input),
                        _ => Ok((input, Health::Default)),
                    })(input)?;
                    let (input, self_color) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(VarInt::deserialize, Color::Color0)(input),
                        "4" => map(VarInt::deserialize, Color::Color4)(input),
                        _ => Ok((input, Color::Default)),
                    })(input)?;
                    let (input, self_dividers) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(VarInt::deserialize, Dividers::Dividers0)(input),
                        "4" => map(VarInt::deserialize, Dividers::Dividers4)(input),
                        _ => Ok((input, Dividers::Default)),
                    })(input)?;
                    let (input, self_flags) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(u8::deserialize, BossBarFlags::BossBarFlags0)(input),
                        "5" => map(u8::deserialize, BossBarFlags::BossBarFlags5)(input),
                        _ => Ok((input, BossBarFlags::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketBossBar {
                            entity_uuid: self_entity_uuid,
                            action: self_action,
                            title: self_title,
                            health: self_health,
                            color: self_color,
                            dividers: self_dividers,
                            flags: self_flags,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketDifficulty {
            difficulty: u8,
            difficulty_locked: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketDifficulty {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.difficulty, w)?;
                let w = bool::serialize(&self.difficulty_locked, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize, bool::deserialize)), |(difficulty, difficulty_locked)| PacketDifficulty {
                    difficulty,
                    difficulty_locked,
                }))(input)
            }
        }

        pub struct Matche<'a> {
            r_match: VarString<'a>,
            tooltip: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Matche<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.r_match, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.tooltip, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, Option::<VarString<'a>>::deserialize)), |(r_match, tooltip)| Matche {
                    r_match,
                    tooltip,
                }))(input)
            }
        }

        pub struct PacketTabComplete<'a> {
            transaction_id: VarInt,
            start: VarInt,
            length: VarInt,
            matches: VarArray<Matche<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketTabComplete<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = VarInt::serialize(&self.start, w)?;
                let w = VarInt::serialize(&self.length, w)?;

                let w = PrefixedArray::<Matche, VarInt>::len(&self.matches).serialize(w)?;

                let mut w = w;
                let items = self.matches.0.iter();
                for i in items {
                    w = Matche::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, VarInt::deserialize, VarInt::deserialize, PrefixedArray::<Matche<'a>, VarInt>::deserialize)),
                    |(transaction_id, start, length, matches)| PacketTabComplete {
                        transaction_id,
                        start,
                        length,
                        matches,
                    },
                ))(input)
            }
        }

        pub struct PacketDeclareCommands<'a> {
            nodes: VarArray<CommandNode<'a>>,
            root_index: VarInt,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDeclareCommands<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<CommandNode, VarInt>::len(&self.nodes).serialize(w)?;

                let mut w = w;
                let items = self.nodes.0.iter();
                for i in items {
                    w = CommandNode::serialize(&i, w)?
                }

                let w = VarInt::serialize(&self.root_index, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedArray::<CommandNode<'a>, VarInt>::deserialize, VarInt::deserialize)), |(nodes, root_index)| {
                    PacketDeclareCommands { nodes, root_index }
                }))(input)
            }
        }

        pub enum FacePlayerEntityId {
            True(VarInt),
            Default,
        }

        impl FacePlayerEntityId {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    FacePlayerEntityId::True(_) => "true",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    FacePlayerEntityId::True(val) => VarInt::serialize(&val, w)?,
                    FacePlayerEntityId::Default => w,
                };

                Ok(w)
            }
        }
        pub enum EntityFeetEyes<'a> {
            True(VarString<'a>),
            Default,
        }

        impl<'a> EntityFeetEyes<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    EntityFeetEyes::True(_) => "true",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    EntityFeetEyes::True(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    EntityFeetEyes::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketFacePlayer<'a> {
            feet_eyes: VarInt,
            x: f64,
            y: f64,
            z: f64,
            is_entity: bool,
            entity_id: FacePlayerEntityId,
            entity_feet_eyes: EntityFeetEyes<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketFacePlayer<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.feet_eyes, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = bool::serialize(&self.is_entity, w)?;
                let w = FacePlayerEntityId::serialize(&self.entity_id, w)?;
                let w = EntityFeetEyes::serialize(&self.entity_feet_eyes, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_feet_eyes) = (VarInt::deserialize)(input)?;
                    let (input, self_x) = (f64::deserialize)(input)?;
                    let (input, self_y) = (f64::deserialize)(input)?;
                    let (input, self_z) = (f64::deserialize)(input)?;
                    let (input, self_is_entity) = (bool::deserialize)(input)?;
                    let (input, self_entity_id) = (|input| match &format!("{}", self_is_entity)[..] {
                        "true" => map(VarInt::deserialize, FacePlayerEntityId::True)(input),
                        _ => Ok((input, FacePlayerEntityId::Default)),
                    })(input)?;
                    let (input, self_entity_feet_eyes) = (|input| match &format!("{}", self_is_entity)[..] {
                        "true" => map(PrefixedString::<'a, VarInt>::deserialize, EntityFeetEyes::True)(input),
                        _ => Ok((input, EntityFeetEyes::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketFacePlayer {
                            feet_eyes: self_feet_eyes,
                            x: self_x,
                            y: self_y,
                            z: self_z,
                            is_entity: self_is_entity,
                            entity_id: self_entity_id,
                            entity_feet_eyes: self_entity_feet_eyes,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketNbtQueryResponse {
            transaction_id: VarInt,
            nbt: OptionalNbt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketNbtQueryResponse {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = OptionalNbt::serialize(&self.nbt, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, OptionalNbt::deserialize)), |(transaction_id, nbt)| PacketNbtQueryResponse {
                    transaction_id,
                    nbt,
                }))(input)
            }
        }

        pub struct PacketChat<'a> {
            message: VarString<'a>,
            position: i8,
            sender: Uuid,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketChat<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.message, w)?;
                let w = i8::serialize(&self.position, w)?;
                let w = Uuid::serialize(&self.sender, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, i8::deserialize, Uuid::deserialize)), |(message, position, sender)| {
                    PacketChat { message, position, sender }
                }))(input)
            }
        }

        pub struct ChunkCoordinates {
            x: i32,
            z: i32,
            y: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for ChunkCoordinates {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = write_bits(
                    &[
                        (unsafe { core::mem::transmute(self.x as i64) }, 22),
                        (unsafe { core::mem::transmute(self.z as i64) }, 22),
                        (unsafe { core::mem::transmute(self.y as i64) }, 20),
                    ],
                    w,
                )?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(tuple((parse_bits_signed(22), parse_bits_signed(22), parse_bits_signed(20))), |(x, z, y)| ChunkCoordinates {
                    x,
                    z,
                    y,
                })))(input)
            }
        }

        pub struct PacketMultiBlockChange {
            chunk_coordinates: ChunkCoordinates,
            not_trust_edges: bool,
            records: VarArray<VarLong>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketMultiBlockChange {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = ChunkCoordinates::serialize(&self.chunk_coordinates, w)?;
                let w = bool::serialize(&self.not_trust_edges, w)?;

                let w = PrefixedArray::<VarLong, VarInt>::len(&self.records).serialize(w)?;

                let mut w = w;
                let items = self.records.0.iter();
                for i in items {
                    w = VarLong::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((ChunkCoordinates::deserialize, bool::deserialize, PrefixedArray::<VarLong, VarInt>::deserialize)),
                    |(chunk_coordinates, not_trust_edges, records)| PacketMultiBlockChange {
                        chunk_coordinates,
                        not_trust_edges,
                        records,
                    },
                ))(input)
            }
        }

        pub struct PacketCloseWindow {
            window_id: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCloseWindow {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize,)), |(window_id,)| PacketCloseWindow { window_id }))(input)
            }
        }

        pub struct PacketOpenWindow<'a> {
            window_id: VarInt,
            inventory_type: VarInt,
            window_title: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketOpenWindow<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.inventory_type, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.window_title, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, VarInt::deserialize, PrefixedString::<'a, VarInt>::deserialize)),
                    |(window_id, inventory_type, window_title)| PacketOpenWindow {
                        window_id,
                        inventory_type,
                        window_title,
                    },
                ))(input)
            }
        }

        pub struct PacketWindowItems {
            window_id: u8,
            state_id: VarInt,
            items: VarArray<Slot>,
            carried_item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWindowItems {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.state_id, w)?;

                let w = PrefixedArray::<Slot, VarInt>::len(&self.items).serialize(w)?;

                let mut w = w;
                let items = self.items.0.iter();
                for i in items {
                    w = Slot::serialize(&i, w)?
                }

                let w = Slot::serialize(&self.carried_item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((u8::deserialize, VarInt::deserialize, PrefixedArray::<Slot, VarInt>::deserialize, Slot::deserialize)),
                    |(window_id, state_id, items, carried_item)| PacketWindowItems {
                        window_id,
                        state_id,
                        items,
                        carried_item,
                    },
                ))(input)
            }
        }

        pub struct PacketCraftProgressBar {
            window_id: u8,
            property: i16,
            value: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCraftProgressBar {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = i16::serialize(&self.property, w)?;
                let w = i16::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize, i16::deserialize, i16::deserialize)), |(window_id, property, value)| PacketCraftProgressBar {
                    window_id,
                    property,
                    value,
                }))(input)
            }
        }

        pub struct PacketSetSlot {
            window_id: i8,
            state_id: VarInt,
            slot: i16,
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetSlot {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.state_id, w)?;
                let w = i16::serialize(&self.slot, w)?;
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((i8::deserialize, VarInt::deserialize, i16::deserialize, Slot::deserialize)),
                    |(window_id, state_id, slot, item)| PacketSetSlot { window_id, state_id, slot, item },
                ))(input)
            }
        }

        pub struct PacketSetCooldown {
            item_id: VarInt,
            cooldown_ticks: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetCooldown {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.item_id, w)?;
                let w = VarInt::serialize(&self.cooldown_ticks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, VarInt::deserialize)), |(item_id, cooldown_ticks)| PacketSetCooldown {
                    item_id,
                    cooldown_ticks,
                }))(input)
            }
        }

        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCustomPayload<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.channel, w)?;
                let w = RestBuffer::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, RestBuffer::deserialize)), |(channel, data)| PacketCustomPayload {
                    channel,
                    data,
                }))(input)
            }
        }

        pub struct PacketNamedSoundEffect<'a> {
            sound_name: VarString<'a>,
            sound_category: VarInt,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketNamedSoundEffect<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.sound_name, w)?;
                let w = VarInt::serialize(&self.sound_category, w)?;
                let w = i32::serialize(&self.x, w)?;
                let w = i32::serialize(&self.y, w)?;
                let w = i32::serialize(&self.z, w)?;
                let w = f32::serialize(&self.volume, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        VarInt::deserialize,
                        i32::deserialize,
                        i32::deserialize,
                        i32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(sound_name, sound_category, x, y, z, volume, pitch)| PacketNamedSoundEffect {
                        sound_name,
                        sound_category,
                        x,
                        y,
                        z,
                        volume,
                        pitch,
                    },
                ))(input)
            }
        }

        pub struct PacketKickDisconnect<'a> {
            reason: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketKickDisconnect<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.reason, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(reason,)| PacketKickDisconnect { reason }))(input)
            }
        }

        pub struct PacketEntityStatus {
            entity_id: i32,
            entity_status: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityStatus {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.entity_status, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i32::deserialize, i8::deserialize)), |(entity_id, entity_status)| PacketEntityStatus { entity_id, entity_status }))(input)
            }
        }

        pub struct AffectedBlockOffset {
            x: i8,
            y: i8,
            z: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for AffectedBlockOffset {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.x, w)?;
                let w = i8::serialize(&self.y, w)?;
                let w = i8::serialize(&self.z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i8::deserialize, i8::deserialize, i8::deserialize)), |(x, y, z)| AffectedBlockOffset { x, y, z }))(input)
            }
        }

        pub struct PacketExplosion {
            x: f32,
            y: f32,
            z: f32,
            radius: f32,
            affected_block_offsets: VarArray<AffectedBlockOffset>,
            player_motion_x: f32,
            player_motion_y: f32,
            player_motion_z: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketExplosion {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.x, w)?;
                let w = f32::serialize(&self.y, w)?;
                let w = f32::serialize(&self.z, w)?;
                let w = f32::serialize(&self.radius, w)?;

                let w = PrefixedArray::<AffectedBlockOffset, VarInt>::len(&self.affected_block_offsets).serialize(w)?;

                let mut w = w;
                let items = self.affected_block_offsets.0.iter();
                for i in items {
                    w = AffectedBlockOffset::serialize(&i, w)?
                }

                let w = f32::serialize(&self.player_motion_x, w)?;
                let w = f32::serialize(&self.player_motion_y, w)?;
                let w = f32::serialize(&self.player_motion_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        PrefixedArray::<AffectedBlockOffset, VarInt>::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(x, y, z, radius, affected_block_offsets, player_motion_x, player_motion_y, player_motion_z)| PacketExplosion {
                        x,
                        y,
                        z,
                        radius,
                        affected_block_offsets,
                        player_motion_x,
                        player_motion_y,
                        player_motion_z,
                    },
                ))(input)
            }
        }

        pub struct PacketUnloadChunk {
            chunk_x: i32,
            chunk_z: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUnloadChunk {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.chunk_x, w)?;
                let w = i32::serialize(&self.chunk_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i32::deserialize, i32::deserialize)), |(chunk_x, chunk_z)| PacketUnloadChunk { chunk_x, chunk_z }))(input)
            }
        }

        pub struct PacketGameStateChange {
            reason: u8,
            game_mode: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketGameStateChange {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.reason, w)?;
                let w = f32::serialize(&self.game_mode, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize, f32::deserialize)), |(reason, game_mode)| PacketGameStateChange { reason, game_mode }))(input)
            }
        }

        pub struct PacketOpenHorseWindow {
            window_id: u8,
            nb_slots: VarInt,
            entity_id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketOpenHorseWindow {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.nb_slots, w)?;
                let w = i32::serialize(&self.entity_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize, VarInt::deserialize, i32::deserialize)), |(window_id, nb_slots, entity_id)| {
                    PacketOpenHorseWindow { window_id, nb_slots, entity_id }
                }))(input)
            }
        }

        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketKeepAlive {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.keep_alive_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i64::deserialize,)), |(keep_alive_id,)| PacketKeepAlive { keep_alive_id }))(input)
            }
        }

        pub struct PacketMapChunk<'a> {
            x: i32,
            z: i32,
            heightmaps: Nbt,
            chunk_data: VarBuffer<'a>,
            block_entities: VarArray<ChunkBlockEntity>,
            trust_edges: bool,
            sky_light_mask: VarArray<i64>,
            block_light_mask: VarArray<i64>,
            empty_sky_light_mask: VarArray<i64>,
            empty_block_light_mask: VarArray<i64>,
            sky_light: VarArray<VarArray<u8>>,
            block_light: VarArray<VarArray<u8>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketMapChunk<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.x, w)?;
                let w = i32::serialize(&self.z, w)?;
                let w = Nbt::serialize(&self.heightmaps, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.chunk_data, w)?;

                let w = PrefixedArray::<ChunkBlockEntity, VarInt>::len(&self.block_entities).serialize(w)?;

                let mut w = w;
                let items = self.block_entities.0.iter();
                for i in items {
                    w = ChunkBlockEntity::serialize(&i, w)?
                }

                let w = bool::serialize(&self.trust_edges, w)?;

                let w = PrefixedArray::<i64, VarInt>::len(&self.sky_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.sky_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<i64, VarInt>::len(&self.block_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.block_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<i64, VarInt>::len(&self.empty_sky_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.empty_sky_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<i64, VarInt>::len(&self.empty_block_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.empty_block_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<VarArray<u8>, VarInt>::len(&self.sky_light).serialize(w)?;

                let mut w = w;
                let items = self.sky_light.0.iter();
                for i in items {
                    w = {
                        let w = PrefixedArray::<u8, VarInt>::len(&i).serialize(w)?;

                        let mut w = w;
                        let items = i.0.iter();
                        for i in items {
                            w = u8::serialize(&i, w)?
                        }
                        w
                    }
                }

                let w = PrefixedArray::<VarArray<u8>, VarInt>::len(&self.block_light).serialize(w)?;

                let mut w = w;
                let items = self.block_light.0.iter();
                for i in items {
                    w = {
                        let w = PrefixedArray::<u8, VarInt>::len(&i).serialize(w)?;

                        let mut w = w;
                        let items = i.0.iter();
                        for i in items {
                            w = u8::serialize(&i, w)?
                        }
                        w
                    }
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        i32::deserialize,
                        i32::deserialize,
                        Nbt::deserialize,
                        PrefixedBuffer::<'a, VarInt>::deserialize,
                        PrefixedArray::<ChunkBlockEntity, VarInt>::deserialize,
                        bool::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<VarArray<u8>, VarInt>::deserialize,
                        PrefixedArray::<VarArray<u8>, VarInt>::deserialize,
                    )),
                    |(x, z, heightmaps, chunk_data, block_entities, trust_edges, sky_light_mask, block_light_mask, empty_sky_light_mask, empty_block_light_mask, sky_light, block_light)| {
                        PacketMapChunk {
                            x,
                            z,
                            heightmaps,
                            chunk_data,
                            block_entities,
                            trust_edges,
                            sky_light_mask,
                            block_light_mask,
                            empty_sky_light_mask,
                            empty_block_light_mask,
                            sky_light,
                            block_light,
                        }
                    },
                ))(input)
            }
        }

        pub struct PacketWorldEvent {
            effect_id: i32,
            location: Position,
            data: i32,
            global: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldEvent {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.effect_id, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = i32::serialize(&self.data, w)?;
                let w = bool::serialize(&self.global, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((i32::deserialize, Position::deserialize, i32::deserialize, bool::deserialize)),
                    |(effect_id, location, data, global)| PacketWorldEvent { effect_id, location, data, global },
                ))(input)
            }
        }

        pub struct WorldParticlesData2 {
            block_state: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData2 {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.block_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(block_state,)| WorldParticlesData2 { block_state }))(input)
            }
        }

        pub struct WorldParticlesData3 {
            block_state: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData3 {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.block_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(block_state,)| WorldParticlesData3 { block_state }))(input)
            }
        }

        pub struct WorldParticlesData14 {
            red: f32,
            green: f32,
            blue: f32,
            scale: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData14 {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.red, w)?;
                let w = f32::serialize(&self.green, w)?;
                let w = f32::serialize(&self.blue, w)?;
                let w = f32::serialize(&self.scale, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f32::deserialize, f32::deserialize, f32::deserialize, f32::deserialize)), |(red, green, blue, scale)| {
                    WorldParticlesData14 { red, green, blue, scale }
                }))(input)
            }
        }

        pub struct WorldParticlesData15 {
            from_red: f32,
            from_green: f32,
            from_blue: f32,
            scale: f32,
            to_red: f32,
            to_green: f32,
            to_blue: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData15 {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.from_red, w)?;
                let w = f32::serialize(&self.from_green, w)?;
                let w = f32::serialize(&self.from_blue, w)?;
                let w = f32::serialize(&self.scale, w)?;
                let w = f32::serialize(&self.to_red, w)?;
                let w = f32::serialize(&self.to_green, w)?;
                let w = f32::serialize(&self.to_blue, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(from_red, from_green, from_blue, scale, to_red, to_green, to_blue)| WorldParticlesData15 {
                        from_red,
                        from_green,
                        from_blue,
                        scale,
                        to_red,
                        to_green,
                        to_blue,
                    },
                ))(input)
            }
        }

        pub struct WorldParticlesData24 {
            block_state: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData24 {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.block_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(block_state,)| WorldParticlesData24 { block_state }))(input)
            }
        }

        pub struct WorldParticlesData35 {
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData35 {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Slot::deserialize,)), |(item,)| WorldParticlesData35 { item }))(input)
            }
        }

        pub enum WorldParticlesData36Destination {
            MinecraftBlock(Position),
            Entity(VarInt),
            Default,
        }

        impl WorldParticlesData36Destination {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    WorldParticlesData36Destination::MinecraftBlock(_) => "minecraft:block",
                    WorldParticlesData36Destination::Entity(_) => "minecraft:entity",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    WorldParticlesData36Destination::MinecraftBlock(val) => Position::serialize(&val, w)?,
                    WorldParticlesData36Destination::Entity(val) => VarInt::serialize(&val, w)?,
                    WorldParticlesData36Destination::Default => w,
                };

                Ok(w)
            }
        }
        pub struct WorldParticlesData36<'a> {
            origin: Position,
            position_type: VarString<'a>,
            destination: WorldParticlesData36Destination,
            ticks: VarInt,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for WorldParticlesData36<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.origin, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.position_type, w)?;
                let w = WorldParticlesData36Destination::serialize(&self.destination, w)?;
                let w = VarInt::serialize(&self.ticks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_origin) = (Position::deserialize)(input)?;
                    let (input, self_position_type) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_destination) = (|input| match &format!("{}", self_position_type)[..] {
                        "minecraft:block" => map(Position::deserialize, WorldParticlesData36Destination::MinecraftBlock)(input),
                        "minecraft:entity" => map(VarInt::deserialize, WorldParticlesData36Destination::Entity)(input),
                        _ => Ok((input, WorldParticlesData36Destination::Default)),
                    })(input)?;
                    let (input, self_ticks) = (VarInt::deserialize)(input)?;
                    Ok((
                        input,
                        WorldParticlesData36 {
                            origin: self_origin,
                            position_type: self_position_type,
                            destination: self_destination,
                            ticks: self_ticks,
                        },
                    ))
                })(input)
            }
        }

        pub enum WorldParticlesData<'a> {
            WorldParticlesData2(WorldParticlesData2),
            WorldParticlesData3(WorldParticlesData3),
            WorldParticlesData14(WorldParticlesData14),
            WorldParticlesData15(WorldParticlesData15),
            WorldParticlesData24(WorldParticlesData24),
            WorldParticlesData35(WorldParticlesData35),
            WorldParticlesData36(WorldParticlesData36<'a>),
            Default,
        }

        impl<'a> WorldParticlesData<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    WorldParticlesData::WorldParticlesData2(_) => "2",
                    WorldParticlesData::WorldParticlesData3(_) => "3",
                    WorldParticlesData::WorldParticlesData14(_) => "14",
                    WorldParticlesData::WorldParticlesData15(_) => "15",
                    WorldParticlesData::WorldParticlesData24(_) => "24",
                    WorldParticlesData::WorldParticlesData35(_) => "35",
                    WorldParticlesData::WorldParticlesData36(_) => "36",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    WorldParticlesData::WorldParticlesData2(val) => WorldParticlesData2::serialize(&val, w)?,
                    WorldParticlesData::WorldParticlesData3(val) => WorldParticlesData3::serialize(&val, w)?,
                    WorldParticlesData::WorldParticlesData14(val) => WorldParticlesData14::serialize(&val, w)?,
                    WorldParticlesData::WorldParticlesData15(val) => WorldParticlesData15::serialize(&val, w)?,
                    WorldParticlesData::WorldParticlesData24(val) => WorldParticlesData24::serialize(&val, w)?,
                    WorldParticlesData::WorldParticlesData35(val) => WorldParticlesData35::serialize(&val, w)?,
                    WorldParticlesData::WorldParticlesData36(val) => WorldParticlesData36::serialize(&val, w)?,
                    WorldParticlesData::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketWorldParticles<'a> {
            particle_id: i32,
            long_distance: bool,
            x: f64,
            y: f64,
            z: f64,
            offset_x: f32,
            offset_y: f32,
            offset_z: f32,
            particle_data: f32,
            particles: i32,
            data: WorldParticlesData<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketWorldParticles<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.particle_id, w)?;
                let w = bool::serialize(&self.long_distance, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.offset_x, w)?;
                let w = f32::serialize(&self.offset_y, w)?;
                let w = f32::serialize(&self.offset_z, w)?;
                let w = f32::serialize(&self.particle_data, w)?;
                let w = i32::serialize(&self.particles, w)?;
                let w = WorldParticlesData::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_particle_id) = (i32::deserialize)(input)?;
                    let (input, self_long_distance) = (bool::deserialize)(input)?;
                    let (input, self_x) = (f64::deserialize)(input)?;
                    let (input, self_y) = (f64::deserialize)(input)?;
                    let (input, self_z) = (f64::deserialize)(input)?;
                    let (input, self_offset_x) = (f32::deserialize)(input)?;
                    let (input, self_offset_y) = (f32::deserialize)(input)?;
                    let (input, self_offset_z) = (f32::deserialize)(input)?;
                    let (input, self_particle_data) = (f32::deserialize)(input)?;
                    let (input, self_particles) = (i32::deserialize)(input)?;
                    let (input, self_data) = (|input| match &format!("{}", self_particle_id)[..] {
                        "2" => map(WorldParticlesData2::deserialize, WorldParticlesData::WorldParticlesData2)(input),
                        "3" => map(WorldParticlesData3::deserialize, WorldParticlesData::WorldParticlesData3)(input),
                        "14" => map(WorldParticlesData14::deserialize, WorldParticlesData::WorldParticlesData14)(input),
                        "15" => map(WorldParticlesData15::deserialize, WorldParticlesData::WorldParticlesData15)(input),
                        "24" => map(WorldParticlesData24::deserialize, WorldParticlesData::WorldParticlesData24)(input),
                        "35" => map(WorldParticlesData35::deserialize, WorldParticlesData::WorldParticlesData35)(input),
                        "36" => map(WorldParticlesData36::deserialize, WorldParticlesData::WorldParticlesData36)(input),
                        _ => Ok((input, WorldParticlesData::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketWorldParticles {
                            particle_id: self_particle_id,
                            long_distance: self_long_distance,
                            x: self_x,
                            y: self_y,
                            z: self_z,
                            offset_x: self_offset_x,
                            offset_y: self_offset_y,
                            offset_z: self_offset_z,
                            particle_data: self_particle_data,
                            particles: self_particles,
                            data: self_data,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketUpdateLight {
            chunk_x: VarInt,
            chunk_z: VarInt,
            trust_edges: bool,
            sky_light_mask: VarArray<i64>,
            block_light_mask: VarArray<i64>,
            empty_sky_light_mask: VarArray<i64>,
            empty_block_light_mask: VarArray<i64>,
            sky_light: VarArray<VarArray<u8>>,
            block_light: VarArray<VarArray<u8>>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateLight {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.chunk_x, w)?;
                let w = VarInt::serialize(&self.chunk_z, w)?;
                let w = bool::serialize(&self.trust_edges, w)?;

                let w = PrefixedArray::<i64, VarInt>::len(&self.sky_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.sky_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<i64, VarInt>::len(&self.block_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.block_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<i64, VarInt>::len(&self.empty_sky_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.empty_sky_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<i64, VarInt>::len(&self.empty_block_light_mask).serialize(w)?;

                let mut w = w;
                let items = self.empty_block_light_mask.0.iter();
                for i in items {
                    w = i64::serialize(&i, w)?
                }

                let w = PrefixedArray::<VarArray<u8>, VarInt>::len(&self.sky_light).serialize(w)?;

                let mut w = w;
                let items = self.sky_light.0.iter();
                for i in items {
                    w = {
                        let w = PrefixedArray::<u8, VarInt>::len(&i).serialize(w)?;

                        let mut w = w;
                        let items = i.0.iter();
                        for i in items {
                            w = u8::serialize(&i, w)?
                        }
                        w
                    }
                }

                let w = PrefixedArray::<VarArray<u8>, VarInt>::len(&self.block_light).serialize(w)?;

                let mut w = w;
                let items = self.block_light.0.iter();
                for i in items {
                    w = {
                        let w = PrefixedArray::<u8, VarInt>::len(&i).serialize(w)?;

                        let mut w = w;
                        let items = i.0.iter();
                        for i in items {
                            w = u8::serialize(&i, w)?
                        }
                        w
                    }
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<i64, VarInt>::deserialize,
                        PrefixedArray::<VarArray<u8>, VarInt>::deserialize,
                        PrefixedArray::<VarArray<u8>, VarInt>::deserialize,
                    )),
                    |(chunk_x, chunk_z, trust_edges, sky_light_mask, block_light_mask, empty_sky_light_mask, empty_block_light_mask, sky_light, block_light)| PacketUpdateLight {
                        chunk_x,
                        chunk_z,
                        trust_edges,
                        sky_light_mask,
                        block_light_mask,
                        empty_sky_light_mask,
                        empty_block_light_mask,
                        sky_light,
                        block_light,
                    },
                ))(input)
            }
        }

        pub struct PacketLogin<'a> {
            entity_id: i32,
            is_hardcore: bool,
            game_mode: u8,
            previous_game_mode: i8,
            world_names: VarStringArray<'a>,
            dimension_codec: Nbt,
            dimension: Nbt,
            world_name: VarString<'a>,
            hashed_seed: i64,
            max_players: VarInt,
            view_distance: VarInt,
            simulation_distance: VarInt,
            reduced_debug_info: bool,
            enable_respawn_screen: bool,
            is_debug: bool,
            is_flat: bool,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketLogin<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.entity_id, w)?;
                let w = bool::serialize(&self.is_hardcore, w)?;
                let w = u8::serialize(&self.game_mode, w)?;
                let w = i8::serialize(&self.previous_game_mode, w)?;

                let w = PrefixedArray::<VarString<'a>, VarInt>::len(&self.world_names).serialize(w)?;

                let mut w = w;
                let items = self.world_names.0.iter();
                for i in items {
                    w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                }

                let w = Nbt::serialize(&self.dimension_codec, w)?;
                let w = Nbt::serialize(&self.dimension, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.world_name, w)?;
                let w = i64::serialize(&self.hashed_seed, w)?;
                let w = VarInt::serialize(&self.max_players, w)?;
                let w = VarInt::serialize(&self.view_distance, w)?;
                let w = VarInt::serialize(&self.simulation_distance, w)?;
                let w = bool::serialize(&self.reduced_debug_info, w)?;
                let w = bool::serialize(&self.enable_respawn_screen, w)?;
                let w = bool::serialize(&self.is_debug, w)?;
                let w = bool::serialize(&self.is_flat, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        i32::deserialize,
                        bool::deserialize,
                        u8::deserialize,
                        i8::deserialize,
                        PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                        Nbt::deserialize,
                        Nbt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        i64::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(
                        entity_id,
                        is_hardcore,
                        game_mode,
                        previous_game_mode,
                        world_names,
                        dimension_codec,
                        dimension,
                        world_name,
                        hashed_seed,
                        max_players,
                        view_distance,
                        simulation_distance,
                        reduced_debug_info,
                        enable_respawn_screen,
                        is_debug,
                        is_flat,
                    )| PacketLogin {
                        entity_id,
                        is_hardcore,
                        game_mode,
                        previous_game_mode,
                        world_names,
                        dimension_codec,
                        dimension,
                        world_name,
                        hashed_seed,
                        max_players,
                        view_distance,
                        simulation_distance,
                        reduced_debug_info,
                        enable_respawn_screen,
                        is_debug,
                        is_flat,
                    },
                ))(input)
            }
        }

        pub struct Ident11<'a> {
            r_type: VarInt,
            x: i8,
            z: i8,
            direction: u8,
            display_name: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Ident11<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.r_type, w)?;
                let w = i8::serialize(&self.x, w)?;
                let w = i8::serialize(&self.z, w)?;
                let w = u8::serialize(&self.direction, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.display_name, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, i8::deserialize, i8::deserialize, u8::deserialize, Option::<VarString<'a>>::deserialize)),
                    |(r_type, x, z, direction, display_name)| Ident11 {
                        r_type,
                        x,
                        z,
                        direction,
                        display_name,
                    },
                ))(input)
            }
        }

        pub enum Rows {
            Rows0,
            Default(u8),
        }

        impl Rows {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Rows::Rows0 => "0",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Rows::Rows0 => w,
                    Rows::Default(val) => u8::serialize(val, w)?,
                };

                Ok(w)
            }
        }
        pub enum MapX {
            MapX0,
            Default(u8),
        }

        impl MapX {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    MapX::MapX0 => "0",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    MapX::MapX0 => w,
                    MapX::Default(val) => u8::serialize(val, w)?,
                };

                Ok(w)
            }
        }
        pub enum MapY {
            MapY0,
            Default(u8),
        }

        impl MapY {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    MapY::MapY0 => "0",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    MapY::MapY0 => w,
                    MapY::Default(val) => u8::serialize(val, w)?,
                };

                Ok(w)
            }
        }
        pub enum MapData<'a> {
            MapData0,
            Default(VarBuffer<'a>),
        }

        impl<'a> MapData<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    MapData::MapData0 => "0",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    MapData::MapData0 => w,
                    MapData::Default(val) => PrefixedBuffer::<'a, VarInt>::serialize(val, w)?,
                };

                Ok(w)
            }
        }
        pub struct PacketMap<'a> {
            item_damage: VarInt,
            scale: i8,
            locked: bool,
            icons: Option<PrefixedArray<Ident11<'a>, VarInt>>,
            columns: u8,
            rows: Rows,
            x: MapX,
            y: MapY,
            data: MapData<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketMap<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.item_damage, w)?;
                let w = i8::serialize(&self.scale, w)?;
                let w = bool::serialize(&self.locked, w)?;
                let w = Option::<VarArray<Ident11>>::serialize(&self.icons, w)?;
                let w = u8::serialize(&self.columns, w)?;
                let w = Rows::serialize(&self.rows, w)?;
                let w = MapX::serialize(&self.x, w)?;
                let w = MapY::serialize(&self.y, w)?;
                let w = MapData::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_item_damage) = (VarInt::deserialize)(input)?;
                    let (input, self_scale) = (i8::deserialize)(input)?;
                    let (input, self_locked) = (bool::deserialize)(input)?;
                    let (input, self_icons) = (Option::<VarArray<Ident11>>::deserialize)(input)?;
                    let (input, self_columns) = (u8::deserialize)(input)?;
                    let (input, self_rows) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => Ok((input, Rows::Rows0)),
                        _ => map(u8::deserialize, Rows::Default)(input),
                    })(input)?;
                    let (input, self_x) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => Ok((input, MapX::MapX0)),
                        _ => map(u8::deserialize, MapX::Default)(input),
                    })(input)?;
                    let (input, self_y) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => Ok((input, MapY::MapY0)),
                        _ => map(u8::deserialize, MapY::Default)(input),
                    })(input)?;
                    let (input, self_data) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => Ok((input, MapData::MapData0)),
                        _ => map(PrefixedBuffer::<'a, VarInt>::deserialize, MapData::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        PacketMap {
                            item_damage: self_item_damage,
                            scale: self_scale,
                            locked: self_locked,
                            icons: self_icons,
                            columns: self_columns,
                            rows: self_rows,
                            x: self_x,
                            y: self_y,
                            data: self_data,
                        },
                    ))
                })(input)
            }
        }

        pub struct Trade {
            input_item1: Slot,
            output_item: Slot,
            input_item2: Option<Slot>,
            trade_disabled: bool,
            nb_trade_uses: i32,
            maximum_nb_trade_uses: i32,
            xp: i32,
            special_price: i32,
            price_multiplier: f32,
            demand: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for Trade {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Slot::serialize(&self.input_item1, w)?;
                let w = Slot::serialize(&self.output_item, w)?;
                let w = Option::<Slot>::serialize(&self.input_item2, w)?;
                let w = bool::serialize(&self.trade_disabled, w)?;
                let w = i32::serialize(&self.nb_trade_uses, w)?;
                let w = i32::serialize(&self.maximum_nb_trade_uses, w)?;
                let w = i32::serialize(&self.xp, w)?;
                let w = i32::serialize(&self.special_price, w)?;
                let w = f32::serialize(&self.price_multiplier, w)?;
                let w = i32::serialize(&self.demand, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        Slot::deserialize,
                        Slot::deserialize,
                        Option::<Slot>::deserialize,
                        bool::deserialize,
                        i32::deserialize,
                        i32::deserialize,
                        i32::deserialize,
                        i32::deserialize,
                        f32::deserialize,
                        i32::deserialize,
                    )),
                    |(input_item1, output_item, input_item2, trade_disabled, nb_trade_uses, maximum_nb_trade_uses, xp, special_price, price_multiplier, demand)| Trade {
                        input_item1,
                        output_item,
                        input_item2,
                        trade_disabled,
                        nb_trade_uses,
                        maximum_nb_trade_uses,
                        xp,
                        special_price,
                        price_multiplier,
                        demand,
                    },
                ))(input)
            }
        }

        pub struct PacketTradeList {
            window_id: VarInt,
            trades: PrefixedArray<Trade, u8>,
            villager_level: VarInt,
            experience: VarInt,
            is_regular_villager: bool,
            can_restock: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketTradeList {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.window_id, w)?;

                let w = PrefixedArray::<Trade, u8>::len(&self.trades).serialize(w)?;

                let mut w = w;
                let items = self.trades.0.iter();
                for i in items {
                    w = Trade::serialize(&i, w)?
                }

                let w = VarInt::serialize(&self.villager_level, w)?;
                let w = VarInt::serialize(&self.experience, w)?;
                let w = bool::serialize(&self.is_regular_villager, w)?;
                let w = bool::serialize(&self.can_restock, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        PrefixedArray::<Trade, u8>::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(window_id, trades, villager_level, experience, is_regular_villager, can_restock)| PacketTradeList {
                        window_id,
                        trades,
                        villager_level,
                        experience,
                        is_regular_villager,
                        can_restock,
                    },
                ))(input)
            }
        }

        pub struct PacketRelEntityMove {
            entity_id: VarInt,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketRelEntityMove {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i16::serialize(&self.d_x, w)?;
                let w = i16::serialize(&self.d_y, w)?;
                let w = i16::serialize(&self.d_z, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, i16::deserialize, i16::deserialize, i16::deserialize, bool::deserialize)),
                    |(entity_id, d_x, d_y, d_z, on_ground)| PacketRelEntityMove { entity_id, d_x, d_y, d_z, on_ground },
                ))(input)
            }
        }

        pub struct PacketEntityMoveLook {
            entity_id: VarInt,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityMoveLook {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i16::serialize(&self.d_x, w)?;
                let w = i16::serialize(&self.d_y, w)?;
                let w = i16::serialize(&self.d_z, w)?;
                let w = i8::serialize(&self.yaw, w)?;
                let w = i8::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        bool::deserialize,
                    )),
                    |(entity_id, d_x, d_y, d_z, yaw, pitch, on_ground)| PacketEntityMoveLook {
                        entity_id,
                        d_x,
                        d_y,
                        d_z,
                        yaw,
                        pitch,
                        on_ground,
                    },
                ))(input)
            }
        }

        pub struct PacketEntityLook {
            entity_id: VarInt,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityLook {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.yaw, w)?;
                let w = i8::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, i8::deserialize, i8::deserialize, bool::deserialize)),
                    |(entity_id, yaw, pitch, on_ground)| PacketEntityLook { entity_id, yaw, pitch, on_ground },
                ))(input)
            }
        }

        pub struct PacketVehicleMove {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketVehicleMove {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((f64::deserialize, f64::deserialize, f64::deserialize, f32::deserialize, f32::deserialize)),
                    |(x, y, z, yaw, pitch)| PacketVehicleMove { x, y, z, yaw, pitch },
                ))(input)
            }
        }

        pub struct PacketOpenBook {
            hand: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketOpenBook {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(hand,)| PacketOpenBook { hand }))(input)
            }
        }

        pub struct PacketOpenSignEntity {
            location: Position,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketOpenSignEntity {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Position::deserialize,)), |(location,)| PacketOpenSignEntity { location }))(input)
            }
        }

        pub struct PacketCraftRecipeResponse<'a> {
            window_id: i8,
            recipe: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCraftRecipeResponse<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i8::deserialize, PrefixedString::<'a, VarInt>::deserialize)), |(window_id, recipe)| PacketCraftRecipeResponse {
                    window_id,
                    recipe,
                }))(input)
            }
        }

        pub struct PacketAbilities {
            flags: i8,
            flying_speed: f32,
            walking_speed: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAbilities {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.flags, w)?;
                let w = f32::serialize(&self.flying_speed, w)?;
                let w = f32::serialize(&self.walking_speed, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i8::deserialize, f32::deserialize, f32::deserialize)), |(flags, flying_speed, walking_speed)| PacketAbilities {
                    flags,
                    flying_speed,
                    walking_speed,
                }))(input)
            }
        }

        pub struct PacketEndCombatEvent {
            duration: VarInt,
            entity_id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEndCombatEvent {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.duration, w)?;
                let w = i32::serialize(&self.entity_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, i32::deserialize)), |(duration, entity_id)| PacketEndCombatEvent { duration, entity_id }))(input)
            }
        }

        pub struct PacketEnterCombatEvent {}

        impl<'t> protocol_lib::Packet<'t> for PacketEnterCombatEvent {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((|i| Ok((i, ())),)), |_| PacketEnterCombatEvent {}))(input)
            }
        }

        pub struct PacketDeathCombatEvent<'a> {
            player_id: VarInt,
            entity_id: i32,
            message: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDeathCombatEvent<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.player_id, w)?;
                let w = i32::serialize(&self.entity_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.message, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, i32::deserialize, PrefixedString::<'a, VarInt>::deserialize)),
                    |(player_id, entity_id, message)| PacketDeathCombatEvent { player_id, entity_id, message },
                ))(input)
            }
        }

        pub enum PlayerInfoDataItemName<'a> {
            PlayerInfoDataItemName0(VarString<'a>),
            Default,
        }

        impl<'a> PlayerInfoDataItemName<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    PlayerInfoDataItemName::PlayerInfoDataItemName0(_) => "0",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    PlayerInfoDataItemName::PlayerInfoDataItemName0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    PlayerInfoDataItemName::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PlayerInfoDataItemProperties0<'a> {
            name: VarString<'a>,
            value: VarString<'a>,
            signature: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PlayerInfoDataItemProperties0<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.value, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.signature, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        Option::<VarString<'a>>::deserialize,
                    )),
                    |(name, value, signature)| PlayerInfoDataItemProperties0 { name, value, signature },
                ))(input)
            }
        }

        pub enum PlayerInfoDataItemProperties<'a> {
            PlayerInfoDataItemProperties0(PrefixedArray<PlayerInfoDataItemProperties0<'a>, VarInt>),
            Default,
        }

        impl<'a> PlayerInfoDataItemProperties<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    PlayerInfoDataItemProperties::PlayerInfoDataItemProperties0(_) => "0",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    PlayerInfoDataItemProperties::PlayerInfoDataItemProperties0(val) => {
                        let w = PrefixedArray::<PlayerInfoDataItemProperties0, VarInt>::len(&val).serialize(w)?;

                        let mut w = w;
                        let items = val.0.iter();
                        for i in items {
                            w = PlayerInfoDataItemProperties0::serialize(&i, w)?
                        }
                        w
                    }
                    PlayerInfoDataItemProperties::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Gamemode {
            Gamemode0(VarInt),
            Gamemode1(VarInt),
            Default,
        }

        impl Gamemode {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Gamemode::Gamemode0(_) => "0",
                    Gamemode::Gamemode1(_) => "1",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Gamemode::Gamemode0(val) => VarInt::serialize(&val, w)?,
                    Gamemode::Gamemode1(val) => VarInt::serialize(&val, w)?,
                    Gamemode::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Ping {
            Ping0(VarInt),
            Ping2(VarInt),
            Default,
        }

        impl Ping {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Ping::Ping0(_) => "0",
                    Ping::Ping2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Ping::Ping0(val) => VarInt::serialize(&val, w)?,
                    Ping::Ping2(val) => VarInt::serialize(&val, w)?,
                    Ping::Default => w,
                };

                Ok(w)
            }
        }
        pub enum PlayerInfoDataItemDisplayName<'a> {
            PlayerInfoDataItemDisplayName0(Option<VarString<'a>>),
            PlayerInfoDataItemDisplayName3(Option<VarString<'a>>),
            Default,
        }

        impl<'a> PlayerInfoDataItemDisplayName<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName0(_) => "0",
                    PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName3(_) => "3",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName0(val) => Option::<VarString<'a>>::serialize(&val, w)?,
                    PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName3(val) => Option::<VarString<'a>>::serialize(&val, w)?,
                    PlayerInfoDataItemDisplayName::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PlayerInfoDataItem<'a> {
            uuid: Uuid,
            name: PlayerInfoDataItemName<'a>,
            properties: PlayerInfoDataItemProperties<'a>,
            gamemode: Gamemode,
            ping: Ping,
            display_name: PlayerInfoDataItemDisplayName<'a>,
        }
        pub struct PacketPlayerInfo<'a> {
            action: VarInt,
            data: VarArray<PlayerInfoDataItem<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketPlayerInfo<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action, w)?;

                let w = PrefixedArray::<PlayerInfoDataItem, VarInt>::len(&self.data).serialize(w)?;

                let mut w = w;
                let items = self.data.0.iter();
                for i in items {
                    w = {
                        let w = Uuid::serialize(&i.uuid, w)?;
                        let w = PlayerInfoDataItemName::serialize(&i.name, w)?;
                        let w = PlayerInfoDataItemProperties::serialize(&i.properties, w)?;
                        let w = Gamemode::serialize(&i.gamemode, w)?;
                        let w = Ping::serialize(&i.ping, w)?;
                        let w = PlayerInfoDataItemDisplayName::serialize(&i.display_name, w)?;
                        w
                    }
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_data) = (|input| {
                        let (input, len) = VarInt::deserialize(input)?;
                        let len = protocol_lib::types::num_traits::ToPrimitive::to_usize(&len).ok_or(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::TooLarge)))?;
                        map(
                            nom::multi::count(
                                |input| {
                                    let (input, self_data_uuid) = (Uuid::deserialize)(input)?;
                                    let (input, self_data_name) = (|input| match &format!("{}", self_action)[..] {
                                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, PlayerInfoDataItemName::PlayerInfoDataItemName0)(input),
                                        _ => Ok((input, PlayerInfoDataItemName::Default)),
                                    })(input)?;
                                    let (input, self_data_properties) = (|input| match &format!("{}", self_action)[..] {
                                        "0" => map(
                                            PrefixedArray::<PlayerInfoDataItemProperties0<'a>, VarInt>::deserialize,
                                            PlayerInfoDataItemProperties::PlayerInfoDataItemProperties0,
                                        )(input),
                                        _ => Ok((input, PlayerInfoDataItemProperties::Default)),
                                    })(input)?;
                                    let (input, self_data_gamemode) = (|input| match &format!("{}", self_action)[..] {
                                        "0" => map(VarInt::deserialize, Gamemode::Gamemode0)(input),
                                        "1" => map(VarInt::deserialize, Gamemode::Gamemode1)(input),
                                        _ => Ok((input, Gamemode::Default)),
                                    })(input)?;
                                    let (input, self_data_ping) = (|input| match &format!("{}", self_action)[..] {
                                        "0" => map(VarInt::deserialize, Ping::Ping0)(input),
                                        "2" => map(VarInt::deserialize, Ping::Ping2)(input),
                                        _ => Ok((input, Ping::Default)),
                                    })(input)?;
                                    let (input, self_data_display_name) = (|input| match &format!("{}", self_action)[..] {
                                        "0" => map(Option::<VarString<'a>>::deserialize, PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName0)(input),
                                        "3" => map(Option::<VarString<'a>>::deserialize, PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName3)(input),
                                        _ => Ok((input, PlayerInfoDataItemDisplayName::Default)),
                                    })(input)?;
                                    Ok((
                                        input,
                                        PlayerInfoDataItem {
                                            uuid: self_data_uuid,
                                            name: self_data_name,
                                            properties: self_data_properties,
                                            gamemode: self_data_gamemode,
                                            ping: self_data_ping,
                                            display_name: self_data_display_name,
                                        },
                                    ))
                                },
                                len,
                            ),
                            |x| PrefixedArray::<PlayerInfoDataItem, VarInt>(x, core::marker::PhantomData),
                        )(input)
                    })(input)?;
                    Ok((input, PacketPlayerInfo { action: self_action, data: self_data }))
                })(input)
            }
        }

        pub struct PacketPosition {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            flags: i8,
            teleport_id: VarInt,
            dismount_vehicle: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPosition {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;
                let w = i8::serialize(&self.flags, w)?;
                let w = VarInt::serialize(&self.teleport_id, w)?;
                let w = bool::serialize(&self.dismount_vehicle, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        i8::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                    )),
                    |(x, y, z, yaw, pitch, flags, teleport_id, dismount_vehicle)| PacketPosition {
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                        flags,
                        teleport_id,
                        dismount_vehicle,
                    },
                ))(input)
            }
        }

        pub enum Recipes2<'a> {
            Recipes20(VarStringArray<'a>),
            Default,
        }

        impl<'a> Recipes2<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Recipes2::Recipes20(_) => "0",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Recipes2::Recipes20(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::len(&val).serialize(w)?;

                        let mut w = w;
                        let items = val.0.iter();
                        for i in items {
                            w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                        }
                        w
                    }
                    Recipes2::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketUnlockRecipes<'a> {
            action: VarInt,
            crafting_book_open: bool,
            filtering_craftable: bool,
            smelting_book_open: bool,
            filtering_smeltable: bool,
            blast_furnace_open: bool,
            filtering_blast_furnace: bool,
            smoker_book_open: bool,
            filtering_smoker: bool,
            recipes1: VarStringArray<'a>,
            recipes2: Recipes2<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUnlockRecipes<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action, w)?;
                let w = bool::serialize(&self.crafting_book_open, w)?;
                let w = bool::serialize(&self.filtering_craftable, w)?;
                let w = bool::serialize(&self.smelting_book_open, w)?;
                let w = bool::serialize(&self.filtering_smeltable, w)?;
                let w = bool::serialize(&self.blast_furnace_open, w)?;
                let w = bool::serialize(&self.filtering_blast_furnace, w)?;
                let w = bool::serialize(&self.smoker_book_open, w)?;
                let w = bool::serialize(&self.filtering_smoker, w)?;

                let w = PrefixedArray::<VarString<'a>, VarInt>::len(&self.recipes1).serialize(w)?;

                let mut w = w;
                let items = self.recipes1.0.iter();
                for i in items {
                    w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                }

                let w = Recipes2::serialize(&self.recipes2, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_crafting_book_open) = (bool::deserialize)(input)?;
                    let (input, self_filtering_craftable) = (bool::deserialize)(input)?;
                    let (input, self_smelting_book_open) = (bool::deserialize)(input)?;
                    let (input, self_filtering_smeltable) = (bool::deserialize)(input)?;
                    let (input, self_blast_furnace_open) = (bool::deserialize)(input)?;
                    let (input, self_filtering_blast_furnace) = (bool::deserialize)(input)?;
                    let (input, self_smoker_book_open) = (bool::deserialize)(input)?;
                    let (input, self_filtering_smoker) = (bool::deserialize)(input)?;
                    let (input, self_recipes1) = (PrefixedArray::<VarString<'a>, VarInt>::deserialize)(input)?;
                    let (input, self_recipes2) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(PrefixedArray::<VarString<'a>, VarInt>::deserialize, Recipes2::Recipes20)(input),
                        _ => Ok((input, Recipes2::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketUnlockRecipes {
                            action: self_action,
                            crafting_book_open: self_crafting_book_open,
                            filtering_craftable: self_filtering_craftable,
                            smelting_book_open: self_smelting_book_open,
                            filtering_smeltable: self_filtering_smeltable,
                            blast_furnace_open: self_blast_furnace_open,
                            filtering_blast_furnace: self_filtering_blast_furnace,
                            smoker_book_open: self_smoker_book_open,
                            filtering_smoker: self_filtering_smoker,
                            recipes1: self_recipes1,
                            recipes2: self_recipes2,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketEntityDestroy {
            entity_ids: VarArray<VarInt>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityDestroy {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<VarInt, VarInt>::len(&self.entity_ids).serialize(w)?;

                let mut w = w;
                let items = self.entity_ids.0.iter();
                for i in items {
                    w = VarInt::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedArray::<VarInt, VarInt>::deserialize,)), |(entity_ids,)| PacketEntityDestroy { entity_ids }))(input)
            }
        }

        pub struct PacketRemoveEntityEffect {
            entity_id: VarInt,
            effect_id: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketRemoveEntityEffect {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.effect_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, i8::deserialize)), |(entity_id, effect_id)| PacketRemoveEntityEffect {
                    entity_id,
                    effect_id,
                }))(input)
            }
        }

        pub struct PacketResourcePackSend<'a> {
            url: VarString<'a>,
            hash: VarString<'a>,
            forced: bool,
            prompt_message: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketResourcePackSend<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.url, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.hash, w)?;
                let w = bool::serialize(&self.forced, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.prompt_message, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        bool::deserialize,
                        Option::<VarString<'a>>::deserialize,
                    )),
                    |(url, hash, forced, prompt_message)| PacketResourcePackSend { url, hash, forced, prompt_message },
                ))(input)
            }
        }

        pub struct PacketRespawn<'a> {
            dimension: Nbt,
            world_name: VarString<'a>,
            hashed_seed: i64,
            gamemode: u8,
            previous_gamemode: u8,
            is_debug: bool,
            is_flat: bool,
            copy_metadata: bool,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketRespawn<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Nbt::serialize(&self.dimension, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.world_name, w)?;
                let w = i64::serialize(&self.hashed_seed, w)?;
                let w = u8::serialize(&self.gamemode, w)?;
                let w = u8::serialize(&self.previous_gamemode, w)?;
                let w = bool::serialize(&self.is_debug, w)?;
                let w = bool::serialize(&self.is_flat, w)?;
                let w = bool::serialize(&self.copy_metadata, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        Nbt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        i64::deserialize,
                        u8::deserialize,
                        u8::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(dimension, world_name, hashed_seed, gamemode, previous_gamemode, is_debug, is_flat, copy_metadata)| PacketRespawn {
                        dimension,
                        world_name,
                        hashed_seed,
                        gamemode,
                        previous_gamemode,
                        is_debug,
                        is_flat,
                        copy_metadata,
                    },
                ))(input)
            }
        }

        pub struct PacketEntityHeadRotation {
            entity_id: VarInt,
            head_yaw: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityHeadRotation {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.head_yaw, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, i8::deserialize)), |(entity_id, head_yaw)| PacketEntityHeadRotation { entity_id, head_yaw }))(input)
            }
        }

        pub struct PacketCamera {
            camera_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCamera {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.camera_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(camera_id,)| PacketCamera { camera_id }))(input)
            }
        }

        pub struct PacketHeldItemSlot {
            slot: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketHeldItemSlot {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.slot, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i8::deserialize,)), |(slot,)| PacketHeldItemSlot { slot }))(input)
            }
        }

        pub struct PacketUpdateViewPosition {
            chunk_x: VarInt,
            chunk_z: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateViewPosition {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.chunk_x, w)?;
                let w = VarInt::serialize(&self.chunk_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, VarInt::deserialize)), |(chunk_x, chunk_z)| PacketUpdateViewPosition { chunk_x, chunk_z }))(input)
            }
        }

        pub struct PacketUpdateViewDistance {
            view_distance: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateViewDistance {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.view_distance, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(view_distance,)| PacketUpdateViewDistance { view_distance }))(input)
            }
        }

        pub struct PacketScoreboardDisplayObjective<'a> {
            position: i8,
            name: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketScoreboardDisplayObjective<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.position, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i8::deserialize, PrefixedString::<'a, VarInt>::deserialize)), |(position, name)| {
                    PacketScoreboardDisplayObjective { position, name }
                }))(input)
            }
        }

        pub struct PacketEntityMetadata<'a> {
            entity_id: VarInt,
            metadata: Vec<EntityMetadata<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEntityMetadata<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;

                let mut w = w;
                for (index, item) in self.metadata.iter().enumerate() {
                    w = u8::serialize(&if index == self.metadata.len() - 1 { 255 } else { index as u8 }, w)?;
                    w = str::parse::<VarInt>(item.discriminant()).unwrap().serialize(w)?;
                    w = EntityMetadata::serialize(&item, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, |mut input| {
                        let mut accum = vec![];
                        loop {
                            let (i, item) = EntityMetadataWrapper::deserialize(input)?;
                            input = i;
                            let index = item.key;
                            accum.push(item.value);
                            if index == 0xFF {
                                break;
                            }
                        }
                        Ok((input, accum))
                    })),
                    |(entity_id, metadata)| PacketEntityMetadata { entity_id, metadata },
                ))(input)
            }
        }

        pub struct PacketAttachEntity {
            entity_id: i32,
            vehicle_id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAttachEntity {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.entity_id, w)?;
                let w = i32::serialize(&self.vehicle_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i32::deserialize, i32::deserialize)), |(entity_id, vehicle_id)| PacketAttachEntity { entity_id, vehicle_id }))(input)
            }
        }

        pub struct PacketEntityVelocity {
            entity_id: VarInt,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityVelocity {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i16::serialize(&self.velocity_x, w)?;
                let w = i16::serialize(&self.velocity_y, w)?;
                let w = i16::serialize(&self.velocity_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, i16::deserialize, i16::deserialize, i16::deserialize)),
                    |(entity_id, velocity_x, velocity_y, velocity_z)| PacketEntityVelocity {
                        entity_id,
                        velocity_x,
                        velocity_y,
                        velocity_z,
                    },
                ))(input)
            }
        }

        pub struct PacketEntityEquipment {
            entity_id: VarInt,
            equipments: std::collections::HashMap<i8, Slot>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityEquipment {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;

                let mut w = w;
                for (i, (k, v)) in self.equipments.iter().enumerate() {
                    let k = if i == self.equipments.len() - 1 { *k | (1i8 << 7) } else { *k };
                    let ww = i8::serialize(&k, w)?;
                    w = v.serialize(ww)?;
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, |mut input| {
                        let mut val = std::collections::HashMap::new();
                        loop {
                            let (i, (k_, v)) = tuple((i8::deserialize, Slot::deserialize))(input)?;
                            input = i;
                            let k = k_ & 0x7F;
                            val.insert(k, v);
                            if k != k_ {
                                break;
                            }
                        }
                        Ok((input, val))
                    })),
                    |(entity_id, equipments)| PacketEntityEquipment { entity_id, equipments },
                ))(input)
            }
        }

        pub struct PacketExperience {
            experience_bar: f32,
            level: VarInt,
            total_experience: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketExperience {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.experience_bar, w)?;
                let w = VarInt::serialize(&self.level, w)?;
                let w = VarInt::serialize(&self.total_experience, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f32::deserialize, VarInt::deserialize, VarInt::deserialize)), |(experience_bar, level, total_experience)| {
                    PacketExperience {
                        experience_bar,
                        level,
                        total_experience,
                    }
                }))(input)
            }
        }

        pub struct PacketUpdateHealth {
            health: f32,
            food: VarInt,
            food_saturation: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateHealth {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.health, w)?;
                let w = VarInt::serialize(&self.food, w)?;
                let w = f32::serialize(&self.food_saturation, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f32::deserialize, VarInt::deserialize, f32::deserialize)), |(health, food, food_saturation)| PacketUpdateHealth {
                    health,
                    food,
                    food_saturation,
                }))(input)
            }
        }

        pub enum DisplayText<'a> {
            DisplayText0(VarString<'a>),
            DisplayText2(VarString<'a>),
            Default,
        }

        impl<'a> DisplayText<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    DisplayText::DisplayText0(_) => "0",
                    DisplayText::DisplayText2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    DisplayText::DisplayText0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    DisplayText::DisplayText2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    DisplayText::Default => w,
                };

                Ok(w)
            }
        }
        pub enum ScoreboardObjectiveType {
            ScoreboardObjectiveType0(VarInt),
            ScoreboardObjectiveType2(VarInt),
            Default,
        }

        impl ScoreboardObjectiveType {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    ScoreboardObjectiveType::ScoreboardObjectiveType0(_) => "0",
                    ScoreboardObjectiveType::ScoreboardObjectiveType2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    ScoreboardObjectiveType::ScoreboardObjectiveType0(val) => VarInt::serialize(&val, w)?,
                    ScoreboardObjectiveType::ScoreboardObjectiveType2(val) => VarInt::serialize(&val, w)?,
                    ScoreboardObjectiveType::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketScoreboardObjective<'a> {
            name: VarString<'a>,
            action: i8,
            display_text: DisplayText<'a>,
            r_type: ScoreboardObjectiveType,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketScoreboardObjective<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = i8::serialize(&self.action, w)?;
                let w = DisplayText::serialize(&self.display_text, w)?;
                let w = ScoreboardObjectiveType::serialize(&self.r_type, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_action) = (i8::deserialize)(input)?;
                    let (input, self_display_text) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, DisplayText::DisplayText0)(input),
                        "2" => map(PrefixedString::<'a, VarInt>::deserialize, DisplayText::DisplayText2)(input),
                        _ => Ok((input, DisplayText::Default)),
                    })(input)?;
                    let (input, self_r_type) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(VarInt::deserialize, ScoreboardObjectiveType::ScoreboardObjectiveType0)(input),
                        "2" => map(VarInt::deserialize, ScoreboardObjectiveType::ScoreboardObjectiveType2)(input),
                        _ => Ok((input, ScoreboardObjectiveType::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketScoreboardObjective {
                            name: self_name,
                            action: self_action,
                            display_text: self_display_text,
                            r_type: self_r_type,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketSetPassengers {
            entity_id: VarInt,
            passengers: VarArray<VarInt>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetPassengers {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;

                let w = PrefixedArray::<VarInt, VarInt>::len(&self.passengers).serialize(w)?;

                let mut w = w;
                let items = self.passengers.0.iter();
                for i in items {
                    w = VarInt::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, PrefixedArray::<VarInt, VarInt>::deserialize)), |(entity_id, passengers)| {
                    PacketSetPassengers { entity_id, passengers }
                }))(input)
            }
        }

        pub enum TeamsName<'a> {
            TeamsName0(VarString<'a>),
            TeamsName2(VarString<'a>),
            Default,
        }

        impl<'a> TeamsName<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    TeamsName::TeamsName0(_) => "0",
                    TeamsName::TeamsName2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    TeamsName::TeamsName0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    TeamsName::TeamsName2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    TeamsName::Default => w,
                };

                Ok(w)
            }
        }
        pub enum FriendlyFire {
            FriendlyFire0(i8),
            FriendlyFire2(i8),
            Default,
        }

        impl FriendlyFire {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    FriendlyFire::FriendlyFire0(_) => "0",
                    FriendlyFire::FriendlyFire2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    FriendlyFire::FriendlyFire0(val) => i8::serialize(&val, w)?,
                    FriendlyFire::FriendlyFire2(val) => i8::serialize(&val, w)?,
                    FriendlyFire::Default => w,
                };

                Ok(w)
            }
        }
        pub enum NameTagVisibility<'a> {
            NameTagVisibility0(VarString<'a>),
            NameTagVisibility2(VarString<'a>),
            Default,
        }

        impl<'a> NameTagVisibility<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    NameTagVisibility::NameTagVisibility0(_) => "0",
                    NameTagVisibility::NameTagVisibility2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    NameTagVisibility::NameTagVisibility0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    NameTagVisibility::NameTagVisibility2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    NameTagVisibility::Default => w,
                };

                Ok(w)
            }
        }
        pub enum CollisionRule<'a> {
            CollisionRule0(VarString<'a>),
            CollisionRule2(VarString<'a>),
            Default,
        }

        impl<'a> CollisionRule<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    CollisionRule::CollisionRule0(_) => "0",
                    CollisionRule::CollisionRule2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    CollisionRule::CollisionRule0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    CollisionRule::CollisionRule2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    CollisionRule::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Formatting {
            Formatting0(VarInt),
            Formatting2(VarInt),
            Default,
        }

        impl Formatting {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Formatting::Formatting0(_) => "0",
                    Formatting::Formatting2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Formatting::Formatting0(val) => VarInt::serialize(&val, w)?,
                    Formatting::Formatting2(val) => VarInt::serialize(&val, w)?,
                    Formatting::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Prefix<'a> {
            Prefix0(VarString<'a>),
            Prefix2(VarString<'a>),
            Default,
        }

        impl<'a> Prefix<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Prefix::Prefix0(_) => "0",
                    Prefix::Prefix2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Prefix::Prefix0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Prefix::Prefix2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Prefix::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Suffix<'a> {
            Suffix0(VarString<'a>),
            Suffix2(VarString<'a>),
            Default,
        }

        impl<'a> Suffix<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Suffix::Suffix0(_) => "0",
                    Suffix::Suffix2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Suffix::Suffix0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Suffix::Suffix2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Suffix::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Players<'a> {
            Players0(VarStringArray<'a>),
            Players3(VarStringArray<'a>),
            Players4(VarStringArray<'a>),
            Default,
        }

        impl<'a> Players<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Players::Players0(_) => "0",
                    Players::Players3(_) => "3",
                    Players::Players4(_) => "4",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Players::Players0(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::len(&val).serialize(w)?;

                        let mut w = w;
                        let items = val.0.iter();
                        for i in items {
                            w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                        }
                        w
                    }
                    Players::Players3(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::len(&val).serialize(w)?;

                        let mut w = w;
                        let items = val.0.iter();
                        for i in items {
                            w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                        }
                        w
                    }
                    Players::Players4(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::len(&val).serialize(w)?;

                        let mut w = w;
                        let items = val.0.iter();
                        for i in items {
                            w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                        }
                        w
                    }
                    Players::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketTeams<'a> {
            team: VarString<'a>,
            mode: i8,
            name: TeamsName<'a>,
            friendly_fire: FriendlyFire,
            name_tag_visibility: NameTagVisibility<'a>,
            collision_rule: CollisionRule<'a>,
            formatting: Formatting,
            prefix: Prefix<'a>,
            suffix: Suffix<'a>,
            players: Players<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketTeams<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.team, w)?;
                let w = i8::serialize(&self.mode, w)?;
                let w = TeamsName::serialize(&self.name, w)?;
                let w = FriendlyFire::serialize(&self.friendly_fire, w)?;
                let w = NameTagVisibility::serialize(&self.name_tag_visibility, w)?;
                let w = CollisionRule::serialize(&self.collision_rule, w)?;
                let w = Formatting::serialize(&self.formatting, w)?;
                let w = Prefix::serialize(&self.prefix, w)?;
                let w = Suffix::serialize(&self.suffix, w)?;
                let w = Players::serialize(&self.players, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_team) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_mode) = (i8::deserialize)(input)?;
                    let (input, self_name) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, TeamsName::TeamsName0)(input),
                        "2" => map(PrefixedString::<'a, VarInt>::deserialize, TeamsName::TeamsName2)(input),
                        _ => Ok((input, TeamsName::Default)),
                    })(input)?;
                    let (input, self_friendly_fire) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(i8::deserialize, FriendlyFire::FriendlyFire0)(input),
                        "2" => map(i8::deserialize, FriendlyFire::FriendlyFire2)(input),
                        _ => Ok((input, FriendlyFire::Default)),
                    })(input)?;
                    let (input, self_name_tag_visibility) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, NameTagVisibility::NameTagVisibility0)(input),
                        "2" => map(PrefixedString::<'a, VarInt>::deserialize, NameTagVisibility::NameTagVisibility2)(input),
                        _ => Ok((input, NameTagVisibility::Default)),
                    })(input)?;
                    let (input, self_collision_rule) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, CollisionRule::CollisionRule0)(input),
                        "2" => map(PrefixedString::<'a, VarInt>::deserialize, CollisionRule::CollisionRule2)(input),
                        _ => Ok((input, CollisionRule::Default)),
                    })(input)?;
                    let (input, self_formatting) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(VarInt::deserialize, Formatting::Formatting0)(input),
                        "2" => map(VarInt::deserialize, Formatting::Formatting2)(input),
                        _ => Ok((input, Formatting::Default)),
                    })(input)?;
                    let (input, self_prefix) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, Prefix::Prefix0)(input),
                        "2" => map(PrefixedString::<'a, VarInt>::deserialize, Prefix::Prefix2)(input),
                        _ => Ok((input, Prefix::Default)),
                    })(input)?;
                    let (input, self_suffix) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, Suffix::Suffix0)(input),
                        "2" => map(PrefixedString::<'a, VarInt>::deserialize, Suffix::Suffix2)(input),
                        _ => Ok((input, Suffix::Default)),
                    })(input)?;
                    let (input, self_players) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => map(PrefixedArray::<VarString<'a>, VarInt>::deserialize, Players::Players0)(input),
                        "3" => map(PrefixedArray::<VarString<'a>, VarInt>::deserialize, Players::Players3)(input),
                        "4" => map(PrefixedArray::<VarString<'a>, VarInt>::deserialize, Players::Players4)(input),
                        _ => Ok((input, Players::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketTeams {
                            team: self_team,
                            mode: self_mode,
                            name: self_name,
                            friendly_fire: self_friendly_fire,
                            name_tag_visibility: self_name_tag_visibility,
                            collision_rule: self_collision_rule,
                            formatting: self_formatting,
                            prefix: self_prefix,
                            suffix: self_suffix,
                            players: self_players,
                        },
                    ))
                })(input)
            }
        }

        pub enum ScoreboardScoreValue {
            ScoreboardScoreValue1,
            Default(VarInt),
        }

        impl ScoreboardScoreValue {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    ScoreboardScoreValue::ScoreboardScoreValue1 => "1",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    ScoreboardScoreValue::ScoreboardScoreValue1 => w,
                    ScoreboardScoreValue::Default(val) => VarInt::serialize(val, w)?,
                };

                Ok(w)
            }
        }
        pub struct PacketScoreboardScore<'a> {
            item_name: VarString<'a>,
            action: VarInt,
            score_name: VarString<'a>,
            value: ScoreboardScoreValue,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketScoreboardScore<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.item_name, w)?;
                let w = VarInt::serialize(&self.action, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.score_name, w)?;
                let w = ScoreboardScoreValue::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_item_name) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_score_name) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_value) = (|input| match &format!("{}", self_action)[..] {
                        "1" => Ok((input, ScoreboardScoreValue::ScoreboardScoreValue1)),
                        _ => map(VarInt::deserialize, ScoreboardScoreValue::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        PacketScoreboardScore {
                            item_name: self_item_name,
                            action: self_action,
                            score_name: self_score_name,
                            value: self_value,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketSpawnPosition {
            location: Position,
            angle: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSpawnPosition {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = f32::serialize(&self.angle, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Position::deserialize, f32::deserialize)), |(location, angle)| PacketSpawnPosition { location, angle }))(input)
            }
        }

        pub struct PacketUpdateTime {
            age: i64,
            time: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateTime {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.age, w)?;
                let w = i64::serialize(&self.time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i64::deserialize, i64::deserialize)), |(age, time)| PacketUpdateTime { age, time }))(input)
            }
        }

        pub struct PacketEntitySoundEffect {
            sound_id: VarInt,
            sound_category: VarInt,
            entity_id: VarInt,
            volume: f32,
            pitch: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntitySoundEffect {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.sound_id, w)?;
                let w = VarInt::serialize(&self.sound_category, w)?;
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = f32::serialize(&self.volume, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, VarInt::deserialize, VarInt::deserialize, f32::deserialize, f32::deserialize)),
                    |(sound_id, sound_category, entity_id, volume, pitch)| PacketEntitySoundEffect {
                        sound_id,
                        sound_category,
                        entity_id,
                        volume,
                        pitch,
                    },
                ))(input)
            }
        }

        pub enum Source {
            Source3(VarInt),
            Source1(VarInt),
            Default,
        }

        impl Source {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Source::Source3(_) => "3",
                    Source::Source1(_) => "1",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Source::Source3(val) => VarInt::serialize(&val, w)?,
                    Source::Source1(val) => VarInt::serialize(&val, w)?,
                    Source::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Sound<'a> {
            Sound3(VarString<'a>),
            Sound2(VarString<'a>),
            Default,
        }

        impl<'a> Sound<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Sound::Sound3(_) => "3",
                    Sound::Sound2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Sound::Sound3(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Sound::Sound2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Sound::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketStopSound<'a> {
            flags: i8,
            source: Source,
            sound: Sound<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketStopSound<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.flags, w)?;
                let w = Source::serialize(&self.source, w)?;
                let w = Sound::serialize(&self.sound, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_flags) = (i8::deserialize)(input)?;
                    let (input, self_source) = (|input| match &format!("{}", self_flags)[..] {
                        "3" => map(VarInt::deserialize, Source::Source3)(input),
                        "1" => map(VarInt::deserialize, Source::Source1)(input),
                        _ => Ok((input, Source::Default)),
                    })(input)?;
                    let (input, self_sound) = (|input| match &format!("{}", self_flags)[..] {
                        "3" => map(PrefixedString::<'a, VarInt>::deserialize, Sound::Sound3)(input),
                        "2" => map(PrefixedString::<'a, VarInt>::deserialize, Sound::Sound2)(input),
                        _ => Ok((input, Sound::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketStopSound {
                            flags: self_flags,
                            source: self_source,
                            sound: self_sound,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketSoundEffect {
            sound_id: VarInt,
            sound_category: VarInt,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSoundEffect {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.sound_id, w)?;
                let w = VarInt::serialize(&self.sound_category, w)?;
                let w = i32::serialize(&self.x, w)?;
                let w = i32::serialize(&self.y, w)?;
                let w = i32::serialize(&self.z, w)?;
                let w = f32::serialize(&self.volume, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        i32::deserialize,
                        i32::deserialize,
                        i32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(sound_id, sound_category, x, y, z, volume, pitch)| PacketSoundEffect {
                        sound_id,
                        sound_category,
                        x,
                        y,
                        z,
                        volume,
                        pitch,
                    },
                ))(input)
            }
        }

        pub struct PacketPlayerlistHeader<'a> {
            header: VarString<'a>,
            footer: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketPlayerlistHeader<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.header, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.footer, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, PrefixedString::<'a, VarInt>::deserialize)), |(header, footer)| {
                    PacketPlayerlistHeader { header, footer }
                }))(input)
            }
        }

        pub struct PacketCollect {
            collected_entity_id: VarInt,
            collector_entity_id: VarInt,
            pickup_item_count: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCollect {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.collected_entity_id, w)?;
                let w = VarInt::serialize(&self.collector_entity_id, w)?;
                let w = VarInt::serialize(&self.pickup_item_count, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, VarInt::deserialize, VarInt::deserialize)),
                    |(collected_entity_id, collector_entity_id, pickup_item_count)| PacketCollect {
                        collected_entity_id,
                        collector_entity_id,
                        pickup_item_count,
                    },
                ))(input)
            }
        }

        pub struct PacketEntityTeleport {
            entity_id: VarInt,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityTeleport {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = i8::serialize(&self.yaw, w)?;
                let w = i8::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        bool::deserialize,
                    )),
                    |(entity_id, x, y, z, yaw, pitch, on_ground)| PacketEntityTeleport {
                        entity_id,
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                        on_ground,
                    },
                ))(input)
            }
        }

        pub struct Modifier {
            uuid: Uuid,
            amount: f64,
            operation: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for Modifier {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.uuid, w)?;
                let w = f64::serialize(&self.amount, w)?;
                let w = i8::serialize(&self.operation, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Uuid::deserialize, f64::deserialize, i8::deserialize)), |(uuid, amount, operation)| Modifier {
                    uuid,
                    amount,
                    operation,
                }))(input)
            }
        }

        pub struct EntityUpdateAttrsProperty<'a> {
            key: VarString<'a>,
            value: f64,
            modifiers: VarArray<Modifier>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for EntityUpdateAttrsProperty<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;
                let w = f64::serialize(&self.value, w)?;

                let w = PrefixedArray::<Modifier, VarInt>::len(&self.modifiers).serialize(w)?;

                let mut w = w;
                let items = self.modifiers.0.iter();
                for i in items {
                    w = Modifier::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((PrefixedString::<'a, VarInt>::deserialize, f64::deserialize, PrefixedArray::<Modifier, VarInt>::deserialize)),
                    |(key, value, modifiers)| EntityUpdateAttrsProperty { key, value, modifiers },
                ))(input)
            }
        }

        pub struct PacketEntityUpdateAttributes<'a> {
            entity_id: VarInt,
            properties: VarArray<EntityUpdateAttrsProperty<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEntityUpdateAttributes<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;

                let w = PrefixedArray::<EntityUpdateAttrsProperty, VarInt>::len(&self.properties).serialize(w)?;

                let mut w = w;
                let items = self.properties.0.iter();
                for i in items {
                    w = EntityUpdateAttrsProperty::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, PrefixedArray::<EntityUpdateAttrsProperty<'a>, VarInt>::deserialize)),
                    |(entity_id, properties)| PacketEntityUpdateAttributes { entity_id, properties },
                ))(input)
            }
        }

        pub struct PacketEntityEffect {
            entity_id: VarInt,
            effect_id: i8,
            amplifier: i8,
            duration: VarInt,
            hide_particles: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityEffect {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.effect_id, w)?;
                let w = i8::serialize(&self.amplifier, w)?;
                let w = VarInt::serialize(&self.duration, w)?;
                let w = i8::serialize(&self.hide_particles, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, i8::deserialize, i8::deserialize, VarInt::deserialize, i8::deserialize)),
                    |(entity_id, effect_id, amplifier, duration, hide_particles)| PacketEntityEffect {
                        entity_id,
                        effect_id,
                        amplifier,
                        duration,
                        hide_particles,
                    },
                ))(input)
            }
        }

        pub struct PacketSelectAdvancementTab<'a> {
            id: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSelectAdvancementTab<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Option::<VarString<'a>>::serialize(&self.id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Option::<VarString<'a>>::deserialize,)), |(id,)| PacketSelectAdvancementTab { id }))(input)
            }
        }

        pub struct CraftingShapeless<'a> {
            group: VarString<'a>,
            ingredients: VarArray<VarArray<Slot>>,
            result: Slot,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for CraftingShapeless<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;

                let w = PrefixedArray::<VarArray<Slot>, VarInt>::len(&self.ingredients).serialize(w)?;

                let mut w = w;
                let items = self.ingredients.0.iter();
                for i in items {
                    w = {
                        let w = PrefixedArray::<Slot, VarInt>::len(&i).serialize(w)?;

                        let mut w = w;
                        let items = i.0.iter();
                        for i in items {
                            w = Slot::serialize(&i, w)?
                        }
                        w
                    }
                }

                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((PrefixedString::<'a, VarInt>::deserialize, PrefixedArray::<VarArray<Slot>, VarInt>::deserialize, Slot::deserialize)),
                    |(group, ingredients, result)| CraftingShapeless { group, ingredients, result },
                ))(input)
            }
        }

        pub struct CraftingShaped<'a> {
            width: VarInt,
            height: VarInt,
            group: VarString<'a>,
            ingredients: Vec<Vec<VarArray<Slot>>>,
            result: Slot,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for CraftingShaped<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.width, w)?;
                let w = VarInt::serialize(&self.height, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;

                let mut w = w;
                let items = self.ingredients.iter();
                for i in items {
                    w = {
                        let mut w = w;
                        let items = i.iter();
                        for i in items {
                            w = {
                                let w = PrefixedArray::<Slot, VarInt>::len(&i).serialize(w)?;

                                let mut w = w;
                                let items = i.0.iter();
                                for i in items {
                                    w = Slot::serialize(&i, w)?
                                }
                                w
                            }
                        }
                        w
                    }
                }

                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_width) = (VarInt::deserialize)(input)?;
                    let (input, self_height) = (VarInt::deserialize)(input)?;
                    let (input, self_group) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_ingredients) = (|input| {
                        let len = self_width;
                        let len = protocol_lib::types::num_traits::ToPrimitive::to_usize(&len).ok_or(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::TooLarge)))?;
                        nom::multi::count(
                            |input| {
                                let len = self_height;
                                let len = protocol_lib::types::num_traits::ToPrimitive::to_usize(&len).ok_or(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::TooLarge)))?;
                                nom::multi::count(PrefixedArray::<Slot, VarInt>::deserialize, len)(input)
                            },
                            len,
                        )(input)
                    })(input)?;
                    let (input, self_result) = (Slot::deserialize)(input)?;
                    Ok((
                        input,
                        CraftingShaped {
                            width: self_width,
                            height: self_height,
                            group: self_group,
                            ingredients: self_ingredients,
                            result: self_result,
                        },
                    ))
                })(input)
            }
        }

        pub struct Stonecutting<'a> {
            group: VarString<'a>,
            ingredient: VarArray<Slot>,
            result: Slot,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Stonecutting<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;

                let w = PrefixedArray::<Slot, VarInt>::len(&self.ingredient).serialize(w)?;

                let mut w = w;
                let items = self.ingredient.0.iter();
                for i in items {
                    w = Slot::serialize(&i, w)?
                }

                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((PrefixedString::<'a, VarInt>::deserialize, PrefixedArray::<Slot, VarInt>::deserialize, Slot::deserialize)),
                    |(group, ingredient, result)| Stonecutting { group, ingredient, result },
                ))(input)
            }
        }

        pub struct Smithing {
            base: VarArray<Slot>,
            addition: VarArray<Slot>,
            result: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for Smithing {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<Slot, VarInt>::len(&self.base).serialize(w)?;

                let mut w = w;
                let items = self.base.0.iter();
                for i in items {
                    w = Slot::serialize(&i, w)?
                }

                let w = PrefixedArray::<Slot, VarInt>::len(&self.addition).serialize(w)?;

                let mut w = w;
                let items = self.addition.0.iter();
                for i in items {
                    w = Slot::serialize(&i, w)?
                }

                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((PrefixedArray::<Slot, VarInt>::deserialize, PrefixedArray::<Slot, VarInt>::deserialize, Slot::deserialize)),
                    |(base, addition, result)| Smithing { base, addition, result },
                ))(input)
            }
        }

        pub enum RecipeData<'a> {
            CraftingShapeless(CraftingShapeless<'a>),
            CraftingShaped(CraftingShaped<'a>),
            CraftingSpecialArmordye,
            CraftingSpecialBookcloning,
            CraftingSpecialMapcloning,
            CraftingSpecialMapextending,
            CraftingSpecialFireworkRocket,
            CraftingSpecialFireworkStar,
            CraftingSpecialFireworkStarFade,
            CraftingSpecialRepairitem,
            CraftingSpecialTippedarrow,
            CraftingSpecialBannerduplicate,
            CraftingSpecialBanneraddpattern,
            CraftingSpecialShielddecoration,
            CraftingSpecialShulkerboxcoloring,
            CraftingSpecialSuspiciousstew,
            Smelting(MinecraftSmeltingFormat<'a>),
            Blasting(MinecraftSmeltingFormat<'a>),
            Smoking(MinecraftSmeltingFormat<'a>),
            CampfireCooking(MinecraftSmeltingFormat<'a>),
            Stonecutting(Stonecutting<'a>),
            Smithing(Smithing),
            Default,
        }

        impl<'a> RecipeData<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    RecipeData::CraftingShapeless(_) => "minecraft:crafting_shapeless",
                    RecipeData::CraftingShaped(_) => "minecraft:crafting_shaped",
                    RecipeData::CraftingSpecialArmordye => "minecraft:crafting_special_armordye",
                    RecipeData::CraftingSpecialBookcloning => "minecraft:crafting_special_bookcloning",
                    RecipeData::CraftingSpecialMapcloning => "minecraft:crafting_special_mapcloning",
                    RecipeData::CraftingSpecialMapextending => "minecraft:crafting_special_mapextending",
                    RecipeData::CraftingSpecialFireworkRocket => "minecraft:crafting_special_firework_rocket",
                    RecipeData::CraftingSpecialFireworkStar => "minecraft:crafting_special_firework_star",
                    RecipeData::CraftingSpecialFireworkStarFade => "minecraft:crafting_special_firework_star_fade",
                    RecipeData::CraftingSpecialRepairitem => "minecraft:crafting_special_repairitem",
                    RecipeData::CraftingSpecialTippedarrow => "minecraft:crafting_special_tippedarrow",
                    RecipeData::CraftingSpecialBannerduplicate => "minecraft:crafting_special_bannerduplicate",
                    RecipeData::CraftingSpecialBanneraddpattern => "minecraft:crafting_special_banneraddpattern",
                    RecipeData::CraftingSpecialShielddecoration => "minecraft:crafting_special_shielddecoration",
                    RecipeData::CraftingSpecialShulkerboxcoloring => "minecraft:crafting_special_shulkerboxcoloring",
                    RecipeData::CraftingSpecialSuspiciousstew => "minecraft:crafting_special_suspiciousstew",
                    RecipeData::Smelting(_) => "minecraft:smelting",
                    RecipeData::Blasting(_) => "minecraft:blasting",
                    RecipeData::Smoking(_) => "minecraft:smoking",
                    RecipeData::CampfireCooking(_) => "minecraft:campfire_cooking",
                    RecipeData::Stonecutting(_) => "minecraft:stonecutting",
                    RecipeData::Smithing(_) => "minecraft:smithing",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    RecipeData::CraftingShapeless(val) => CraftingShapeless::serialize(&val, w)?,
                    RecipeData::CraftingShaped(val) => CraftingShaped::serialize(&val, w)?,
                    RecipeData::CraftingSpecialArmordye => w,
                    RecipeData::CraftingSpecialBookcloning => w,
                    RecipeData::CraftingSpecialMapcloning => w,
                    RecipeData::CraftingSpecialMapextending => w,
                    RecipeData::CraftingSpecialFireworkRocket => w,
                    RecipeData::CraftingSpecialFireworkStar => w,
                    RecipeData::CraftingSpecialFireworkStarFade => w,
                    RecipeData::CraftingSpecialRepairitem => w,
                    RecipeData::CraftingSpecialTippedarrow => w,
                    RecipeData::CraftingSpecialBannerduplicate => w,
                    RecipeData::CraftingSpecialBanneraddpattern => w,
                    RecipeData::CraftingSpecialShielddecoration => w,
                    RecipeData::CraftingSpecialShulkerboxcoloring => w,
                    RecipeData::CraftingSpecialSuspiciousstew => w,
                    RecipeData::Smelting(val) => MinecraftSmeltingFormat::serialize(&val, w)?,
                    RecipeData::Blasting(val) => MinecraftSmeltingFormat::serialize(&val, w)?,
                    RecipeData::Smoking(val) => MinecraftSmeltingFormat::serialize(&val, w)?,
                    RecipeData::CampfireCooking(val) => MinecraftSmeltingFormat::serialize(&val, w)?,
                    RecipeData::Stonecutting(val) => Stonecutting::serialize(&val, w)?,
                    RecipeData::Smithing(val) => Smithing::serialize(&val, w)?,
                    RecipeData::Default => w,
                };

                Ok(w)
            }
        }
        pub struct RecipesItem<'a> {
            r_type: VarString<'a>,
            recipe_id: VarString<'a>,
            data: RecipeData<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for RecipesItem<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.r_type, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe_id, w)?;
                let w = RecipeData::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_r_type) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_recipe_id) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_data) = (|input| match &format!("{}", self_r_type)[..] {
                        "minecraft:crafting_shapeless" => map(CraftingShapeless::deserialize, RecipeData::CraftingShapeless)(input),
                        "minecraft:crafting_shaped" => map(CraftingShaped::deserialize, RecipeData::CraftingShaped)(input),
                        "minecraft:crafting_special_armordye" => Ok((input, RecipeData::CraftingSpecialArmordye)),
                        "minecraft:crafting_special_bookcloning" => Ok((input, RecipeData::CraftingSpecialBookcloning)),
                        "minecraft:crafting_special_mapcloning" => Ok((input, RecipeData::CraftingSpecialMapcloning)),
                        "minecraft:crafting_special_mapextending" => Ok((input, RecipeData::CraftingSpecialMapextending)),
                        "minecraft:crafting_special_firework_rocket" => Ok((input, RecipeData::CraftingSpecialFireworkRocket)),
                        "minecraft:crafting_special_firework_star" => Ok((input, RecipeData::CraftingSpecialFireworkStar)),
                        "minecraft:crafting_special_firework_star_fade" => Ok((input, RecipeData::CraftingSpecialFireworkStarFade)),
                        "minecraft:crafting_special_repairitem" => Ok((input, RecipeData::CraftingSpecialRepairitem)),
                        "minecraft:crafting_special_tippedarrow" => Ok((input, RecipeData::CraftingSpecialTippedarrow)),
                        "minecraft:crafting_special_bannerduplicate" => Ok((input, RecipeData::CraftingSpecialBannerduplicate)),
                        "minecraft:crafting_special_banneraddpattern" => Ok((input, RecipeData::CraftingSpecialBanneraddpattern)),
                        "minecraft:crafting_special_shielddecoration" => Ok((input, RecipeData::CraftingSpecialShielddecoration)),
                        "minecraft:crafting_special_shulkerboxcoloring" => Ok((input, RecipeData::CraftingSpecialShulkerboxcoloring)),
                        "minecraft:crafting_special_suspiciousstew" => Ok((input, RecipeData::CraftingSpecialSuspiciousstew)),
                        "minecraft:smelting" => map(MinecraftSmeltingFormat::deserialize, RecipeData::Smelting)(input),
                        "minecraft:blasting" => map(MinecraftSmeltingFormat::deserialize, RecipeData::Blasting)(input),
                        "minecraft:smoking" => map(MinecraftSmeltingFormat::deserialize, RecipeData::Smoking)(input),
                        "minecraft:campfire_cooking" => map(MinecraftSmeltingFormat::deserialize, RecipeData::CampfireCooking)(input),
                        "minecraft:stonecutting" => map(Stonecutting::deserialize, RecipeData::Stonecutting)(input),
                        "minecraft:smithing" => map(Smithing::deserialize, RecipeData::Smithing)(input),
                        _ => Ok((input, RecipeData::Default)),
                    })(input)?;
                    Ok((
                        input,
                        RecipesItem {
                            r_type: self_r_type,
                            recipe_id: self_recipe_id,
                            data: self_data,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketDeclareRecipes<'a> {
            recipes: VarArray<RecipesItem<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDeclareRecipes<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<RecipesItem, VarInt>::len(&self.recipes).serialize(w)?;

                let mut w = w;
                let items = self.recipes.0.iter();
                for i in items {
                    w = RecipesItem::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedArray::<RecipesItem<'a>, VarInt>::deserialize,)), |(recipes,)| PacketDeclareRecipes { recipes }))(input)
            }
        }

        pub struct TagsTag<'a> {
            tag_type: VarString<'a>,
            tags: VarArray<Tag<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for TagsTag<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.tag_type, w)?;

                let w = PrefixedArray::<Tag, VarInt>::len(&self.tags).serialize(w)?;

                let mut w = w;
                let items = self.tags.0.iter();
                for i in items {
                    w = Tag::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, PrefixedArray::<Tag<'a>, VarInt>::deserialize)), |(tag_type, tags)| {
                    TagsTag { tag_type, tags }
                }))(input)
            }
        }

        pub struct PacketTags<'a> {
            tags: VarArray<TagsTag<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketTags<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<TagsTag, VarInt>::len(&self.tags).serialize(w)?;

                let mut w = w;
                let items = self.tags.0.iter();
                for i in items {
                    w = TagsTag::serialize(&i, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedArray::<TagsTag<'a>, VarInt>::deserialize,)), |(tags,)| PacketTags { tags }))(input)
            }
        }

        pub struct PacketAcknowledgePlayerDigging {
            location: Position,
            block: VarInt,
            status: VarInt,
            successful: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAcknowledgePlayerDigging {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.block, w)?;
                let w = VarInt::serialize(&self.status, w)?;
                let w = bool::serialize(&self.successful, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((Position::deserialize, VarInt::deserialize, VarInt::deserialize, bool::deserialize)),
                    |(location, block, status, successful)| PacketAcknowledgePlayerDigging { location, block, status, successful },
                ))(input)
            }
        }

        pub enum SculkVibrationSignalDestination {
            Block(Position),
            EntityId(VarInt),
            Default,
        }

        impl SculkVibrationSignalDestination {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    SculkVibrationSignalDestination::Block(_) => "block",
                    SculkVibrationSignalDestination::EntityId(_) => "entityId",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    SculkVibrationSignalDestination::Block(val) => Position::serialize(&val, w)?,
                    SculkVibrationSignalDestination::EntityId(val) => VarInt::serialize(&val, w)?,
                    SculkVibrationSignalDestination::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketSculkVibrationSignal<'a> {
            source_position: Position,
            destination_identifier: VarString<'a>,
            destination: SculkVibrationSignalDestination,
            arrival_ticks: VarInt,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSculkVibrationSignal<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.source_position, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.destination_identifier, w)?;
                let w = SculkVibrationSignalDestination::serialize(&self.destination, w)?;
                let w = VarInt::serialize(&self.arrival_ticks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_source_position) = (Position::deserialize)(input)?;
                    let (input, self_destination_identifier) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_destination) = (|input| match &format!("{}", self_destination_identifier)[..] {
                        "block" => map(Position::deserialize, SculkVibrationSignalDestination::Block)(input),
                        "entityId" => map(VarInt::deserialize, SculkVibrationSignalDestination::EntityId)(input),
                        _ => Ok((input, SculkVibrationSignalDestination::Default)),
                    })(input)?;
                    let (input, self_arrival_ticks) = (VarInt::deserialize)(input)?;
                    Ok((
                        input,
                        PacketSculkVibrationSignal {
                            source_position: self_source_position,
                            destination_identifier: self_destination_identifier,
                            destination: self_destination,
                            arrival_ticks: self_arrival_ticks,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketClearTitles {
            reset: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketClearTitles {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.reset, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((bool::deserialize,)), |(reset,)| PacketClearTitles { reset }))(input)
            }
        }

        pub struct PacketInitializeWorldBorder {
            x: f64,
            z: f64,
            old_diameter: f64,
            new_diameter: f64,
            speed: VarLong,
            portal_teleport_boundary: VarInt,
            warning_blocks: VarInt,
            warning_time: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketInitializeWorldBorder {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f64::serialize(&self.old_diameter, w)?;
                let w = f64::serialize(&self.new_diameter, w)?;
                let w = VarLong::serialize(&self.speed, w)?;
                let w = VarInt::serialize(&self.portal_teleport_boundary, w)?;
                let w = VarInt::serialize(&self.warning_blocks, w)?;
                let w = VarInt::serialize(&self.warning_time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        VarLong::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                    )),
                    |(x, z, old_diameter, new_diameter, speed, portal_teleport_boundary, warning_blocks, warning_time)| PacketInitializeWorldBorder {
                        x,
                        z,
                        old_diameter,
                        new_diameter,
                        speed,
                        portal_teleport_boundary,
                        warning_blocks,
                        warning_time,
                    },
                ))(input)
            }
        }

        pub struct PacketActionBar<'a> {
            text: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketActionBar<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(text,)| PacketActionBar { text }))(input)
            }
        }

        pub struct PacketWorldBorderCenter {
            x: f64,
            z: f64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderCenter {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f64::deserialize, f64::deserialize)), |(x, z)| PacketWorldBorderCenter { x, z }))(input)
            }
        }

        pub struct PacketWorldBorderLerpSize {
            old_diameter: f64,
            new_diameter: f64,
            speed: VarLong,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderLerpSize {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.old_diameter, w)?;
                let w = f64::serialize(&self.new_diameter, w)?;
                let w = VarLong::serialize(&self.speed, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f64::deserialize, f64::deserialize, VarLong::deserialize)), |(old_diameter, new_diameter, speed)| {
                    PacketWorldBorderLerpSize { old_diameter, new_diameter, speed }
                }))(input)
            }
        }

        pub struct PacketWorldBorderSize {
            diameter: f64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderSize {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.diameter, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f64::deserialize,)), |(diameter,)| PacketWorldBorderSize { diameter }))(input)
            }
        }

        pub struct PacketWorldBorderWarningDelay {
            warning_time: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderWarningDelay {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.warning_time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(warning_time,)| PacketWorldBorderWarningDelay { warning_time }))(input)
            }
        }

        pub struct PacketWorldBorderWarningReach {
            warning_blocks: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderWarningReach {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.warning_blocks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(warning_blocks,)| PacketWorldBorderWarningReach { warning_blocks }))(input)
            }
        }

        pub struct PacketPing {
            id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPing {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i32::deserialize,)), |(id,)| PacketPing { id }))(input)
            }
        }

        pub struct PacketSetTitleSubtitle<'a> {
            text: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSetTitleSubtitle<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(text,)| PacketSetTitleSubtitle { text }))(input)
            }
        }

        pub struct PacketSetTitleText<'a> {
            text: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSetTitleText<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(text,)| PacketSetTitleText { text }))(input)
            }
        }

        pub struct PacketSetTitleTime {
            fade_in: i32,
            stay: i32,
            fade_out: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetTitleTime {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.fade_in, w)?;
                let w = i32::serialize(&self.stay, w)?;
                let w = i32::serialize(&self.fade_out, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i32::deserialize, i32::deserialize, i32::deserialize)), |(fade_in, stay, fade_out)| PacketSetTitleTime {
                    fade_in,
                    stay,
                    fade_out,
                }))(input)
            }
        }

        pub struct PacketSimulationDistance {
            distance: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSimulationDistance {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.distance, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(distance,)| PacketSimulationDistance { distance }))(input)
            }
        }

        pub enum Params<'a> {
            SpawnEntity(PacketSpawnEntity),
            SpawnEntityExperienceOrb(PacketSpawnEntityExperienceOrb),
            SpawnEntityLiving(PacketSpawnEntityLiving),
            SpawnEntityPainting(PacketSpawnEntityPainting),
            NamedEntitySpawn(PacketNamedEntitySpawn),
            Animation(PacketAnimation),
            Statistics(PacketStatistics),
            Advancements(PacketAdvancements<'a>),
            BlockBreakAnimation(PacketBlockBreakAnimation),
            TileEntityData(PacketTileEntityData),
            BlockAction(PacketBlockAction),
            BlockChange(PacketBlockChange),
            BossBar(PacketBossBar<'a>),
            Difficulty(PacketDifficulty),
            TabComplete(PacketTabComplete<'a>),
            DeclareCommands(PacketDeclareCommands<'a>),
            FacePlayer(PacketFacePlayer<'a>),
            NbtQueryResponse(PacketNbtQueryResponse),
            Chat(PacketChat<'a>),
            MultiBlockChange(PacketMultiBlockChange),
            CloseWindow(PacketCloseWindow),
            OpenWindow(PacketOpenWindow<'a>),
            WindowItems(PacketWindowItems),
            CraftProgressBar(PacketCraftProgressBar),
            SetSlot(PacketSetSlot),
            SetCooldown(PacketSetCooldown),
            CustomPayload(PacketCustomPayload<'a>),
            NamedSoundEffect(PacketNamedSoundEffect<'a>),
            KickDisconnect(PacketKickDisconnect<'a>),
            EntityStatus(PacketEntityStatus),
            Explosion(PacketExplosion),
            UnloadChunk(PacketUnloadChunk),
            GameStateChange(PacketGameStateChange),
            OpenHorseWindow(PacketOpenHorseWindow),
            KeepAlive(PacketKeepAlive),
            MapChunk(PacketMapChunk<'a>),
            WorldEvent(PacketWorldEvent),
            WorldParticles(PacketWorldParticles<'a>),
            UpdateLight(PacketUpdateLight),
            Login(PacketLogin<'a>),
            Map(PacketMap<'a>),
            TradeList(PacketTradeList),
            RelEntityMove(PacketRelEntityMove),
            EntityMoveLook(PacketEntityMoveLook),
            EntityLook(PacketEntityLook),
            VehicleMove(PacketVehicleMove),
            OpenBook(PacketOpenBook),
            OpenSignEntity(PacketOpenSignEntity),
            CraftRecipeResponse(PacketCraftRecipeResponse<'a>),
            Abilities(PacketAbilities),
            EndCombatEvent(PacketEndCombatEvent),
            EnterCombatEvent(PacketEnterCombatEvent),
            DeathCombatEvent(PacketDeathCombatEvent<'a>),
            PlayerInfo(PacketPlayerInfo<'a>),
            Position(PacketPosition),
            UnlockRecipes(PacketUnlockRecipes<'a>),
            EntityDestroy(PacketEntityDestroy),
            RemoveEntityEffect(PacketRemoveEntityEffect),
            ResourcePackSend(PacketResourcePackSend<'a>),
            Respawn(PacketRespawn<'a>),
            EntityUpdateAttributes(PacketEntityUpdateAttributes<'a>),
            Camera(PacketCamera),
            HeldItemSlot(PacketHeldItemSlot),
            UpdateViewPosition(PacketUpdateViewPosition),
            UpdateViewDistance(PacketUpdateViewDistance),
            ScoreboardDisplayObjective(PacketScoreboardDisplayObjective<'a>),
            EntityMetadata(PacketEntityMetadata<'a>),
            AttachEntity(PacketAttachEntity),
            EntityVelocity(PacketEntityVelocity),
            EntityEquipment(PacketEntityEquipment),
            Experience(PacketExperience),
            UpdateHealth(PacketUpdateHealth),
            ScoreboardObjective(PacketScoreboardObjective<'a>),
            SetPassengers(PacketSetPassengers),
            Teams(PacketTeams<'a>),
            ScoreboardScore(PacketScoreboardScore<'a>),
            SimulationDistance(PacketSimulationDistance),
            SpawnPosition(PacketSpawnPosition),
            UpdateTime(PacketUpdateTime),
            EntitySoundEffect(PacketEntitySoundEffect),
            StopSound(PacketStopSound<'a>),
            SoundEffect(PacketSoundEffect),
            PlayerlistHeader(PacketPlayerlistHeader<'a>),
            Collect(PacketCollect),
            EntityTeleport(PacketEntityTeleport),
            EntityHeadRotation(PacketEntityHeadRotation),
            EntityEffect(PacketEntityEffect),
            SelectAdvancementTab(PacketSelectAdvancementTab<'a>),
            DeclareRecipes(PacketDeclareRecipes<'a>),
            Tags(PacketTags<'a>),
            AcknowledgePlayerDigging(PacketAcknowledgePlayerDigging),
            SculkVibrationSignal(PacketSculkVibrationSignal<'a>),
            ClearTitles(PacketClearTitles),
            InitializeWorldBorder(PacketInitializeWorldBorder),
            ActionBar(PacketActionBar<'a>),
            WorldBorderCenter(PacketWorldBorderCenter),
            WorldBorderLerpSize(PacketWorldBorderLerpSize),
            WorldBorderSize(PacketWorldBorderSize),
            WorldBorderWarningDelay(PacketWorldBorderWarningDelay),
            WorldBorderWarningReach(PacketWorldBorderWarningReach),
            Ping(PacketPing),
            SetTitleSubtitle(PacketSetTitleSubtitle<'a>),
            SetTitleText(PacketSetTitleText<'a>),
            SetTitleTime(PacketSetTitleTime),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::SpawnEntity(_) => "spawn_entity",
                    Params::SpawnEntityExperienceOrb(_) => "spawn_entity_experience_orb",
                    Params::SpawnEntityLiving(_) => "spawn_entity_living",
                    Params::SpawnEntityPainting(_) => "spawn_entity_painting",
                    Params::NamedEntitySpawn(_) => "named_entity_spawn",
                    Params::Animation(_) => "animation",
                    Params::Statistics(_) => "statistics",
                    Params::Advancements(_) => "advancements",
                    Params::BlockBreakAnimation(_) => "block_break_animation",
                    Params::TileEntityData(_) => "tile_entity_data",
                    Params::BlockAction(_) => "block_action",
                    Params::BlockChange(_) => "block_change",
                    Params::BossBar(_) => "boss_bar",
                    Params::Difficulty(_) => "difficulty",
                    Params::TabComplete(_) => "tab_complete",
                    Params::DeclareCommands(_) => "declare_commands",
                    Params::FacePlayer(_) => "face_player",
                    Params::NbtQueryResponse(_) => "nbt_query_response",
                    Params::Chat(_) => "chat",
                    Params::MultiBlockChange(_) => "multi_block_change",
                    Params::CloseWindow(_) => "close_window",
                    Params::OpenWindow(_) => "open_window",
                    Params::WindowItems(_) => "window_items",
                    Params::CraftProgressBar(_) => "craft_progress_bar",
                    Params::SetSlot(_) => "set_slot",
                    Params::SetCooldown(_) => "set_cooldown",
                    Params::CustomPayload(_) => "custom_payload",
                    Params::NamedSoundEffect(_) => "named_sound_effect",
                    Params::KickDisconnect(_) => "kick_disconnect",
                    Params::EntityStatus(_) => "entity_status",
                    Params::Explosion(_) => "explosion",
                    Params::UnloadChunk(_) => "unload_chunk",
                    Params::GameStateChange(_) => "game_state_change",
                    Params::OpenHorseWindow(_) => "open_horse_window",
                    Params::KeepAlive(_) => "keep_alive",
                    Params::MapChunk(_) => "map_chunk",
                    Params::WorldEvent(_) => "world_event",
                    Params::WorldParticles(_) => "world_particles",
                    Params::UpdateLight(_) => "update_light",
                    Params::Login(_) => "login",
                    Params::Map(_) => "map",
                    Params::TradeList(_) => "trade_list",
                    Params::RelEntityMove(_) => "rel_entity_move",
                    Params::EntityMoveLook(_) => "entity_move_look",
                    Params::EntityLook(_) => "entity_look",
                    Params::VehicleMove(_) => "vehicle_move",
                    Params::OpenBook(_) => "open_book",
                    Params::OpenSignEntity(_) => "open_sign_entity",
                    Params::CraftRecipeResponse(_) => "craft_recipe_response",
                    Params::Abilities(_) => "abilities",
                    Params::EndCombatEvent(_) => "end_combat_event",
                    Params::EnterCombatEvent(_) => "enter_combat_event",
                    Params::DeathCombatEvent(_) => "death_combat_event",
                    Params::PlayerInfo(_) => "player_info",
                    Params::Position(_) => "position",
                    Params::UnlockRecipes(_) => "unlock_recipes",
                    Params::EntityDestroy(_) => "entity_destroy",
                    Params::RemoveEntityEffect(_) => "remove_entity_effect",
                    Params::ResourcePackSend(_) => "resource_pack_send",
                    Params::Respawn(_) => "respawn",
                    Params::EntityUpdateAttributes(_) => "entity_update_attributes",
                    Params::Camera(_) => "camera",
                    Params::HeldItemSlot(_) => "held_item_slot",
                    Params::UpdateViewPosition(_) => "update_view_position",
                    Params::UpdateViewDistance(_) => "update_view_distance",
                    Params::ScoreboardDisplayObjective(_) => "scoreboard_display_objective",
                    Params::EntityMetadata(_) => "entity_metadata",
                    Params::AttachEntity(_) => "attach_entity",
                    Params::EntityVelocity(_) => "entity_velocity",
                    Params::EntityEquipment(_) => "entity_equipment",
                    Params::Experience(_) => "experience",
                    Params::UpdateHealth(_) => "update_health",
                    Params::ScoreboardObjective(_) => "scoreboard_objective",
                    Params::SetPassengers(_) => "set_passengers",
                    Params::Teams(_) => "teams",
                    Params::ScoreboardScore(_) => "scoreboard_score",
                    Params::SimulationDistance(_) => "simulation_distance",
                    Params::SpawnPosition(_) => "spawn_position",
                    Params::UpdateTime(_) => "update_time",
                    Params::EntitySoundEffect(_) => "entity_sound_effect",
                    Params::StopSound(_) => "stop_sound",
                    Params::SoundEffect(_) => "sound_effect",
                    Params::PlayerlistHeader(_) => "playerlist_header",
                    Params::Collect(_) => "collect",
                    Params::EntityTeleport(_) => "entity_teleport",
                    Params::EntityHeadRotation(_) => "entity_head_rotation",
                    Params::EntityEffect(_) => "entity_effect",
                    Params::SelectAdvancementTab(_) => "select_advancement_tab",
                    Params::DeclareRecipes(_) => "declare_recipes",
                    Params::Tags(_) => "tags",
                    Params::AcknowledgePlayerDigging(_) => "acknowledge_player_digging",
                    Params::SculkVibrationSignal(_) => "sculk_vibration_signal",
                    Params::ClearTitles(_) => "clear_titles",
                    Params::InitializeWorldBorder(_) => "initialize_world_border",
                    Params::ActionBar(_) => "action_bar",
                    Params::WorldBorderCenter(_) => "world_border_center",
                    Params::WorldBorderLerpSize(_) => "world_border_lerp_size",
                    Params::WorldBorderSize(_) => "world_border_size",
                    Params::WorldBorderWarningDelay(_) => "world_border_warning_delay",
                    Params::WorldBorderWarningReach(_) => "world_border_warning_reach",
                    Params::Ping(_) => "ping",
                    Params::SetTitleSubtitle(_) => "set_title_subtitle",
                    Params::SetTitleText(_) => "set_title_text",
                    Params::SetTitleTime(_) => "set_title_time",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::SpawnEntity(val) => PacketSpawnEntity::serialize(&val, w)?,
                    Params::SpawnEntityExperienceOrb(val) => PacketSpawnEntityExperienceOrb::serialize(&val, w)?,
                    Params::SpawnEntityLiving(val) => PacketSpawnEntityLiving::serialize(&val, w)?,
                    Params::SpawnEntityPainting(val) => PacketSpawnEntityPainting::serialize(&val, w)?,
                    Params::NamedEntitySpawn(val) => PacketNamedEntitySpawn::serialize(&val, w)?,
                    Params::Animation(val) => PacketAnimation::serialize(&val, w)?,
                    Params::Statistics(val) => PacketStatistics::serialize(&val, w)?,
                    Params::Advancements(val) => PacketAdvancements::serialize(&val, w)?,
                    Params::BlockBreakAnimation(val) => PacketBlockBreakAnimation::serialize(&val, w)?,
                    Params::TileEntityData(val) => PacketTileEntityData::serialize(&val, w)?,
                    Params::BlockAction(val) => PacketBlockAction::serialize(&val, w)?,
                    Params::BlockChange(val) => PacketBlockChange::serialize(&val, w)?,
                    Params::BossBar(val) => PacketBossBar::serialize(&val, w)?,
                    Params::Difficulty(val) => PacketDifficulty::serialize(&val, w)?,
                    Params::TabComplete(val) => PacketTabComplete::serialize(&val, w)?,
                    Params::DeclareCommands(val) => PacketDeclareCommands::serialize(&val, w)?,
                    Params::FacePlayer(val) => PacketFacePlayer::serialize(&val, w)?,
                    Params::NbtQueryResponse(val) => PacketNbtQueryResponse::serialize(&val, w)?,
                    Params::Chat(val) => PacketChat::serialize(&val, w)?,
                    Params::MultiBlockChange(val) => PacketMultiBlockChange::serialize(&val, w)?,
                    Params::CloseWindow(val) => PacketCloseWindow::serialize(&val, w)?,
                    Params::OpenWindow(val) => PacketOpenWindow::serialize(&val, w)?,
                    Params::WindowItems(val) => PacketWindowItems::serialize(&val, w)?,
                    Params::CraftProgressBar(val) => PacketCraftProgressBar::serialize(&val, w)?,
                    Params::SetSlot(val) => PacketSetSlot::serialize(&val, w)?,
                    Params::SetCooldown(val) => PacketSetCooldown::serialize(&val, w)?,
                    Params::CustomPayload(val) => PacketCustomPayload::serialize(&val, w)?,
                    Params::NamedSoundEffect(val) => PacketNamedSoundEffect::serialize(&val, w)?,
                    Params::KickDisconnect(val) => PacketKickDisconnect::serialize(&val, w)?,
                    Params::EntityStatus(val) => PacketEntityStatus::serialize(&val, w)?,
                    Params::Explosion(val) => PacketExplosion::serialize(&val, w)?,
                    Params::UnloadChunk(val) => PacketUnloadChunk::serialize(&val, w)?,
                    Params::GameStateChange(val) => PacketGameStateChange::serialize(&val, w)?,
                    Params::OpenHorseWindow(val) => PacketOpenHorseWindow::serialize(&val, w)?,
                    Params::KeepAlive(val) => PacketKeepAlive::serialize(&val, w)?,
                    Params::MapChunk(val) => PacketMapChunk::serialize(&val, w)?,
                    Params::WorldEvent(val) => PacketWorldEvent::serialize(&val, w)?,
                    Params::WorldParticles(val) => PacketWorldParticles::serialize(&val, w)?,
                    Params::UpdateLight(val) => PacketUpdateLight::serialize(&val, w)?,
                    Params::Login(val) => PacketLogin::serialize(&val, w)?,
                    Params::Map(val) => PacketMap::serialize(&val, w)?,
                    Params::TradeList(val) => PacketTradeList::serialize(&val, w)?,
                    Params::RelEntityMove(val) => PacketRelEntityMove::serialize(&val, w)?,
                    Params::EntityMoveLook(val) => PacketEntityMoveLook::serialize(&val, w)?,
                    Params::EntityLook(val) => PacketEntityLook::serialize(&val, w)?,
                    Params::VehicleMove(val) => PacketVehicleMove::serialize(&val, w)?,
                    Params::OpenBook(val) => PacketOpenBook::serialize(&val, w)?,
                    Params::OpenSignEntity(val) => PacketOpenSignEntity::serialize(&val, w)?,
                    Params::CraftRecipeResponse(val) => PacketCraftRecipeResponse::serialize(&val, w)?,
                    Params::Abilities(val) => PacketAbilities::serialize(&val, w)?,
                    Params::EndCombatEvent(val) => PacketEndCombatEvent::serialize(&val, w)?,
                    Params::EnterCombatEvent(val) => PacketEnterCombatEvent::serialize(&val, w)?,
                    Params::DeathCombatEvent(val) => PacketDeathCombatEvent::serialize(&val, w)?,
                    Params::PlayerInfo(val) => PacketPlayerInfo::serialize(&val, w)?,
                    Params::Position(val) => PacketPosition::serialize(&val, w)?,
                    Params::UnlockRecipes(val) => PacketUnlockRecipes::serialize(&val, w)?,
                    Params::EntityDestroy(val) => PacketEntityDestroy::serialize(&val, w)?,
                    Params::RemoveEntityEffect(val) => PacketRemoveEntityEffect::serialize(&val, w)?,
                    Params::ResourcePackSend(val) => PacketResourcePackSend::serialize(&val, w)?,
                    Params::Respawn(val) => PacketRespawn::serialize(&val, w)?,
                    Params::EntityUpdateAttributes(val) => PacketEntityUpdateAttributes::serialize(&val, w)?,
                    Params::Camera(val) => PacketCamera::serialize(&val, w)?,
                    Params::HeldItemSlot(val) => PacketHeldItemSlot::serialize(&val, w)?,
                    Params::UpdateViewPosition(val) => PacketUpdateViewPosition::serialize(&val, w)?,
                    Params::UpdateViewDistance(val) => PacketUpdateViewDistance::serialize(&val, w)?,
                    Params::ScoreboardDisplayObjective(val) => PacketScoreboardDisplayObjective::serialize(&val, w)?,
                    Params::EntityMetadata(val) => PacketEntityMetadata::serialize(&val, w)?,
                    Params::AttachEntity(val) => PacketAttachEntity::serialize(&val, w)?,
                    Params::EntityVelocity(val) => PacketEntityVelocity::serialize(&val, w)?,
                    Params::EntityEquipment(val) => PacketEntityEquipment::serialize(&val, w)?,
                    Params::Experience(val) => PacketExperience::serialize(&val, w)?,
                    Params::UpdateHealth(val) => PacketUpdateHealth::serialize(&val, w)?,
                    Params::ScoreboardObjective(val) => PacketScoreboardObjective::serialize(&val, w)?,
                    Params::SetPassengers(val) => PacketSetPassengers::serialize(&val, w)?,
                    Params::Teams(val) => PacketTeams::serialize(&val, w)?,
                    Params::ScoreboardScore(val) => PacketScoreboardScore::serialize(&val, w)?,
                    Params::SimulationDistance(val) => PacketSimulationDistance::serialize(&val, w)?,
                    Params::SpawnPosition(val) => PacketSpawnPosition::serialize(&val, w)?,
                    Params::UpdateTime(val) => PacketUpdateTime::serialize(&val, w)?,
                    Params::EntitySoundEffect(val) => PacketEntitySoundEffect::serialize(&val, w)?,
                    Params::StopSound(val) => PacketStopSound::serialize(&val, w)?,
                    Params::SoundEffect(val) => PacketSoundEffect::serialize(&val, w)?,
                    Params::PlayerlistHeader(val) => PacketPlayerlistHeader::serialize(&val, w)?,
                    Params::Collect(val) => PacketCollect::serialize(&val, w)?,
                    Params::EntityTeleport(val) => PacketEntityTeleport::serialize(&val, w)?,
                    Params::EntityHeadRotation(val) => PacketEntityHeadRotation::serialize(&val, w)?,
                    Params::EntityEffect(val) => PacketEntityEffect::serialize(&val, w)?,
                    Params::SelectAdvancementTab(val) => PacketSelectAdvancementTab::serialize(&val, w)?,
                    Params::DeclareRecipes(val) => PacketDeclareRecipes::serialize(&val, w)?,
                    Params::Tags(val) => PacketTags::serialize(&val, w)?,
                    Params::AcknowledgePlayerDigging(val) => PacketAcknowledgePlayerDigging::serialize(&val, w)?,
                    Params::SculkVibrationSignal(val) => PacketSculkVibrationSignal::serialize(&val, w)?,
                    Params::ClearTitles(val) => PacketClearTitles::serialize(&val, w)?,
                    Params::InitializeWorldBorder(val) => PacketInitializeWorldBorder::serialize(&val, w)?,
                    Params::ActionBar(val) => PacketActionBar::serialize(&val, w)?,
                    Params::WorldBorderCenter(val) => PacketWorldBorderCenter::serialize(&val, w)?,
                    Params::WorldBorderLerpSize(val) => PacketWorldBorderLerpSize::serialize(&val, w)?,
                    Params::WorldBorderSize(val) => PacketWorldBorderSize::serialize(&val, w)?,
                    Params::WorldBorderWarningDelay(val) => PacketWorldBorderWarningDelay::serialize(&val, w)?,
                    Params::WorldBorderWarningReach(val) => PacketWorldBorderWarningReach::serialize(&val, w)?,
                    Params::Ping(val) => PacketPing::serialize(&val, w)?,
                    Params::SetTitleSubtitle(val) => PacketSetTitleSubtitle::serialize(&val, w)?,
                    Params::SetTitleText(val) => PacketSetTitleText::serialize(&val, w)?,
                    Params::SetTitleTime(val) => PacketSetTitleTime::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "spawn_entity" => "0x00",
                    "spawn_entity_experience_orb" => "0x01",
                    "spawn_entity_living" => "0x02",
                    "spawn_entity_painting" => "0x03",
                    "named_entity_spawn" => "0x04",
                    "sculk_vibration_signal" => "0x05",
                    "animation" => "0x06",
                    "statistics" => "0x07",
                    "acknowledge_player_digging" => "0x08",
                    "block_break_animation" => "0x09",
                    "tile_entity_data" => "0x0a",
                    "block_action" => "0x0b",
                    "block_change" => "0x0c",
                    "boss_bar" => "0x0d",
                    "difficulty" => "0x0e",
                    "chat" => "0x0f",
                    "clear_titles" => "0x10",
                    "tab_complete" => "0x11",
                    "declare_commands" => "0x12",
                    "close_window" => "0x13",
                    "window_items" => "0x14",
                    "craft_progress_bar" => "0x15",
                    "set_slot" => "0x16",
                    "set_cooldown" => "0x17",
                    "custom_payload" => "0x18",
                    "named_sound_effect" => "0x19",
                    "kick_disconnect" => "0x1a",
                    "entity_status" => "0x1b",
                    "explosion" => "0x1c",
                    "unload_chunk" => "0x1d",
                    "game_state_change" => "0x1e",
                    "open_horse_window" => "0x1f",
                    "initialize_world_border" => "0x20",
                    "keep_alive" => "0x21",
                    "map_chunk" => "0x22",
                    "world_event" => "0x23",
                    "world_particles" => "0x24",
                    "update_light" => "0x25",
                    "login" => "0x26",
                    "map" => "0x27",
                    "trade_list" => "0x28",
                    "rel_entity_move" => "0x29",
                    "entity_move_look" => "0x2a",
                    "entity_look" => "0x2b",
                    "vehicle_move" => "0x2c",
                    "open_book" => "0x2d",
                    "open_window" => "0x2e",
                    "open_sign_entity" => "0x2f",
                    "ping" => "0x30",
                    "craft_recipe_response" => "0x31",
                    "abilities" => "0x32",
                    "end_combat_event" => "0x33",
                    "enter_combat_event" => "0x34",
                    "death_combat_event" => "0x35",
                    "player_info" => "0x36",
                    "face_player" => "0x37",
                    "position" => "0x38",
                    "unlock_recipes" => "0x39",
                    "entity_destroy" => "0x3a",
                    "remove_entity_effect" => "0x3b",
                    "resource_pack_send" => "0x3c",
                    "respawn" => "0x3d",
                    "entity_head_rotation" => "0x3e",
                    "multi_block_change" => "0x3f",
                    "select_advancement_tab" => "0x40",
                    "action_bar" => "0x41",
                    "world_border_center" => "0x42",
                    "world_border_lerp_size" => "0x43",
                    "world_border_size" => "0x44",
                    "world_border_warning_delay" => "0x45",
                    "world_border_warning_reach" => "0x46",
                    "camera" => "0x47",
                    "held_item_slot" => "0x48",
                    "update_view_position" => "0x49",
                    "update_view_distance" => "0x4a",
                    "spawn_position" => "0x4b",
                    "scoreboard_display_objective" => "0x4c",
                    "entity_metadata" => "0x4d",
                    "attach_entity" => "0x4e",
                    "entity_velocity" => "0x4f",
                    "entity_equipment" => "0x50",
                    "experience" => "0x51",
                    "update_health" => "0x52",
                    "scoreboard_objective" => "0x53",
                    "set_passengers" => "0x54",
                    "teams" => "0x55",
                    "scoreboard_score" => "0x56",
                    "simulation_distance" => "0x57",
                    "set_title_subtitle" => "0x58",
                    "update_time" => "0x59",
                    "set_title_text" => "0x5a",
                    "set_title_time" => "0x5b",
                    "entity_sound_effect" => "0x5c",
                    "sound_effect" => "0x5d",
                    "stop_sound" => "0x5e",
                    "playerlist_header" => "0x5f",
                    "nbt_query_response" => "0x60",
                    "collect" => "0x61",
                    "entity_teleport" => "0x62",
                    "advancements" => "0x63",
                    "entity_update_attributes" => "0x64",
                    "entity_effect" => "0x65",
                    "declare_recipes" => "0x66",
                    "tags" => "0x67",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|input| {
                        let (input, x) = (VarInt::deserialize)(input)?;
                        let x = format!("{x}");
                        let val = match &x[..] {
                            "0x00" => "spawn_entity",
                            "0x01" => "spawn_entity_experience_orb",
                            "0x02" => "spawn_entity_living",
                            "0x03" => "spawn_entity_painting",
                            "0x04" => "named_entity_spawn",
                            "0x05" => "sculk_vibration_signal",
                            "0x06" => "animation",
                            "0x07" => "statistics",
                            "0x08" => "acknowledge_player_digging",
                            "0x09" => "block_break_animation",
                            "0x0a" => "tile_entity_data",
                            "0x0b" => "block_action",
                            "0x0c" => "block_change",
                            "0x0d" => "boss_bar",
                            "0x0e" => "difficulty",
                            "0x0f" => "chat",
                            "0x10" => "clear_titles",
                            "0x11" => "tab_complete",
                            "0x12" => "declare_commands",
                            "0x13" => "close_window",
                            "0x14" => "window_items",
                            "0x15" => "craft_progress_bar",
                            "0x16" => "set_slot",
                            "0x17" => "set_cooldown",
                            "0x18" => "custom_payload",
                            "0x19" => "named_sound_effect",
                            "0x1a" => "kick_disconnect",
                            "0x1b" => "entity_status",
                            "0x1c" => "explosion",
                            "0x1d" => "unload_chunk",
                            "0x1e" => "game_state_change",
                            "0x1f" => "open_horse_window",
                            "0x20" => "initialize_world_border",
                            "0x21" => "keep_alive",
                            "0x22" => "map_chunk",
                            "0x23" => "world_event",
                            "0x24" => "world_particles",
                            "0x25" => "update_light",
                            "0x26" => "login",
                            "0x27" => "map",
                            "0x28" => "trade_list",
                            "0x29" => "rel_entity_move",
                            "0x2a" => "entity_move_look",
                            "0x2b" => "entity_look",
                            "0x2c" => "vehicle_move",
                            "0x2d" => "open_book",
                            "0x2e" => "open_window",
                            "0x2f" => "open_sign_entity",
                            "0x30" => "ping",
                            "0x31" => "craft_recipe_response",
                            "0x32" => "abilities",
                            "0x33" => "end_combat_event",
                            "0x34" => "enter_combat_event",
                            "0x35" => "death_combat_event",
                            "0x36" => "player_info",
                            "0x37" => "face_player",
                            "0x38" => "position",
                            "0x39" => "unlock_recipes",
                            "0x3a" => "entity_destroy",
                            "0x3b" => "remove_entity_effect",
                            "0x3c" => "resource_pack_send",
                            "0x3d" => "respawn",
                            "0x3e" => "entity_head_rotation",
                            "0x3f" => "multi_block_change",
                            "0x40" => "select_advancement_tab",
                            "0x41" => "action_bar",
                            "0x42" => "world_border_center",
                            "0x43" => "world_border_lerp_size",
                            "0x44" => "world_border_size",
                            "0x45" => "world_border_warning_delay",
                            "0x46" => "world_border_warning_reach",
                            "0x47" => "camera",
                            "0x48" => "held_item_slot",
                            "0x49" => "update_view_position",
                            "0x4a" => "update_view_distance",
                            "0x4b" => "spawn_position",
                            "0x4c" => "scoreboard_display_objective",
                            "0x4d" => "entity_metadata",
                            "0x4e" => "attach_entity",
                            "0x4f" => "entity_velocity",
                            "0x50" => "entity_equipment",
                            "0x51" => "experience",
                            "0x52" => "update_health",
                            "0x53" => "scoreboard_objective",
                            "0x54" => "set_passengers",
                            "0x55" => "teams",
                            "0x56" => "scoreboard_score",
                            "0x57" => "simulation_distance",
                            "0x58" => "set_title_subtitle",
                            "0x59" => "update_time",
                            "0x5a" => "set_title_text",
                            "0x5b" => "set_title_time",
                            "0x5c" => "entity_sound_effect",
                            "0x5d" => "sound_effect",
                            "0x5e" => "stop_sound",
                            "0x5f" => "playerlist_header",
                            "0x60" => "nbt_query_response",
                            "0x61" => "collect",
                            "0x62" => "entity_teleport",
                            "0x63" => "advancements",
                            "0x64" => "entity_update_attributes",
                            "0x65" => "entity_effect",
                            "0x66" => "declare_recipes",
                            "0x67" => "tags",

                            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "spawn_entity" => map(PacketSpawnEntity::deserialize, Params::SpawnEntity)(input),
                        "spawn_entity_experience_orb" => map(PacketSpawnEntityExperienceOrb::deserialize, Params::SpawnEntityExperienceOrb)(input),
                        "spawn_entity_living" => map(PacketSpawnEntityLiving::deserialize, Params::SpawnEntityLiving)(input),
                        "spawn_entity_painting" => map(PacketSpawnEntityPainting::deserialize, Params::SpawnEntityPainting)(input),
                        "named_entity_spawn" => map(PacketNamedEntitySpawn::deserialize, Params::NamedEntitySpawn)(input),
                        "animation" => map(PacketAnimation::deserialize, Params::Animation)(input),
                        "statistics" => map(PacketStatistics::deserialize, Params::Statistics)(input),
                        "advancements" => map(PacketAdvancements::deserialize, Params::Advancements)(input),
                        "block_break_animation" => map(PacketBlockBreakAnimation::deserialize, Params::BlockBreakAnimation)(input),
                        "tile_entity_data" => map(PacketTileEntityData::deserialize, Params::TileEntityData)(input),
                        "block_action" => map(PacketBlockAction::deserialize, Params::BlockAction)(input),
                        "block_change" => map(PacketBlockChange::deserialize, Params::BlockChange)(input),
                        "boss_bar" => map(PacketBossBar::deserialize, Params::BossBar)(input),
                        "difficulty" => map(PacketDifficulty::deserialize, Params::Difficulty)(input),
                        "tab_complete" => map(PacketTabComplete::deserialize, Params::TabComplete)(input),
                        "declare_commands" => map(PacketDeclareCommands::deserialize, Params::DeclareCommands)(input),
                        "face_player" => map(PacketFacePlayer::deserialize, Params::FacePlayer)(input),
                        "nbt_query_response" => map(PacketNbtQueryResponse::deserialize, Params::NbtQueryResponse)(input),
                        "chat" => map(PacketChat::deserialize, Params::Chat)(input),
                        "multi_block_change" => map(PacketMultiBlockChange::deserialize, Params::MultiBlockChange)(input),
                        "close_window" => map(PacketCloseWindow::deserialize, Params::CloseWindow)(input),
                        "open_window" => map(PacketOpenWindow::deserialize, Params::OpenWindow)(input),
                        "window_items" => map(PacketWindowItems::deserialize, Params::WindowItems)(input),
                        "craft_progress_bar" => map(PacketCraftProgressBar::deserialize, Params::CraftProgressBar)(input),
                        "set_slot" => map(PacketSetSlot::deserialize, Params::SetSlot)(input),
                        "set_cooldown" => map(PacketSetCooldown::deserialize, Params::SetCooldown)(input),
                        "custom_payload" => map(PacketCustomPayload::deserialize, Params::CustomPayload)(input),
                        "named_sound_effect" => map(PacketNamedSoundEffect::deserialize, Params::NamedSoundEffect)(input),
                        "kick_disconnect" => map(PacketKickDisconnect::deserialize, Params::KickDisconnect)(input),
                        "entity_status" => map(PacketEntityStatus::deserialize, Params::EntityStatus)(input),
                        "explosion" => map(PacketExplosion::deserialize, Params::Explosion)(input),
                        "unload_chunk" => map(PacketUnloadChunk::deserialize, Params::UnloadChunk)(input),
                        "game_state_change" => map(PacketGameStateChange::deserialize, Params::GameStateChange)(input),
                        "open_horse_window" => map(PacketOpenHorseWindow::deserialize, Params::OpenHorseWindow)(input),
                        "keep_alive" => map(PacketKeepAlive::deserialize, Params::KeepAlive)(input),
                        "map_chunk" => map(PacketMapChunk::deserialize, Params::MapChunk)(input),
                        "world_event" => map(PacketWorldEvent::deserialize, Params::WorldEvent)(input),
                        "world_particles" => map(PacketWorldParticles::deserialize, Params::WorldParticles)(input),
                        "update_light" => map(PacketUpdateLight::deserialize, Params::UpdateLight)(input),
                        "login" => map(PacketLogin::deserialize, Params::Login)(input),
                        "map" => map(PacketMap::deserialize, Params::Map)(input),
                        "trade_list" => map(PacketTradeList::deserialize, Params::TradeList)(input),
                        "rel_entity_move" => map(PacketRelEntityMove::deserialize, Params::RelEntityMove)(input),
                        "entity_move_look" => map(PacketEntityMoveLook::deserialize, Params::EntityMoveLook)(input),
                        "entity_look" => map(PacketEntityLook::deserialize, Params::EntityLook)(input),
                        "vehicle_move" => map(PacketVehicleMove::deserialize, Params::VehicleMove)(input),
                        "open_book" => map(PacketOpenBook::deserialize, Params::OpenBook)(input),
                        "open_sign_entity" => map(PacketOpenSignEntity::deserialize, Params::OpenSignEntity)(input),
                        "craft_recipe_response" => map(PacketCraftRecipeResponse::deserialize, Params::CraftRecipeResponse)(input),
                        "abilities" => map(PacketAbilities::deserialize, Params::Abilities)(input),
                        "end_combat_event" => map(PacketEndCombatEvent::deserialize, Params::EndCombatEvent)(input),
                        "enter_combat_event" => map(PacketEnterCombatEvent::deserialize, Params::EnterCombatEvent)(input),
                        "death_combat_event" => map(PacketDeathCombatEvent::deserialize, Params::DeathCombatEvent)(input),
                        "player_info" => map(PacketPlayerInfo::deserialize, Params::PlayerInfo)(input),
                        "position" => map(PacketPosition::deserialize, Params::Position)(input),
                        "unlock_recipes" => map(PacketUnlockRecipes::deserialize, Params::UnlockRecipes)(input),
                        "entity_destroy" => map(PacketEntityDestroy::deserialize, Params::EntityDestroy)(input),
                        "remove_entity_effect" => map(PacketRemoveEntityEffect::deserialize, Params::RemoveEntityEffect)(input),
                        "resource_pack_send" => map(PacketResourcePackSend::deserialize, Params::ResourcePackSend)(input),
                        "respawn" => map(PacketRespawn::deserialize, Params::Respawn)(input),
                        "entity_update_attributes" => map(PacketEntityUpdateAttributes::deserialize, Params::EntityUpdateAttributes)(input),
                        "camera" => map(PacketCamera::deserialize, Params::Camera)(input),
                        "held_item_slot" => map(PacketHeldItemSlot::deserialize, Params::HeldItemSlot)(input),
                        "update_view_position" => map(PacketUpdateViewPosition::deserialize, Params::UpdateViewPosition)(input),
                        "update_view_distance" => map(PacketUpdateViewDistance::deserialize, Params::UpdateViewDistance)(input),
                        "scoreboard_display_objective" => map(PacketScoreboardDisplayObjective::deserialize, Params::ScoreboardDisplayObjective)(input),
                        "entity_metadata" => map(PacketEntityMetadata::deserialize, Params::EntityMetadata)(input),
                        "attach_entity" => map(PacketAttachEntity::deserialize, Params::AttachEntity)(input),
                        "entity_velocity" => map(PacketEntityVelocity::deserialize, Params::EntityVelocity)(input),
                        "entity_equipment" => map(PacketEntityEquipment::deserialize, Params::EntityEquipment)(input),
                        "experience" => map(PacketExperience::deserialize, Params::Experience)(input),
                        "update_health" => map(PacketUpdateHealth::deserialize, Params::UpdateHealth)(input),
                        "scoreboard_objective" => map(PacketScoreboardObjective::deserialize, Params::ScoreboardObjective)(input),
                        "set_passengers" => map(PacketSetPassengers::deserialize, Params::SetPassengers)(input),
                        "teams" => map(PacketTeams::deserialize, Params::Teams)(input),
                        "scoreboard_score" => map(PacketScoreboardScore::deserialize, Params::ScoreboardScore)(input),
                        "simulation_distance" => map(PacketSimulationDistance::deserialize, Params::SimulationDistance)(input),
                        "spawn_position" => map(PacketSpawnPosition::deserialize, Params::SpawnPosition)(input),
                        "update_time" => map(PacketUpdateTime::deserialize, Params::UpdateTime)(input),
                        "entity_sound_effect" => map(PacketEntitySoundEffect::deserialize, Params::EntitySoundEffect)(input),
                        "stop_sound" => map(PacketStopSound::deserialize, Params::StopSound)(input),
                        "sound_effect" => map(PacketSoundEffect::deserialize, Params::SoundEffect)(input),
                        "playerlist_header" => map(PacketPlayerlistHeader::deserialize, Params::PlayerlistHeader)(input),
                        "collect" => map(PacketCollect::deserialize, Params::Collect)(input),
                        "entity_teleport" => map(PacketEntityTeleport::deserialize, Params::EntityTeleport)(input),
                        "entity_head_rotation" => map(PacketEntityHeadRotation::deserialize, Params::EntityHeadRotation)(input),
                        "entity_effect" => map(PacketEntityEffect::deserialize, Params::EntityEffect)(input),
                        "select_advancement_tab" => map(PacketSelectAdvancementTab::deserialize, Params::SelectAdvancementTab)(input),
                        "declare_recipes" => map(PacketDeclareRecipes::deserialize, Params::DeclareRecipes)(input),
                        "tags" => map(PacketTags::deserialize, Params::Tags)(input),
                        "acknowledge_player_digging" => map(PacketAcknowledgePlayerDigging::deserialize, Params::AcknowledgePlayerDigging)(input),
                        "sculk_vibration_signal" => map(PacketSculkVibrationSignal::deserialize, Params::SculkVibrationSignal)(input),
                        "clear_titles" => map(PacketClearTitles::deserialize, Params::ClearTitles)(input),
                        "initialize_world_border" => map(PacketInitializeWorldBorder::deserialize, Params::InitializeWorldBorder)(input),
                        "action_bar" => map(PacketActionBar::deserialize, Params::ActionBar)(input),
                        "world_border_center" => map(PacketWorldBorderCenter::deserialize, Params::WorldBorderCenter)(input),
                        "world_border_lerp_size" => map(PacketWorldBorderLerpSize::deserialize, Params::WorldBorderLerpSize)(input),
                        "world_border_size" => map(PacketWorldBorderSize::deserialize, Params::WorldBorderSize)(input),
                        "world_border_warning_delay" => map(PacketWorldBorderWarningDelay::deserialize, Params::WorldBorderWarningDelay)(input),
                        "world_border_warning_reach" => map(PacketWorldBorderWarningReach::deserialize, Params::WorldBorderWarningReach)(input),
                        "ping" => map(PacketPing::deserialize, Params::Ping)(input),
                        "set_title_subtitle" => map(PacketSetTitleSubtitle::deserialize, Params::SetTitleSubtitle)(input),
                        "set_title_text" => map(PacketSetTitleText::deserialize, Params::SetTitleText)(input),
                        "set_title_time" => map(PacketSetTitleTime::deserialize, Params::SetTitleTime)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        pub struct PacketTeleportConfirm {
            teleport_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketTeleportConfirm {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.teleport_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(teleport_id,)| PacketTeleportConfirm { teleport_id }))(input)
            }
        }

        pub struct PacketQueryBlockNbt {
            transaction_id: VarInt,
            location: Position,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketQueryBlockNbt {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = Position::serialize(&self.location, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, Position::deserialize)), |(transaction_id, location)| PacketQueryBlockNbt {
                    transaction_id,
                    location,
                }))(input)
            }
        }

        pub struct PacketSetDifficulty {
            new_difficulty: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetDifficulty {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.new_difficulty, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize,)), |(new_difficulty,)| PacketSetDifficulty { new_difficulty }))(input)
            }
        }

        pub struct PacketEditBook<'a> {
            hand: VarInt,
            pages: VarStringArray<'a>,
            title: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEditBook<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;

                let w = PrefixedArray::<VarString<'a>, VarInt>::len(&self.pages).serialize(w)?;

                let mut w = w;
                let items = self.pages.0.iter();
                for i in items {
                    w = PrefixedString::<'a, VarInt>::serialize(&i, w)?
                }

                let w = Option::<VarString<'a>>::serialize(&self.title, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, PrefixedArray::<VarString<'a>, VarInt>::deserialize, Option::<VarString<'a>>::deserialize)),
                    |(hand, pages, title)| PacketEditBook { hand, pages, title },
                ))(input)
            }
        }

        pub struct PacketQueryEntityNbt {
            transaction_id: VarInt,
            entity_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketQueryEntityNbt {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = VarInt::serialize(&self.entity_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, VarInt::deserialize)), |(transaction_id, entity_id)| PacketQueryEntityNbt {
                    transaction_id,
                    entity_id,
                }))(input)
            }
        }

        pub struct PacketPickItem {
            slot: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPickItem {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.slot, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(slot,)| PacketPickItem { slot }))(input)
            }
        }

        pub struct PacketNameItem<'a> {
            name: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketNameItem<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(name,)| PacketNameItem { name }))(input)
            }
        }

        pub struct PacketSelectTrade {
            slot: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSelectTrade {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.slot, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(slot,)| PacketSelectTrade { slot }))(input)
            }
        }

        pub struct PacketSetBeaconEffect {
            primary_effect: VarInt,
            secondary_effect: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetBeaconEffect {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.primary_effect, w)?;
                let w = VarInt::serialize(&self.secondary_effect, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, VarInt::deserialize)), |(primary_effect, secondary_effect)| PacketSetBeaconEffect {
                    primary_effect,
                    secondary_effect,
                }))(input)
            }
        }

        pub struct PacketUpdateCommandBlock<'a> {
            location: Position,
            command: VarString<'a>,
            mode: VarInt,
            flags: u8,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUpdateCommandBlock<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.command, w)?;
                let w = VarInt::serialize(&self.mode, w)?;
                let w = u8::serialize(&self.flags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((Position::deserialize, PrefixedString::<'a, VarInt>::deserialize, VarInt::deserialize, u8::deserialize)),
                    |(location, command, mode, flags)| PacketUpdateCommandBlock { location, command, mode, flags },
                ))(input)
            }
        }

        pub struct PacketUpdateCommandBlockMinecart<'a> {
            entity_id: VarInt,
            command: VarString<'a>,
            track_output: bool,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUpdateCommandBlockMinecart<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.command, w)?;
                let w = bool::serialize(&self.track_output, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((VarInt::deserialize, PrefixedString::<'a, VarInt>::deserialize, bool::deserialize)),
                    |(entity_id, command, track_output)| PacketUpdateCommandBlockMinecart { entity_id, command, track_output },
                ))(input)
            }
        }

        pub struct PacketUpdateStructureBlock<'a> {
            location: Position,
            action: VarInt,
            mode: VarInt,
            name: VarString<'a>,
            offset_x: i8,
            offset_y: i8,
            offset_z: i8,
            size_x: i8,
            size_y: i8,
            size_z: i8,
            mirror: VarInt,
            rotation: VarInt,
            metadata: VarString<'a>,
            integrity: f32,
            seed: VarLong,
            flags: u8,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUpdateStructureBlock<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.action, w)?;
                let w = VarInt::serialize(&self.mode, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = i8::serialize(&self.offset_x, w)?;
                let w = i8::serialize(&self.offset_y, w)?;
                let w = i8::serialize(&self.offset_z, w)?;
                let w = i8::serialize(&self.size_x, w)?;
                let w = i8::serialize(&self.size_y, w)?;
                let w = i8::serialize(&self.size_z, w)?;
                let w = VarInt::serialize(&self.mirror, w)?;
                let w = VarInt::serialize(&self.rotation, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.metadata, w)?;
                let w = f32::serialize(&self.integrity, w)?;
                let w = VarLong::serialize(&self.seed, w)?;
                let w = u8::serialize(&self.flags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        Position::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        f32::deserialize,
                        VarLong::deserialize,
                        u8::deserialize,
                    )),
                    |(location, action, mode, name, offset_x, offset_y, offset_z, size_x, size_y, size_z, mirror, rotation, metadata, integrity, seed, flags)| PacketUpdateStructureBlock {
                        location,
                        action,
                        mode,
                        name,
                        offset_x,
                        offset_y,
                        offset_z,
                        size_x,
                        size_y,
                        size_z,
                        mirror,
                        rotation,
                        metadata,
                        integrity,
                        seed,
                        flags,
                    },
                ))(input)
            }
        }

        pub struct PacketTabComplete<'a> {
            transaction_id: VarInt,
            text: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketTabComplete<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, PrefixedString::<'a, VarInt>::deserialize)), |(transaction_id, text)| PacketTabComplete {
                    transaction_id,
                    text,
                }))(input)
            }
        }

        pub struct PacketChat<'a> {
            message: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketChat<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.message, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(message,)| PacketChat { message }))(input)
            }
        }

        pub struct PacketClientCommand {
            action_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketClientCommand {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(action_id,)| PacketClientCommand { action_id }))(input)
            }
        }

        pub struct PacketSettings<'a> {
            locale: VarString<'a>,
            view_distance: i8,
            chat_flags: VarInt,
            chat_colors: bool,
            skin_parts: u8,
            main_hand: VarInt,
            enable_text_filtering: bool,
            enable_server_listing: bool,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSettings<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.locale, w)?;
                let w = i8::serialize(&self.view_distance, w)?;
                let w = VarInt::serialize(&self.chat_flags, w)?;
                let w = bool::serialize(&self.chat_colors, w)?;
                let w = u8::serialize(&self.skin_parts, w)?;
                let w = VarInt::serialize(&self.main_hand, w)?;
                let w = bool::serialize(&self.enable_text_filtering, w)?;
                let w = bool::serialize(&self.enable_server_listing, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        i8::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        u8::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(locale, view_distance, chat_flags, chat_colors, skin_parts, main_hand, enable_text_filtering, enable_server_listing)| PacketSettings {
                        locale,
                        view_distance,
                        chat_flags,
                        chat_colors,
                        skin_parts,
                        main_hand,
                        enable_text_filtering,
                        enable_server_listing,
                    },
                ))(input)
            }
        }

        pub struct PacketEnchantItem {
            window_id: i8,
            enchantment: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEnchantItem {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = i8::serialize(&self.enchantment, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i8::deserialize, i8::deserialize)), |(window_id, enchantment)| PacketEnchantItem { window_id, enchantment }))(input)
            }
        }

        pub struct ChangedSlot {
            location: i16,
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for ChangedSlot {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i16::serialize(&self.location, w)?;
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i16::deserialize, Slot::deserialize)), |(location, item)| ChangedSlot { location, item }))(input)
            }
        }

        pub struct PacketWindowClick {
            window_id: u8,
            state_id: VarInt,
            slot: i16,
            mouse_button: i8,
            mode: VarInt,
            changed_slots: VarArray<ChangedSlot>,
            cursor_item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWindowClick {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.state_id, w)?;
                let w = i16::serialize(&self.slot, w)?;
                let w = i8::serialize(&self.mouse_button, w)?;
                let w = VarInt::serialize(&self.mode, w)?;

                let w = PrefixedArray::<ChangedSlot, VarInt>::len(&self.changed_slots).serialize(w)?;

                let mut w = w;
                let items = self.changed_slots.0.iter();
                for i in items {
                    w = ChangedSlot::serialize(&i, w)?
                }

                let w = Slot::serialize(&self.cursor_item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        u8::deserialize,
                        VarInt::deserialize,
                        i16::deserialize,
                        i8::deserialize,
                        VarInt::deserialize,
                        PrefixedArray::<ChangedSlot, VarInt>::deserialize,
                        Slot::deserialize,
                    )),
                    |(window_id, state_id, slot, mouse_button, mode, changed_slots, cursor_item)| PacketWindowClick {
                        window_id,
                        state_id,
                        slot,
                        mouse_button,
                        mode,
                        changed_slots,
                        cursor_item,
                    },
                ))(input)
            }
        }

        pub struct PacketCloseWindow {
            window_id: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCloseWindow {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((u8::deserialize,)), |(window_id,)| PacketCloseWindow { window_id }))(input)
            }
        }

        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCustomPayload<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.channel, w)?;
                let w = RestBuffer::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize, RestBuffer::deserialize)), |(channel, data)| PacketCustomPayload {
                    channel,
                    data,
                }))(input)
            }
        }

        pub enum X {
            X2(f32),
            Default,
        }

        impl X {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    X::X2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    X::X2(val) => f32::serialize(&val, w)?,
                    X::Default => w,
                };

                Ok(w)
            }
        }
        pub enum UseEntityY {
            UseEntityY2(f32),
            Default,
        }

        impl UseEntityY {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    UseEntityY::UseEntityY2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    UseEntityY::UseEntityY2(val) => f32::serialize(&val, w)?,
                    UseEntityY::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Z {
            Z2(f32),
            Default,
        }

        impl Z {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Z::Z2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Z::Z2(val) => f32::serialize(&val, w)?,
                    Z::Default => w,
                };

                Ok(w)
            }
        }
        pub enum UseEntityHand {
            UseEntityHand0(VarInt),
            UseEntityHand2(VarInt),
            Default,
        }

        impl UseEntityHand {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    UseEntityHand::UseEntityHand0(_) => "0",
                    UseEntityHand::UseEntityHand2(_) => "2",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    UseEntityHand::UseEntityHand0(val) => VarInt::serialize(&val, w)?,
                    UseEntityHand::UseEntityHand2(val) => VarInt::serialize(&val, w)?,
                    UseEntityHand::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketUseEntity {
            target: VarInt,
            mouse: VarInt,
            x: X,
            y: UseEntityY,
            z: Z,
            hand: UseEntityHand,
            sneaking: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUseEntity {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.target, w)?;
                let w = VarInt::serialize(&self.mouse, w)?;
                let w = X::serialize(&self.x, w)?;
                let w = UseEntityY::serialize(&self.y, w)?;
                let w = Z::serialize(&self.z, w)?;
                let w = UseEntityHand::serialize(&self.hand, w)?;
                let w = bool::serialize(&self.sneaking, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_target) = (VarInt::deserialize)(input)?;
                    let (input, self_mouse) = (VarInt::deserialize)(input)?;
                    let (input, self_x) = (|input| match &format!("{}", self_mouse)[..] {
                        "2" => map(f32::deserialize, X::X2)(input),
                        _ => Ok((input, X::Default)),
                    })(input)?;
                    let (input, self_y) = (|input| match &format!("{}", self_mouse)[..] {
                        "2" => map(f32::deserialize, UseEntityY::UseEntityY2)(input),
                        _ => Ok((input, UseEntityY::Default)),
                    })(input)?;
                    let (input, self_z) = (|input| match &format!("{}", self_mouse)[..] {
                        "2" => map(f32::deserialize, Z::Z2)(input),
                        _ => Ok((input, Z::Default)),
                    })(input)?;
                    let (input, self_hand) = (|input| match &format!("{}", self_mouse)[..] {
                        "0" => map(VarInt::deserialize, UseEntityHand::UseEntityHand0)(input),
                        "2" => map(VarInt::deserialize, UseEntityHand::UseEntityHand2)(input),
                        _ => Ok((input, UseEntityHand::Default)),
                    })(input)?;
                    let (input, self_sneaking) = (bool::deserialize)(input)?;
                    Ok((
                        input,
                        PacketUseEntity {
                            target: self_target,
                            mouse: self_mouse,
                            x: self_x,
                            y: self_y,
                            z: self_z,
                            hand: self_hand,
                            sneaking: self_sneaking,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketGenerateStructure {
            location: Position,
            levels: VarInt,
            keep_jigsaws: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketGenerateStructure {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.levels, w)?;
                let w = bool::serialize(&self.keep_jigsaws, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Position::deserialize, VarInt::deserialize, bool::deserialize)), |(location, levels, keep_jigsaws)| {
                    PacketGenerateStructure { location, levels, keep_jigsaws }
                }))(input)
            }
        }

        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketKeepAlive {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.keep_alive_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i64::deserialize,)), |(keep_alive_id,)| PacketKeepAlive { keep_alive_id }))(input)
            }
        }

        pub struct PacketLockDifficulty {
            locked: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketLockDifficulty {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.locked, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((bool::deserialize,)), |(locked,)| PacketLockDifficulty { locked }))(input)
            }
        }

        pub struct PacketPosition {
            x: f64,
            y: f64,
            z: f64,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPosition {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f64::deserialize, f64::deserialize, f64::deserialize, bool::deserialize)), |(x, y, z, on_ground)| {
                    PacketPosition { x, y, z, on_ground }
                }))(input)
            }
        }

        pub struct PacketPositionLook {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPositionLook {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((f64::deserialize, f64::deserialize, f64::deserialize, f32::deserialize, f32::deserialize, bool::deserialize)),
                    |(x, y, z, yaw, pitch, on_ground)| PacketPositionLook { x, y, z, yaw, pitch, on_ground },
                ))(input)
            }
        }

        pub struct PacketLook {
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketLook {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f32::deserialize, f32::deserialize, bool::deserialize)), |(yaw, pitch, on_ground)| PacketLook {
                    yaw,
                    pitch,
                    on_ground,
                }))(input)
            }
        }

        pub struct PacketFlying {
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketFlying {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((bool::deserialize,)), |(on_ground,)| PacketFlying { on_ground }))(input)
            }
        }

        pub struct PacketVehicleMove {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketVehicleMove {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((f64::deserialize, f64::deserialize, f64::deserialize, f32::deserialize, f32::deserialize)),
                    |(x, y, z, yaw, pitch)| PacketVehicleMove { x, y, z, yaw, pitch },
                ))(input)
            }
        }

        pub struct PacketSteerBoat {
            left_paddle: bool,
            right_paddle: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSteerBoat {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.left_paddle, w)?;
                let w = bool::serialize(&self.right_paddle, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((bool::deserialize, bool::deserialize)), |(left_paddle, right_paddle)| PacketSteerBoat {
                    left_paddle,
                    right_paddle,
                }))(input)
            }
        }

        pub struct PacketCraftRecipeRequest<'a> {
            window_id: i8,
            recipe: VarString<'a>,
            make_all: bool,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCraftRecipeRequest<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe, w)?;
                let w = bool::serialize(&self.make_all, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((i8::deserialize, PrefixedString::<'a, VarInt>::deserialize, bool::deserialize)),
                    |(window_id, recipe, make_all)| PacketCraftRecipeRequest { window_id, recipe, make_all },
                ))(input)
            }
        }

        pub struct PacketAbilities {
            flags: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAbilities {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.flags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i8::deserialize,)), |(flags,)| PacketAbilities { flags }))(input)
            }
        }

        pub struct PacketBlockDig {
            status: VarInt,
            location: Position,
            face: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockDig {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.status, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = i8::serialize(&self.face, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, Position::deserialize, i8::deserialize)), |(status, location, face)| PacketBlockDig {
                    status,
                    location,
                    face,
                }))(input)
            }
        }

        pub struct PacketEntityAction {
            entity_id: VarInt,
            action_id: VarInt,
            jump_boost: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityAction {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = VarInt::serialize(&self.action_id, w)?;
                let w = VarInt::serialize(&self.jump_boost, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, VarInt::deserialize, VarInt::deserialize)), |(entity_id, action_id, jump_boost)| {
                    PacketEntityAction { entity_id, action_id, jump_boost }
                }))(input)
            }
        }

        pub struct PacketSteerVehicle {
            sideways: f32,
            forward: f32,
            jump: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSteerVehicle {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.sideways, w)?;
                let w = f32::serialize(&self.forward, w)?;
                let w = u8::serialize(&self.jump, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((f32::deserialize, f32::deserialize, u8::deserialize)), |(sideways, forward, jump)| PacketSteerVehicle {
                    sideways,
                    forward,
                    jump,
                }))(input)
            }
        }

        pub struct PacketDisplayedRecipe<'a> {
            recipe_id: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDisplayedRecipe<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((PrefixedString::<'a, VarInt>::deserialize,)), |(recipe_id,)| PacketDisplayedRecipe { recipe_id }))(input)
            }
        }

        pub struct PacketRecipeBook {
            book_id: VarInt,
            book_open: bool,
            filter_active: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketRecipeBook {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.book_id, w)?;
                let w = bool::serialize(&self.book_open, w)?;
                let w = bool::serialize(&self.filter_active, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize, bool::deserialize, bool::deserialize)), |(book_id, book_open, filter_active)| {
                    PacketRecipeBook { book_id, book_open, filter_active }
                }))(input)
            }
        }

        pub struct PacketResourcePackReceive {
            result: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketResourcePackReceive {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(result,)| PacketResourcePackReceive { result }))(input)
            }
        }

        pub struct PacketHeldItemSlot {
            slot_id: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketHeldItemSlot {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i16::serialize(&self.slot_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i16::deserialize,)), |(slot_id,)| PacketHeldItemSlot { slot_id }))(input)
            }
        }

        pub struct PacketSetCreativeSlot {
            slot: i16,
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetCreativeSlot {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i16::serialize(&self.slot, w)?;
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i16::deserialize, Slot::deserialize)), |(slot, item)| PacketSetCreativeSlot { slot, item }))(input)
            }
        }

        pub struct PacketUpdateJigsawBlock<'a> {
            location: Position,
            name: VarString<'a>,
            target: VarString<'a>,
            pool: VarString<'a>,
            final_state: VarString<'a>,
            joint_type: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUpdateJigsawBlock<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.target, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.pool, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.final_state, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.joint_type, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        Position::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(location, name, target, pool, final_state, joint_type)| PacketUpdateJigsawBlock {
                        location,
                        name,
                        target,
                        pool,
                        final_state,
                        joint_type,
                    },
                ))(input)
            }
        }

        pub struct PacketUpdateSign<'a> {
            location: Position,
            text1: VarString<'a>,
            text2: VarString<'a>,
            text3: VarString<'a>,
            text4: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUpdateSign<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text1, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text2, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text3, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text4, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        Position::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(location, text1, text2, text3, text4)| PacketUpdateSign { location, text1, text2, text3, text4 },
                ))(input)
            }
        }

        pub struct PacketArmAnimation {
            hand: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketArmAnimation {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(hand,)| PacketArmAnimation { hand }))(input)
            }
        }

        pub struct PacketSpectate {
            target: Uuid,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSpectate {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.target, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((Uuid::deserialize,)), |(target,)| PacketSpectate { target }))(input)
            }
        }

        pub struct PacketBlockPlace {
            hand: VarInt,
            location: Position,
            direction: VarInt,
            cursor_x: f32,
            cursor_y: f32,
            cursor_z: f32,
            inside_block: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockPlace {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.direction, w)?;
                let w = f32::serialize(&self.cursor_x, w)?;
                let w = f32::serialize(&self.cursor_y, w)?;
                let w = f32::serialize(&self.cursor_z, w)?;
                let w = bool::serialize(&self.inside_block, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(
                    tuple((
                        VarInt::deserialize,
                        Position::deserialize,
                        VarInt::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        bool::deserialize,
                    )),
                    |(hand, location, direction, cursor_x, cursor_y, cursor_z, inside_block)| PacketBlockPlace {
                        hand,
                        location,
                        direction,
                        cursor_x,
                        cursor_y,
                        cursor_z,
                        inside_block,
                    },
                ))(input)
            }
        }

        pub struct PacketUseItem {
            hand: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUseItem {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((VarInt::deserialize,)), |(hand,)| PacketUseItem { hand }))(input)
            }
        }

        pub enum TabId<'a> {
            TabId0(VarString<'a>),
            TabId1,
            Default,
        }

        impl<'a> TabId<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    TabId::TabId0(_) => "0",
                    TabId::TabId1 => "1",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    TabId::TabId0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    TabId::TabId1 => w,
                    TabId::Default => w,
                };

                Ok(w)
            }
        }
        pub struct PacketAdvancementTab<'a> {
            action: VarInt,
            tab_id: TabId<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketAdvancementTab<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action, w)?;
                let w = TabId::serialize(&self.tab_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_tab_id) = (|input| match &format!("{}", self_action)[..] {
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, TabId::TabId0)(input),
                        "1" => Ok((input, TabId::TabId1)),
                        _ => Ok((input, TabId::Default)),
                    })(input)?;
                    Ok((
                        input,
                        PacketAdvancementTab {
                            action: self_action,
                            tab_id: self_tab_id,
                        },
                    ))
                })(input)
            }
        }

        pub struct PacketPong {
            id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPong {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (map(tuple((i32::deserialize,)), |(id,)| PacketPong { id }))(input)
            }
        }

        pub enum Params<'a> {
            TeleportConfirm(PacketTeleportConfirm),
            QueryBlockNbt(PacketQueryBlockNbt),
            SetDifficulty(PacketSetDifficulty),
            EditBook(PacketEditBook<'a>),
            QueryEntityNbt(PacketQueryEntityNbt),
            PickItem(PacketPickItem),
            NameItem(PacketNameItem<'a>),
            SelectTrade(PacketSelectTrade),
            SetBeaconEffect(PacketSetBeaconEffect),
            UpdateCommandBlock(PacketUpdateCommandBlock<'a>),
            UpdateCommandBlockMinecart(PacketUpdateCommandBlockMinecart<'a>),
            UpdateStructureBlock(PacketUpdateStructureBlock<'a>),
            TabComplete(PacketTabComplete<'a>),
            Chat(PacketChat<'a>),
            ClientCommand(PacketClientCommand),
            Settings(PacketSettings<'a>),
            EnchantItem(PacketEnchantItem),
            WindowClick(PacketWindowClick),
            CloseWindow(PacketCloseWindow),
            CustomPayload(PacketCustomPayload<'a>),
            UseEntity(PacketUseEntity),
            GenerateStructure(PacketGenerateStructure),
            KeepAlive(PacketKeepAlive),
            LockDifficulty(PacketLockDifficulty),
            Position(PacketPosition),
            PositionLook(PacketPositionLook),
            Look(PacketLook),
            Flying(PacketFlying),
            VehicleMove(PacketVehicleMove),
            SteerBoat(PacketSteerBoat),
            CraftRecipeRequest(PacketCraftRecipeRequest<'a>),
            Abilities(PacketAbilities),
            BlockDig(PacketBlockDig),
            EntityAction(PacketEntityAction),
            SteerVehicle(PacketSteerVehicle),
            DisplayedRecipe(PacketDisplayedRecipe<'a>),
            RecipeBook(PacketRecipeBook),
            ResourcePackReceive(PacketResourcePackReceive),
            HeldItemSlot(PacketHeldItemSlot),
            SetCreativeSlot(PacketSetCreativeSlot),
            UpdateJigsawBlock(PacketUpdateJigsawBlock<'a>),
            UpdateSign(PacketUpdateSign<'a>),
            ArmAnimation(PacketArmAnimation),
            Spectate(PacketSpectate),
            BlockPlace(PacketBlockPlace),
            UseItem(PacketUseItem),
            AdvancementTab(PacketAdvancementTab<'a>),
            Pong(PacketPong),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::TeleportConfirm(_) => "teleport_confirm",
                    Params::QueryBlockNbt(_) => "query_block_nbt",
                    Params::SetDifficulty(_) => "set_difficulty",
                    Params::EditBook(_) => "edit_book",
                    Params::QueryEntityNbt(_) => "query_entity_nbt",
                    Params::PickItem(_) => "pick_item",
                    Params::NameItem(_) => "name_item",
                    Params::SelectTrade(_) => "select_trade",
                    Params::SetBeaconEffect(_) => "set_beacon_effect",
                    Params::UpdateCommandBlock(_) => "update_command_block",
                    Params::UpdateCommandBlockMinecart(_) => "update_command_block_minecart",
                    Params::UpdateStructureBlock(_) => "update_structure_block",
                    Params::TabComplete(_) => "tab_complete",
                    Params::Chat(_) => "chat",
                    Params::ClientCommand(_) => "client_command",
                    Params::Settings(_) => "settings",
                    Params::EnchantItem(_) => "enchant_item",
                    Params::WindowClick(_) => "window_click",
                    Params::CloseWindow(_) => "close_window",
                    Params::CustomPayload(_) => "custom_payload",
                    Params::UseEntity(_) => "use_entity",
                    Params::GenerateStructure(_) => "generate_structure",
                    Params::KeepAlive(_) => "keep_alive",
                    Params::LockDifficulty(_) => "lock_difficulty",
                    Params::Position(_) => "position",
                    Params::PositionLook(_) => "position_look",
                    Params::Look(_) => "look",
                    Params::Flying(_) => "flying",
                    Params::VehicleMove(_) => "vehicle_move",
                    Params::SteerBoat(_) => "steer_boat",
                    Params::CraftRecipeRequest(_) => "craft_recipe_request",
                    Params::Abilities(_) => "abilities",
                    Params::BlockDig(_) => "block_dig",
                    Params::EntityAction(_) => "entity_action",
                    Params::SteerVehicle(_) => "steer_vehicle",
                    Params::DisplayedRecipe(_) => "displayed_recipe",
                    Params::RecipeBook(_) => "recipe_book",
                    Params::ResourcePackReceive(_) => "resource_pack_receive",
                    Params::HeldItemSlot(_) => "held_item_slot",
                    Params::SetCreativeSlot(_) => "set_creative_slot",
                    Params::UpdateJigsawBlock(_) => "update_jigsaw_block",
                    Params::UpdateSign(_) => "update_sign",
                    Params::ArmAnimation(_) => "arm_animation",
                    Params::Spectate(_) => "spectate",
                    Params::BlockPlace(_) => "block_place",
                    Params::UseItem(_) => "use_item",
                    Params::AdvancementTab(_) => "advancement_tab",
                    Params::Pong(_) => "pong",
                    _ => "",
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                use protocol_lib::Packet;

                let w = match &self {
                    Params::TeleportConfirm(val) => PacketTeleportConfirm::serialize(&val, w)?,
                    Params::QueryBlockNbt(val) => PacketQueryBlockNbt::serialize(&val, w)?,
                    Params::SetDifficulty(val) => PacketSetDifficulty::serialize(&val, w)?,
                    Params::EditBook(val) => PacketEditBook::serialize(&val, w)?,
                    Params::QueryEntityNbt(val) => PacketQueryEntityNbt::serialize(&val, w)?,
                    Params::PickItem(val) => PacketPickItem::serialize(&val, w)?,
                    Params::NameItem(val) => PacketNameItem::serialize(&val, w)?,
                    Params::SelectTrade(val) => PacketSelectTrade::serialize(&val, w)?,
                    Params::SetBeaconEffect(val) => PacketSetBeaconEffect::serialize(&val, w)?,
                    Params::UpdateCommandBlock(val) => PacketUpdateCommandBlock::serialize(&val, w)?,
                    Params::UpdateCommandBlockMinecart(val) => PacketUpdateCommandBlockMinecart::serialize(&val, w)?,
                    Params::UpdateStructureBlock(val) => PacketUpdateStructureBlock::serialize(&val, w)?,
                    Params::TabComplete(val) => PacketTabComplete::serialize(&val, w)?,
                    Params::Chat(val) => PacketChat::serialize(&val, w)?,
                    Params::ClientCommand(val) => PacketClientCommand::serialize(&val, w)?,
                    Params::Settings(val) => PacketSettings::serialize(&val, w)?,
                    Params::EnchantItem(val) => PacketEnchantItem::serialize(&val, w)?,
                    Params::WindowClick(val) => PacketWindowClick::serialize(&val, w)?,
                    Params::CloseWindow(val) => PacketCloseWindow::serialize(&val, w)?,
                    Params::CustomPayload(val) => PacketCustomPayload::serialize(&val, w)?,
                    Params::UseEntity(val) => PacketUseEntity::serialize(&val, w)?,
                    Params::GenerateStructure(val) => PacketGenerateStructure::serialize(&val, w)?,
                    Params::KeepAlive(val) => PacketKeepAlive::serialize(&val, w)?,
                    Params::LockDifficulty(val) => PacketLockDifficulty::serialize(&val, w)?,
                    Params::Position(val) => PacketPosition::serialize(&val, w)?,
                    Params::PositionLook(val) => PacketPositionLook::serialize(&val, w)?,
                    Params::Look(val) => PacketLook::serialize(&val, w)?,
                    Params::Flying(val) => PacketFlying::serialize(&val, w)?,
                    Params::VehicleMove(val) => PacketVehicleMove::serialize(&val, w)?,
                    Params::SteerBoat(val) => PacketSteerBoat::serialize(&val, w)?,
                    Params::CraftRecipeRequest(val) => PacketCraftRecipeRequest::serialize(&val, w)?,
                    Params::Abilities(val) => PacketAbilities::serialize(&val, w)?,
                    Params::BlockDig(val) => PacketBlockDig::serialize(&val, w)?,
                    Params::EntityAction(val) => PacketEntityAction::serialize(&val, w)?,
                    Params::SteerVehicle(val) => PacketSteerVehicle::serialize(&val, w)?,
                    Params::DisplayedRecipe(val) => PacketDisplayedRecipe::serialize(&val, w)?,
                    Params::RecipeBook(val) => PacketRecipeBook::serialize(&val, w)?,
                    Params::ResourcePackReceive(val) => PacketResourcePackReceive::serialize(&val, w)?,
                    Params::HeldItemSlot(val) => PacketHeldItemSlot::serialize(&val, w)?,
                    Params::SetCreativeSlot(val) => PacketSetCreativeSlot::serialize(&val, w)?,
                    Params::UpdateJigsawBlock(val) => PacketUpdateJigsawBlock::serialize(&val, w)?,
                    Params::UpdateSign(val) => PacketUpdateSign::serialize(&val, w)?,
                    Params::ArmAnimation(val) => PacketArmAnimation::serialize(&val, w)?,
                    Params::Spectate(val) => PacketSpectate::serialize(&val, w)?,
                    Params::BlockPlace(val) => PacketBlockPlace::serialize(&val, w)?,
                    Params::UseItem(val) => PacketUseItem::serialize(&val, w)?,
                    Params::AdvancementTab(val) => PacketAdvancementTab::serialize(&val, w)?,
                    Params::Pong(val) => PacketPong::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "teleport_confirm" => "0x00",
                    "query_block_nbt" => "0x01",
                    "set_difficulty" => "0x02",
                    "chat" => "0x03",
                    "client_command" => "0x04",
                    "settings" => "0x05",
                    "tab_complete" => "0x06",
                    "enchant_item" => "0x07",
                    "window_click" => "0x08",
                    "close_window" => "0x09",
                    "custom_payload" => "0x0a",
                    "edit_book" => "0x0b",
                    "query_entity_nbt" => "0x0c",
                    "use_entity" => "0x0d",
                    "generate_structure" => "0x0e",
                    "keep_alive" => "0x0f",
                    "lock_difficulty" => "0x10",
                    "position" => "0x11",
                    "position_look" => "0x12",
                    "look" => "0x13",
                    "flying" => "0x14",
                    "vehicle_move" => "0x15",
                    "steer_boat" => "0x16",
                    "pick_item" => "0x17",
                    "craft_recipe_request" => "0x18",
                    "abilities" => "0x19",
                    "block_dig" => "0x1a",
                    "entity_action" => "0x1b",
                    "steer_vehicle" => "0x1c",
                    "pong" => "0x1d",
                    "recipe_book" => "0x1e",
                    "displayed_recipe" => "0x1f",
                    "name_item" => "0x20",
                    "resource_pack_receive" => "0x21",
                    "advancement_tab" => "0x22",
                    "select_trade" => "0x23",
                    "set_beacon_effect" => "0x24",
                    "held_item_slot" => "0x25",
                    "update_command_block" => "0x26",
                    "update_command_block_minecart" => "0x27",
                    "set_creative_slot" => "0x28",
                    "update_jigsaw_block" => "0x29",
                    "update_structure_block" => "0x2a",
                    "update_sign" => "0x2b",
                    "arm_animation" => "0x2c",
                    "spectate" => "0x2d",
                    "block_place" => "0x2e",
                    "use_item" => "0x2f",

                    _ => panic!("invalid value"),
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|input| {
                        let (input, x) = (VarInt::deserialize)(input)?;
                        let x = format!("{x}");
                        let val = match &x[..] {
                            "0x00" => "teleport_confirm",
                            "0x01" => "query_block_nbt",
                            "0x02" => "set_difficulty",
                            "0x03" => "chat",
                            "0x04" => "client_command",
                            "0x05" => "settings",
                            "0x06" => "tab_complete",
                            "0x07" => "enchant_item",
                            "0x08" => "window_click",
                            "0x09" => "close_window",
                            "0x0a" => "custom_payload",
                            "0x0b" => "edit_book",
                            "0x0c" => "query_entity_nbt",
                            "0x0d" => "use_entity",
                            "0x0e" => "generate_structure",
                            "0x0f" => "keep_alive",
                            "0x10" => "lock_difficulty",
                            "0x11" => "position",
                            "0x12" => "position_look",
                            "0x13" => "look",
                            "0x14" => "flying",
                            "0x15" => "vehicle_move",
                            "0x16" => "steer_boat",
                            "0x17" => "pick_item",
                            "0x18" => "craft_recipe_request",
                            "0x19" => "abilities",
                            "0x1a" => "block_dig",
                            "0x1b" => "entity_action",
                            "0x1c" => "steer_vehicle",
                            "0x1d" => "pong",
                            "0x1e" => "recipe_book",
                            "0x1f" => "displayed_recipe",
                            "0x20" => "name_item",
                            "0x21" => "resource_pack_receive",
                            "0x22" => "advancement_tab",
                            "0x23" => "select_trade",
                            "0x24" => "set_beacon_effect",
                            "0x25" => "held_item_slot",
                            "0x26" => "update_command_block",
                            "0x27" => "update_command_block_minecart",
                            "0x28" => "set_creative_slot",
                            "0x29" => "update_jigsaw_block",
                            "0x2a" => "update_structure_block",
                            "0x2b" => "update_sign",
                            "0x2c" => "arm_animation",
                            "0x2d" => "spectate",
                            "0x2e" => "block_place",
                            "0x2f" => "use_item",

                            _ => return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))),
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "teleport_confirm" => map(PacketTeleportConfirm::deserialize, Params::TeleportConfirm)(input),
                        "query_block_nbt" => map(PacketQueryBlockNbt::deserialize, Params::QueryBlockNbt)(input),
                        "set_difficulty" => map(PacketSetDifficulty::deserialize, Params::SetDifficulty)(input),
                        "edit_book" => map(PacketEditBook::deserialize, Params::EditBook)(input),
                        "query_entity_nbt" => map(PacketQueryEntityNbt::deserialize, Params::QueryEntityNbt)(input),
                        "pick_item" => map(PacketPickItem::deserialize, Params::PickItem)(input),
                        "name_item" => map(PacketNameItem::deserialize, Params::NameItem)(input),
                        "select_trade" => map(PacketSelectTrade::deserialize, Params::SelectTrade)(input),
                        "set_beacon_effect" => map(PacketSetBeaconEffect::deserialize, Params::SetBeaconEffect)(input),
                        "update_command_block" => map(PacketUpdateCommandBlock::deserialize, Params::UpdateCommandBlock)(input),
                        "update_command_block_minecart" => map(PacketUpdateCommandBlockMinecart::deserialize, Params::UpdateCommandBlockMinecart)(input),
                        "update_structure_block" => map(PacketUpdateStructureBlock::deserialize, Params::UpdateStructureBlock)(input),
                        "tab_complete" => map(PacketTabComplete::deserialize, Params::TabComplete)(input),
                        "chat" => map(PacketChat::deserialize, Params::Chat)(input),
                        "client_command" => map(PacketClientCommand::deserialize, Params::ClientCommand)(input),
                        "settings" => map(PacketSettings::deserialize, Params::Settings)(input),
                        "enchant_item" => map(PacketEnchantItem::deserialize, Params::EnchantItem)(input),
                        "window_click" => map(PacketWindowClick::deserialize, Params::WindowClick)(input),
                        "close_window" => map(PacketCloseWindow::deserialize, Params::CloseWindow)(input),
                        "custom_payload" => map(PacketCustomPayload::deserialize, Params::CustomPayload)(input),
                        "use_entity" => map(PacketUseEntity::deserialize, Params::UseEntity)(input),
                        "generate_structure" => map(PacketGenerateStructure::deserialize, Params::GenerateStructure)(input),
                        "keep_alive" => map(PacketKeepAlive::deserialize, Params::KeepAlive)(input),
                        "lock_difficulty" => map(PacketLockDifficulty::deserialize, Params::LockDifficulty)(input),
                        "position" => map(PacketPosition::deserialize, Params::Position)(input),
                        "position_look" => map(PacketPositionLook::deserialize, Params::PositionLook)(input),
                        "look" => map(PacketLook::deserialize, Params::Look)(input),
                        "flying" => map(PacketFlying::deserialize, Params::Flying)(input),
                        "vehicle_move" => map(PacketVehicleMove::deserialize, Params::VehicleMove)(input),
                        "steer_boat" => map(PacketSteerBoat::deserialize, Params::SteerBoat)(input),
                        "craft_recipe_request" => map(PacketCraftRecipeRequest::deserialize, Params::CraftRecipeRequest)(input),
                        "abilities" => map(PacketAbilities::deserialize, Params::Abilities)(input),
                        "block_dig" => map(PacketBlockDig::deserialize, Params::BlockDig)(input),
                        "entity_action" => map(PacketEntityAction::deserialize, Params::EntityAction)(input),
                        "steer_vehicle" => map(PacketSteerVehicle::deserialize, Params::SteerVehicle)(input),
                        "displayed_recipe" => map(PacketDisplayedRecipe::deserialize, Params::DisplayedRecipe)(input),
                        "recipe_book" => map(PacketRecipeBook::deserialize, Params::RecipeBook)(input),
                        "resource_pack_receive" => map(PacketResourcePackReceive::deserialize, Params::ResourcePackReceive)(input),
                        "held_item_slot" => map(PacketHeldItemSlot::deserialize, Params::HeldItemSlot)(input),
                        "set_creative_slot" => map(PacketSetCreativeSlot::deserialize, Params::SetCreativeSlot)(input),
                        "update_jigsaw_block" => map(PacketUpdateJigsawBlock::deserialize, Params::UpdateJigsawBlock)(input),
                        "update_sign" => map(PacketUpdateSign::deserialize, Params::UpdateSign)(input),
                        "arm_animation" => map(PacketArmAnimation::deserialize, Params::ArmAnimation)(input),
                        "spectate" => map(PacketSpectate::deserialize, Params::Spectate)(input),
                        "block_place" => map(PacketBlockPlace::deserialize, Params::BlockPlace)(input),
                        "use_item" => map(PacketUseItem::deserialize, Params::UseItem)(input),
                        "advancement_tab" => map(PacketAdvancementTab::deserialize, Params::AdvancementTab)(input),
                        "pong" => map(PacketPong::deserialize, Params::Pong)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, Packet { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
