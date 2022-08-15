
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
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
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((
                parse_bits_signed(26),
                parse_bits_signed(26),
                parse_bits_signed(12),
            )),
            |(x, z, y)| Position { x, z, y },
        )))(input)
    }
}

pub struct RTrue {
    item_id: VarInt,
    item_count: i8,
    nbt_data: OptionalNbt,
}

impl<'t> protocol_lib::Packet<'t> for RTrue {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.item_id, w)?;
        let w = i8::serialize(&self.item_count, w)?;
        let w = OptionalNbt::serialize(&self.nbt_data, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((
                VarInt::deserialize,
                i8::deserialize,
                OptionalNbt::deserialize,
            )),
            |(item_id, item_count, nbt_data)| RTrue {
                item_id,
                item_count,
                nbt_data,
            },
        ))(input)
    }
}

pub enum Ident0 {
    RFalse(Void),
    RTrue(RTrue),
    Default(Void),
}

impl Ident0 {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Ident0::RFalse(_) => "false",
            Ident0::RTrue(_) => "true",
            _ => "",
        }
    }
}
pub struct Slot {
    present: bool,
    ident0: Ident0,
}

impl<'t> protocol_lib::Packet<'t> for Slot {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = bool::serialize(&self.present, w)?;

        let w = match &self.ident0 {
            Ident0::RFalse(val) => {
                let w = Void::serialize(&val, w)?;
                w
            }
            Ident0::RTrue(val) => {
                let w = RTrue::serialize(&val, w)?;
                w
            }
            Ident0::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_present) = (bool::deserialize)(input)?;
            let (input, self_ident0) = (|input| match &format!("{}", self_present)[..] {
                "false" => nom::combinator::map(Void::deserialize, Ident0::RFalse)(input),
                "true" => nom::combinator::map(RTrue::deserialize, Ident0::RTrue)(input),
                _ => nom::combinator::map(Void::deserialize, Ident0::Default)(input),
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.block_state, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((VarInt::deserialize,)),
            |(block_state,)| Data2 { block_state },
        ))(input)
    }
}

pub struct Data3 {
    block_state: VarInt,
}

impl<'t> protocol_lib::Packet<'t> for Data3 {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.block_state, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((VarInt::deserialize,)),
            |(block_state,)| Data3 { block_state },
        ))(input)
    }
}

pub struct Data14 {
    red: f32,
    green: f32,
    blue: f32,
    scale: f32,
}

impl<'t> protocol_lib::Packet<'t> for Data14 {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = f32::serialize(&self.red, w)?;
        let w = f32::serialize(&self.green, w)?;
        let w = f32::serialize(&self.blue, w)?;
        let w = f32::serialize(&self.scale, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((
                f32::deserialize,
                f32::deserialize,
                f32::deserialize,
                f32::deserialize,
            )),
            |(red, green, blue, scale)| Data14 {
                red,
                green,
                blue,
                scale,
            },
        ))(input)
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
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
        (nom::combinator::map(
            nom::sequence::tuple((
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.block_state, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((VarInt::deserialize,)),
            |(block_state,)| Data24 { block_state },
        ))(input)
    }
}

pub struct Data35 {
    item: Slot,
}

impl<'t> protocol_lib::Packet<'t> for Data35 {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = Slot::serialize(&self.item, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(nom::sequence::tuple((Slot::deserialize,)), |(item,)| {
            Data35 { item }
        }))(input)
    }
}

pub enum Destination {
    Block(Position),
    Entity(VarInt),
    Default(Void),
}

impl Destination {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Destination::Block(_) => "minecraft:block",
            Destination::Entity(_) => "minecraft:entity",
            _ => "",
        }
    }
}
pub struct Data36<'a> {
    origin: Position,
    position_type: VarString<'a>,
    destination: Destination,
    ticks: VarInt,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Data36<'a> {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = Position::serialize(&self.origin, w)?;
        let w = PrefixedString::<'a, VarInt>::serialize(&self.position_type, w)?;

        let w = match &self.destination {
            Destination::Block(val) => {
                let w = Position::serialize(&val, w)?;
                w
            }
            Destination::Entity(val) => {
                let w = VarInt::serialize(&val, w)?;
                w
            }
            Destination::Default(val) => Void::serialize(val, w)?,
        };

        let w = VarInt::serialize(&self.ticks, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_origin) = (Position::deserialize)(input)?;
            let (input, self_position_type) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
            let (input, self_destination) = (|input| match &format!("{}", self_position_type)[..] {
                "minecraft:block" => {
                    nom::combinator::map(Position::deserialize, Destination::Block)(input)
                }
                "minecraft:entity" => {
                    nom::combinator::map(VarInt::deserialize, Destination::Entity)(input)
                }
                _ => nom::combinator::map(Void::deserialize, Destination::Default)(input),
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
    Default(Void),
}

impl<'a> Data<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Data::<'a>::Data2(_) => "2",
            Data::<'a>::Data3(_) => "3",
            Data::<'a>::Data14(_) => "14",
            Data::<'a>::Data15(_) => "15",
            Data::<'a>::Data24(_) => "24",
            Data::<'a>::Data35(_) => "35",
            Data::<'a>::Data36(_) => "36",
            _ => "",
        }
    }
}
pub struct Particle<'a> {
    particle_id: VarInt,
    data: Data<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Particle<'a> {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.particle_id, w)?;

        let w = match &self.data {
            Data::<'a>::Data2(val) => {
                let w = Data2::serialize(&val, w)?;
                w
            }
            Data::<'a>::Data3(val) => {
                let w = Data3::serialize(&val, w)?;
                w
            }
            Data::<'a>::Data14(val) => {
                let w = Data14::serialize(&val, w)?;
                w
            }
            Data::<'a>::Data15(val) => {
                let w = Data15::serialize(&val, w)?;
                w
            }
            Data::<'a>::Data24(val) => {
                let w = Data24::serialize(&val, w)?;
                w
            }
            Data::<'a>::Data35(val) => {
                let w = Data35::serialize(&val, w)?;
                w
            }
            Data::<'a>::Data36(val) => {
                let w = Data36::<'a>::serialize(&val, w)?;
                w
            }
            Data::<'a>::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_particle_id) = (VarInt::deserialize)(input)?;
            let (input, self_data) = (|input| match &format!("{}", self_particle_id)[..] {
                "2" => nom::combinator::map(Data2::deserialize, Data::<'a>::Data2)(input),
                "3" => nom::combinator::map(Data3::deserialize, Data::<'a>::Data3)(input),
                "14" => nom::combinator::map(Data14::deserialize, Data::<'a>::Data14)(input),
                "15" => nom::combinator::map(Data15::deserialize, Data::<'a>::Data15)(input),
                "24" => nom::combinator::map(Data24::deserialize, Data::<'a>::Data24)(input),
                "35" => nom::combinator::map(Data35::deserialize, Data::<'a>::Data35)(input),
                "36" => nom::combinator::map(Data36::<'a>::deserialize, Data::<'a>::Data36)(input),
                _ => nom::combinator::map(Void::deserialize, Data::<'a>::Default)(input),
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = u8::serialize(&self.key, w)?;
        let w = VarInt::serialize(&self.r_type, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((u8::deserialize, VarInt::deserialize)),
            |(key, r_type)| Ident1 { key, r_type },
        ))(input)
    }
}

pub struct Value8 {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl<'t> protocol_lib::Packet<'t> for Value8 {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = f32::serialize(&self.pitch, w)?;
        let w = f32::serialize(&self.yaw, w)?;
        let w = f32::serialize(&self.roll, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((f32::deserialize, f32::deserialize, f32::deserialize)),
            |(pitch, yaw, roll)| Value8 { pitch, yaw, roll },
        ))(input)
    }
}

pub struct Value16 {
    villager_type: VarInt,
    villager_profession: VarInt,
    level: VarInt,
}

impl<'t> protocol_lib::Packet<'t> for Value16 {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = VarInt::serialize(&self.villager_type, w)?;
        let w = VarInt::serialize(&self.villager_profession, w)?;
        let w = VarInt::serialize(&self.level, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((
                VarInt::deserialize,
                VarInt::deserialize,
                VarInt::deserialize,
            )),
            |(villager_type, villager_profession, level)| Value16 {
                villager_type,
                villager_profession,
                level,
            },
        ))(input)
    }
}

pub enum Value<'a> {
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
    Default(Void),
}

impl<'a> Value<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Value::<'a>::Value0(_) => "0",
            Value::<'a>::Value1(_) => "1",
            Value::<'a>::Value2(_) => "2",
            Value::<'a>::Value3(_) => "3",
            Value::<'a>::Value4(_) => "4",
            Value::<'a>::Value5(_) => "5",
            Value::<'a>::Value6(_) => "6",
            Value::<'a>::Value7(_) => "7",
            Value::<'a>::Value8(_) => "8",
            Value::<'a>::Value9(_) => "9",
            Value::<'a>::Value10(_) => "10",
            Value::<'a>::Value11(_) => "11",
            Value::<'a>::Value12(_) => "12",
            Value::<'a>::Value13(_) => "13",
            Value::<'a>::Value14(_) => "14",
            Value::<'a>::Value15(_) => "15",
            Value::<'a>::Value16(_) => "16",
            Value::<'a>::Value17(_) => "17",
            Value::<'a>::Value18(_) => "18",
            _ => "",
        }
    }
}
pub struct EntityMetadata<'a> {
    key: u8,
    r_type: VarInt,
    value: Value<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for EntityMetadata<'a> {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = u8::serialize(&self.key, w)?;
        let w = VarInt::serialize(&self.r_type, w)?;

        let w = match &self.value {
            Value::<'a>::Value0(val) => {
                let w = i8::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value1(val) => {
                let w = VarInt::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value2(val) => {
                let w = f32::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value3(val) => {
                let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value4(val) => {
                let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value5(val) => {
                let w = Option::<VarString<'a>>::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value6(val) => {
                let w = Slot::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value7(val) => {
                let w = bool::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value8(val) => {
                let w = Value8::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value9(val) => {
                let w = Position::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value10(val) => {
                let w = Option::<Position>::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value11(val) => {
                let w = VarInt::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value12(val) => {
                let w = Option::<Uuid>::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value13(val) => {
                let w = VarInt::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value14(val) => {
                let w = Nbt::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value15(val) => {
                let w = Particle::<'a>::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value16(val) => {
                let w = Value16::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value17(val) => {
                let w = VarInt::serialize(&val, w)?;
                w
            }
            Value::<'a>::Value18(val) => {
                let w = VarInt::serialize(&val, w)?;
                w
            }
            Value::<'a>::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_key) = (u8::deserialize)(input)?;
            let (input, self_r_type) = (VarInt::deserialize)(input)?;
            let (input, self_value) = (|input| match &format!("{}", self_r_type)[..] {
                "0" => nom::combinator::map(i8::deserialize, Value::<'a>::Value0)(input),
                "1" => nom::combinator::map(VarInt::deserialize, Value::<'a>::Value1)(input),
                "2" => nom::combinator::map(f32::deserialize, Value::<'a>::Value2)(input),
                "3" => nom::combinator::map(
                    PrefixedString::<'a, VarInt>::deserialize,
                    Value::<'a>::Value3,
                )(input),
                "4" => nom::combinator::map(
                    PrefixedString::<'a, VarInt>::deserialize,
                    Value::<'a>::Value4,
                )(input),
                "5" => nom::combinator::map(
                    Option::<VarString<'a>>::deserialize,
                    Value::<'a>::Value5,
                )(input),
                "6" => nom::combinator::map(Slot::deserialize, Value::<'a>::Value6)(input),
                "7" => nom::combinator::map(bool::deserialize, Value::<'a>::Value7)(input),
                "8" => nom::combinator::map(Value8::deserialize, Value::<'a>::Value8)(input),
                "9" => nom::combinator::map(Position::deserialize, Value::<'a>::Value9)(input),
                "10" => {
                    nom::combinator::map(Option::<Position>::deserialize, Value::<'a>::Value10)(
                        input,
                    )
                }
                "11" => nom::combinator::map(VarInt::deserialize, Value::<'a>::Value11)(input),
                "12" => {
                    nom::combinator::map(Option::<Uuid>::deserialize, Value::<'a>::Value12)(input)
                }
                "13" => nom::combinator::map(VarInt::deserialize, Value::<'a>::Value13)(input),
                "14" => nom::combinator::map(Nbt::deserialize, Value::<'a>::Value14)(input),
                "15" => {
                    nom::combinator::map(Particle::<'a>::deserialize, Value::<'a>::Value15)(input)
                }
                "16" => nom::combinator::map(Value16::deserialize, Value::<'a>::Value16)(input),
                "17" => nom::combinator::map(VarInt::deserialize, Value::<'a>::Value17)(input),
                "18" => nom::combinator::map(VarInt::deserialize, Value::<'a>::Value18)(input),
                _ => nom::combinator::map(Void::deserialize, Value::<'a>::Default)(input),
            })(input)?;
            Ok((
                input,
                EntityMetadata {
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;
        let w = PrefixedArray::<Slot, VarInt>::serialize(&self.ingredient, w)?;
        let w = Slot::serialize(&self.result, w)?;
        let w = f32::serialize(&self.experience, w)?;
        let w = VarInt::serialize(&self.cook_time, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.tag_name, w)?;
        let w = PrefixedArray::<VarInt, VarInt>::serialize(&self.entries, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((
                PrefixedString::<'a, VarInt>::deserialize,
                PrefixedArray::<VarInt, VarInt>::deserialize,
            )),
            |(tag_name, entries)| Tag { tag_name, entries },
        ))(input)
    }
}

pub struct Ident5 {
    x: u8,
    z: u8,
}

impl<'t> protocol_lib::Packet<'t> for Ident5 {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.x as u64, 4), (self.z as u64, 4)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((parse_bits_unsigned(4), parse_bits_unsigned(4))),
            |(x, z)| Ident5 { x, z },
        )))(input)
    }
}

pub struct ChunkBlockEntity {
    ident5: Ident5,
    y: i16,
    r_type: VarInt,
    nbt_data: OptionalNbt,
}

impl<'t> protocol_lib::Packet<'t> for ChunkBlockEntity {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = Ident5::serialize(&self.ident5, w)?;
        let w = i16::serialize(&self.y, w)?;
        let w = VarInt::serialize(&self.r_type, w)?;
        let w = OptionalNbt::serialize(&self.nbt_data, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((
                Ident5::deserialize,
                i16::deserialize,
                VarInt::deserialize,
                OptionalNbt::deserialize,
            )),
            |(ident5, y, r_type, nbt_data)| ChunkBlockEntity {
                ident5,
                y,
                r_type,
                nbt_data,
            },
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
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
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((
                parse_bits_unsigned(3),
                parse_bits_unsigned(1),
                parse_bits_unsigned(1),
                parse_bits_unsigned(1),
                parse_bits_unsigned(2),
            )),
            |(
                unused,
                has_custom_suggestions,
                has_redirect_node,
                has_command,
                command_node_type,
            )| Flags {
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
    Default(Void),
}

impl RedirectNode {
    pub fn discriminant(&self) -> &'static str {
        match self {
            RedirectNode::RedirectNode1(_) => "1",
            _ => "",
        }
    }
}
pub struct ExtraNodeData1<'a> {
    name: VarString<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for ExtraNodeData1<'a> {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
            |(name,)| ExtraNodeData1 { name },
        ))(input)
    }
}

pub struct FloatFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

impl<'t> protocol_lib::Packet<'t> for FloatFlags {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[
                (self.unused as u64, 6),
                (self.max_present as u64, 1),
                (self.min_present as u64, 1),
            ],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((
                parse_bits_unsigned(6),
                parse_bits_unsigned(1),
                parse_bits_unsigned(1),
            )),
            |(unused, max_present, min_present)| FloatFlags {
                unused,
                max_present,
                min_present,
            },
        )))(input)
    }
}

pub enum Min {
    Min1(f32),
    Default(Void),
}

impl Min {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Min::Min1(_) => "1",
            _ => "",
        }
    }
}
pub enum Max {
    Max1(f32),
    Default(Void),
}

impl Max {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Max::Max1(_) => "1",
            _ => "",
        }
    }
}
pub struct Float {
    flags: FloatFlags,
    min: Min,
    max: Max,
}

impl<'t> protocol_lib::Packet<'t> for Float {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = FloatFlags::serialize(&self.flags, w)?;

        let w = match &self.min {
            Min::Min1(val) => {
                let w = f32::serialize(&val, w)?;
                w
            }
            Min::Default(val) => Void::serialize(val, w)?,
        };

        let w = match &self.max {
            Max::Max1(val) => {
                let w = f32::serialize(&val, w)?;
                w
            }
            Max::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (FloatFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => nom::combinator::map(f32::deserialize, Min::Min1)(input),
                _ => nom::combinator::map(Void::deserialize, Min::Default)(input),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => nom::combinator::map(f32::deserialize, Max::Max1)(input),
                _ => nom::combinator::map(Void::deserialize, Max::Default)(input),
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[
                (self.unused as u64, 6),
                (self.max_present as u64, 1),
                (self.min_present as u64, 1),
            ],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((
                parse_bits_unsigned(6),
                parse_bits_unsigned(1),
                parse_bits_unsigned(1),
            )),
            |(unused, max_present, min_present)| DoubleFlags {
                unused,
                max_present,
                min_present,
            },
        )))(input)
    }
}

pub enum DoubleMin {
    DoubleMin1(f64),
    Default(Void),
}

impl DoubleMin {
    pub fn discriminant(&self) -> &'static str {
        match self {
            DoubleMin::DoubleMin1(_) => "1",
            _ => "",
        }
    }
}
pub enum DoubleMax {
    DoubleMax1(f64),
    Default(Void),
}

impl DoubleMax {
    pub fn discriminant(&self) -> &'static str {
        match self {
            DoubleMax::DoubleMax1(_) => "1",
            _ => "",
        }
    }
}
pub struct Double {
    flags: DoubleFlags,
    min: DoubleMin,
    max: DoubleMax,
}

impl<'t> protocol_lib::Packet<'t> for Double {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = DoubleFlags::serialize(&self.flags, w)?;

        let w = match &self.min {
            DoubleMin::DoubleMin1(val) => {
                let w = f64::serialize(&val, w)?;
                w
            }
            DoubleMin::Default(val) => Void::serialize(val, w)?,
        };

        let w = match &self.max {
            DoubleMax::DoubleMax1(val) => {
                let w = f64::serialize(&val, w)?;
                w
            }
            DoubleMax::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (DoubleFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => nom::combinator::map(f64::deserialize, DoubleMin::DoubleMin1)(input),
                _ => nom::combinator::map(Void::deserialize, DoubleMin::Default)(input),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => nom::combinator::map(f64::deserialize, DoubleMax::DoubleMax1)(input),
                _ => nom::combinator::map(Void::deserialize, DoubleMax::Default)(input),
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[
                (self.unused as u64, 6),
                (self.max_present as u64, 1),
                (self.min_present as u64, 1),
            ],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((
                parse_bits_unsigned(6),
                parse_bits_unsigned(1),
                parse_bits_unsigned(1),
            )),
            |(unused, max_present, min_present)| IntegerFlags {
                unused,
                max_present,
                min_present,
            },
        )))(input)
    }
}

pub enum IntegerMin {
    IntegerMin1(i32),
    Default(Void),
}

impl IntegerMin {
    pub fn discriminant(&self) -> &'static str {
        match self {
            IntegerMin::IntegerMin1(_) => "1",
            _ => "",
        }
    }
}
pub enum IntegerMax {
    IntegerMax1(i32),
    Default(Void),
}

impl IntegerMax {
    pub fn discriminant(&self) -> &'static str {
        match self {
            IntegerMax::IntegerMax1(_) => "1",
            _ => "",
        }
    }
}
pub struct Integer {
    flags: IntegerFlags,
    min: IntegerMin,
    max: IntegerMax,
}

impl<'t> protocol_lib::Packet<'t> for Integer {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = IntegerFlags::serialize(&self.flags, w)?;

        let w = match &self.min {
            IntegerMin::IntegerMin1(val) => {
                let w = i32::serialize(&val, w)?;
                w
            }
            IntegerMin::Default(val) => Void::serialize(val, w)?,
        };

        let w = match &self.max {
            IntegerMax::IntegerMax1(val) => {
                let w = i32::serialize(&val, w)?;
                w
            }
            IntegerMax::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (IntegerFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => nom::combinator::map(i32::deserialize, IntegerMin::IntegerMin1)(input),
                _ => nom::combinator::map(Void::deserialize, IntegerMin::Default)(input),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => nom::combinator::map(i32::deserialize, IntegerMax::IntegerMax1)(input),
                _ => nom::combinator::map(Void::deserialize, IntegerMax::Default)(input),
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[
                (self.unused as u64, 6),
                (self.max_present as u64, 1),
                (self.min_present as u64, 1),
            ],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((
                parse_bits_unsigned(6),
                parse_bits_unsigned(1),
                parse_bits_unsigned(1),
            )),
            |(unused, max_present, min_present)| LongFlags {
                unused,
                max_present,
                min_present,
            },
        )))(input)
    }
}

pub enum LongMin {
    LongMin1(i64),
    Default(Void),
}

impl LongMin {
    pub fn discriminant(&self) -> &'static str {
        match self {
            LongMin::LongMin1(_) => "1",
            _ => "",
        }
    }
}
pub enum LongMax {
    LongMax1(i64),
    Default(Void),
}

impl LongMax {
    pub fn discriminant(&self) -> &'static str {
        match self {
            LongMax::LongMax1(_) => "1",
            _ => "",
        }
    }
}
pub struct Long {
    flags: LongFlags,
    min: LongMin,
    max: LongMax,
}

impl<'t> protocol_lib::Packet<'t> for Long {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = LongFlags::serialize(&self.flags, w)?;

        let w = match &self.min {
            LongMin::LongMin1(val) => {
                let w = i64::serialize(&val, w)?;
                w
            }
            LongMin::Default(val) => Void::serialize(val, w)?,
        };

        let w = match &self.max {
            LongMax::LongMax1(val) => {
                let w = i64::serialize(&val, w)?;
                w
            }
            LongMax::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (LongFlags::deserialize)(input)?;
            let (input, self_min) = (|input| match &format!("{}", self_flags.min_present)[..] {
                "1" => nom::combinator::map(i64::deserialize, LongMin::LongMin1)(input),
                _ => nom::combinator::map(Void::deserialize, LongMin::Default)(input),
            })(input)?;
            let (input, self_max) = (|input| match &format!("{}", self_flags.max_present)[..] {
                "1" => nom::combinator::map(i64::deserialize, LongMax::LongMax1)(input),
                _ => nom::combinator::map(Void::deserialize, LongMax::Default)(input),
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[
                (self.unused as u64, 6),
                (self.only_allow_players as u64, 1),
                (self.only_allow_entities as u64, 1),
            ],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((
                parse_bits_unsigned(6),
                parse_bits_unsigned(1),
                parse_bits_unsigned(1),
            )),
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
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = write_bits(
            &[(self.unused as u64, 7), (self.allow_multiple as u64, 1)],
            w,
        )?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
            nom::sequence::tuple((parse_bits_unsigned(7), parse_bits_unsigned(1))),
            |(unused, allow_multiple)| ScoreHolder {
                unused,
                allow_multiple,
            },
        )))(input)
    }
}

pub struct Range {
    allow_decimals: bool,
}

impl<'t> protocol_lib::Packet<'t> for Range {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = bool::serialize(&self.allow_decimals, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((bool::deserialize,)),
            |(allow_decimals,)| Range { allow_decimals },
        ))(input)
    }
}

pub struct ResourceOrTag<'a> {
    registry: VarString<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for ResourceOrTag<'a> {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.registry, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
            |(registry,)| ResourceOrTag { registry },
        ))(input)
    }
}

pub struct Resource<'a> {
    registry: VarString<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Resource<'a> {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = PrefixedString::<'a, VarInt>::serialize(&self.registry, w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::combinator::map(
            nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
            |(registry,)| Resource { registry },
        ))(input)
    }
}

pub enum Properties<'a> {
    Bool(Void),
    Float(Float),
    Double(Double),
    Integer(Integer),
    Long(Long),
    String(&'static str),
    MinecraftEntity(MinecraftEntity),
    GameProfile(Void),
    BlockPos(Void),
    ColumnPos(Void),
    Vec3(Void),
    Vec2(Void),
    BlockState(Void),
    BlockPredicate(Void),
    ItemStack(Void),
    ItemPredicate(Void),
    Color(Void),
    Component(Void),
    Message(Void),
    Nbt(Void),
    NbtPath(Void),
    Objective(Void),
    ObjectiveCriteria(Void),
    Operation(Void),
    Particle(Void),
    Angle(Void),
    Rotation(Void),
    ScoreboardSlot(Void),
    ScoreHolder(ScoreHolder),
    Swizzle(Void),
    Team(Void),
    ItemSlot(Void),
    ResourceLocation(Void),
    MobEffect(Void),
    Function(Void),
    EntityAnchor(Void),
    Range(Range),
    IntRange(Void),
    FloatRange(Void),
    ItemEnchantment(Void),
    EntitySummon(Void),
    Dimension(Void),
    NbtCompoundTag(Void),
    Time(Void),
    ResourceOrTag(ResourceOrTag<'a>),
    Resource(Resource<'a>),
    Uuid(Void),
    Default(Void),
}

impl<'a> Properties<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            Properties::<'a>::Bool(_) => "brigadier:bool",
            Properties::<'a>::Float(_) => "brigadier:float",
            Properties::<'a>::Double(_) => "brigadier:double",
            Properties::<'a>::Integer(_) => "brigadier:integer",
            Properties::<'a>::Long(_) => "brigadier:long",
            Properties::<'a>::String(_) => "brigadier:string",
            Properties::<'a>::MinecraftEntity(_) => "minecraft:entity",
            Properties::<'a>::GameProfile(_) => "minecraft:game_profile",
            Properties::<'a>::BlockPos(_) => "minecraft:block_pos",
            Properties::<'a>::ColumnPos(_) => "minecraft:column_pos",
            Properties::<'a>::Vec3(_) => "minecraft:vec3",
            Properties::<'a>::Vec2(_) => "minecraft:vec2",
            Properties::<'a>::BlockState(_) => "minecraft:block_state",
            Properties::<'a>::BlockPredicate(_) => "minecraft:block_predicate",
            Properties::<'a>::ItemStack(_) => "minecraft:item_stack",
            Properties::<'a>::ItemPredicate(_) => "minecraft:item_predicate",
            Properties::<'a>::Color(_) => "minecraft:color",
            Properties::<'a>::Component(_) => "minecraft:component",
            Properties::<'a>::Message(_) => "minecraft:message",
            Properties::<'a>::Nbt(_) => "minecraft:nbt",
            Properties::<'a>::NbtPath(_) => "minecraft:nbt_path",
            Properties::<'a>::Objective(_) => "minecraft:objective",
            Properties::<'a>::ObjectiveCriteria(_) => "minecraft:objective_criteria",
            Properties::<'a>::Operation(_) => "minecraft:operation",
            Properties::<'a>::Particle(_) => "minecraft:particle",
            Properties::<'a>::Angle(_) => "minecraft:angle",
            Properties::<'a>::Rotation(_) => "minecraft:rotation",
            Properties::<'a>::ScoreboardSlot(_) => "minecraft:scoreboard_slot",
            Properties::<'a>::ScoreHolder(_) => "minecraft:score_holder",
            Properties::<'a>::Swizzle(_) => "minecraft:swizzle",
            Properties::<'a>::Team(_) => "minecraft:team",
            Properties::<'a>::ItemSlot(_) => "minecraft:item_slot",
            Properties::<'a>::ResourceLocation(_) => "minecraft:resource_location",
            Properties::<'a>::MobEffect(_) => "minecraft:mob_effect",
            Properties::<'a>::Function(_) => "minecraft:function",
            Properties::<'a>::EntityAnchor(_) => "minecraft:entity_anchor",
            Properties::<'a>::Range(_) => "minecraft:range",
            Properties::<'a>::IntRange(_) => "minecraft:int_range",
            Properties::<'a>::FloatRange(_) => "minecraft:float_range",
            Properties::<'a>::ItemEnchantment(_) => "minecraft:item_enchantment",
            Properties::<'a>::EntitySummon(_) => "minecraft:entity_summon",
            Properties::<'a>::Dimension(_) => "minecraft:dimension",
            Properties::<'a>::NbtCompoundTag(_) => "minecraft:nbt_compound_tag",
            Properties::<'a>::Time(_) => "minecraft:time",
            Properties::<'a>::ResourceOrTag(_) => "minecraft:resource_or_tag",
            Properties::<'a>::Resource(_) => "minecraft:resource",
            Properties::<'a>::Uuid(_) => "minecraft:uuid",
            _ => "",
        }
    }
}
pub enum SuggestionType<'a> {
    SuggestionType1(VarString<'a>),
    Default(Void),
}

impl<'a> SuggestionType<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            SuggestionType::<'a>::SuggestionType1(_) => "1",
            _ => "",
        }
    }
}
pub struct ExtraNodeData2<'a> {
    name: VarString<'a>,
    parser: VarString<'a>,
    properties: Properties<'a>,
    suggestion_type: SuggestionType<'a>,
}
pub enum ExtraNodeData<'a> {
    ExtraNodeData0(Void),
    ExtraNodeData1(ExtraNodeData1<'a>),
    ExtraNodeData2(ExtraNodeData2<'a>),
    Default(Void),
}

impl<'a> ExtraNodeData<'a> {
    pub fn discriminant(&self) -> &'static str {
        match self {
            ExtraNodeData::<'a>::ExtraNodeData0(_) => "0",
            ExtraNodeData::<'a>::ExtraNodeData1(_) => "1",
            ExtraNodeData::<'a>::ExtraNodeData2(_) => "2",
            _ => "",
        }
    }
}
pub struct CommandNode<'a> {
    flags: Flags,
    children: VarArray<VarInt>,
    redirect_node: RedirectNode,
    extra_node_data: ExtraNodeData<'a>,
}

impl<'t: 'a, 'a> protocol_lib::Packet<'t> for CommandNode<'a> {
    fn serialize<W: std::io::Write>(
        &self,
        w: cookie_factory::WriteContext<W>,
    ) -> cookie_factory::GenResult<W> {
        let w = Flags::serialize(&self.flags, w)?;
        let w = PrefixedArray::<VarInt, VarInt>::serialize(&self.children, w)?;

        let w = match &self.redirect_node {
            RedirectNode::RedirectNode1(val) => {
                let w = VarInt::serialize(&val, w)?;
                w
            }
            RedirectNode::Default(val) => Void::serialize(val, w)?,
        };

        let w = match &self.extra_node_data {
            ExtraNodeData::<'a>::ExtraNodeData0(val) => {
                let w = Void::serialize(&val, w)?;
                w
            }
            ExtraNodeData::<'a>::ExtraNodeData1(val) => {
                let w = ExtraNodeData1::<'a>::serialize(&val, w)?;
                w
            }
            ExtraNodeData::<'a>::ExtraNodeData2(val) => {
                let w = PrefixedString::<'a, VarInt>::serialize(&val.name, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&val.parser, w)?;

                let w = match &val.properties {
                    Properties::<'a>::Bool(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Float(val) => {
                        let w = Float::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Double(val) => {
                        let w = Double::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Integer(val) => {
                        let w = Integer::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Long(val) => {
                        let w = Long::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::String(val) => {
                        let tag = match &val[..] {
                            "SINGLE_WORD" => "0",
                            "QUOTABLE_PHRASE" => "1",
                            "GREEDY_PHRASE" => "2",
                        };
                        let tag2 = str::parse(tag).unwrap();
                        let w = VarInt::serialize(&tag2, w)?;
                        w
                    }
                    Properties::<'a>::MinecraftEntity(val) => {
                        let w = MinecraftEntity::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::GameProfile(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::BlockPos(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ColumnPos(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Vec3(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Vec2(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::BlockState(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::BlockPredicate(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ItemStack(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ItemPredicate(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Color(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Component(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Message(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Nbt(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::NbtPath(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Objective(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ObjectiveCriteria(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Operation(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Particle(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Angle(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Rotation(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ScoreboardSlot(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ScoreHolder(val) => {
                        let w = ScoreHolder::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Swizzle(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Team(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ItemSlot(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ResourceLocation(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::MobEffect(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Function(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::EntityAnchor(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Range(val) => {
                        let w = Range::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::IntRange(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::FloatRange(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ItemEnchantment(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::EntitySummon(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Dimension(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::NbtCompoundTag(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Time(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::ResourceOrTag(val) => {
                        let w = ResourceOrTag::<'a>::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Resource(val) => {
                        let w = Resource::<'a>::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Uuid(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Properties::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &val.suggestion_type {
                    SuggestionType::<'a>::SuggestionType1(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    SuggestionType::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                w
            }
            ExtraNodeData::<'a>::Default(val) => Void::serialize(val, w)?,
        };

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (|input| {
            let (input, self_flags) = (Flags::deserialize)(input)?;
            let (input, self_children) = (PrefixedArray::<VarInt, VarInt>::deserialize)(input)?;
            let (input, self_redirect_node) =
                (|input| match &format!("{}", self_flags.has_redirect_node)[..] {
                    "1" => nom::combinator::map(VarInt::deserialize, RedirectNode::RedirectNode1)(
                        input,
                    ),
                    _ => nom::combinator::map(Void::deserialize, RedirectNode::Default)(input),
                })(input)?;
            let (input, self_extra_node_data) = (|input| match &format!(
                "{}",
                self_flags.command_node_type
            )[..]
            {
                "0" => {
                    nom::combinator::map(Void::deserialize, ExtraNodeData::<'a>::ExtraNodeData0)(
                        input,
                    )
                }
                "1" => nom::combinator::map(
                    ExtraNodeData1::<'a>::deserialize,
                    ExtraNodeData::<'a>::ExtraNodeData1,
                )(input),
                "2" => nom::combinator::map(
                    |input| {
                        let (input, self_extra_node_data_name) =
                            (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                        let (input, self_extra_node_data_parser) =
                            (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                        let (input, self_extra_node_data_properties) =
                            (|input| match &format!("{}", self_extra_node_data_parser)[..] {
                                "brigadier:bool" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Bool,
                                )(input),
                                "brigadier:float" => nom::combinator::map(
                                    Float::deserialize,
                                    Properties::<'a>::Float,
                                )(input),
                                "brigadier:double" => nom::combinator::map(
                                    Double::deserialize,
                                    Properties::<'a>::Double,
                                )(input),
                                "brigadier:integer" => nom::combinator::map(
                                    Integer::deserialize,
                                    Properties::<'a>::Integer,
                                )(input),
                                "brigadier:long" => nom::combinator::map(
                                    Long::deserialize,
                                    Properties::<'a>::Long,
                                )(input),
                                "brigadier:string" => nom::combinator::map(
                                    |input| {
                                        let (input, x) = (VarInt::deserialize)(input)?;
                                        let x = format!("{x}");
                                        let val = match &x[..] {
                                            "0" => "SINGLE_WORD",
                                            "1" => "QUOTABLE_PHRASE",
                                            "2" => "GREEDY_PHRASE",

                                            _ => {
                                                return Err(nom::Err::Error(
                                                    nom::error::Error::new(
                                                        input,
                                                        nom::error::ErrorKind::Verify,
                                                    ),
                                                ))
                                            }
                                        };
                                        Ok((input, val))
                                    },
                                    Properties::<'a>::String,
                                )(input),
                                "minecraft:entity" => nom::combinator::map(
                                    MinecraftEntity::deserialize,
                                    Properties::<'a>::MinecraftEntity,
                                )(input),
                                "minecraft:game_profile" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::GameProfile,
                                )(input),
                                "minecraft:block_pos" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::BlockPos,
                                )(input),
                                "minecraft:column_pos" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::ColumnPos,
                                )(input),
                                "minecraft:vec3" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Vec3,
                                )(input),
                                "minecraft:vec2" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Vec2,
                                )(input),
                                "minecraft:block_state" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::BlockState,
                                )(input),
                                "minecraft:block_predicate" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::BlockPredicate,
                                    )(input)
                                }
                                "minecraft:item_stack" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::ItemStack,
                                )(input),
                                "minecraft:item_predicate" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::ItemPredicate,
                                    )(input)
                                }
                                "minecraft:color" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Color,
                                )(input),
                                "minecraft:component" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Component,
                                )(input),
                                "minecraft:message" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Message,
                                )(input),
                                "minecraft:nbt" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Nbt,
                                )(input),
                                "minecraft:nbt_path" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::NbtPath,
                                )(input),
                                "minecraft:objective" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Objective,
                                )(input),
                                "minecraft:objective_criteria" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::ObjectiveCriteria,
                                    )(input)
                                }
                                "minecraft:operation" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Operation,
                                )(input),
                                "minecraft:particle" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Particle,
                                )(input),
                                "minecraft:angle" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Angle,
                                )(input),
                                "minecraft:rotation" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Rotation,
                                )(input),
                                "minecraft:scoreboard_slot" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::ScoreboardSlot,
                                    )(input)
                                }
                                "minecraft:score_holder" => nom::combinator::map(
                                    ScoreHolder::deserialize,
                                    Properties::<'a>::ScoreHolder,
                                )(input),
                                "minecraft:swizzle" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Swizzle,
                                )(input),
                                "minecraft:team" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Team,
                                )(input),
                                "minecraft:item_slot" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::ItemSlot,
                                )(input),
                                "minecraft:resource_location" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::ResourceLocation,
                                    )(input)
                                }
                                "minecraft:mob_effect" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::MobEffect,
                                )(input),
                                "minecraft:function" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Function,
                                )(input),
                                "minecraft:entity_anchor" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::EntityAnchor,
                                    )(input)
                                }
                                "minecraft:range" => nom::combinator::map(
                                    Range::deserialize,
                                    Properties::<'a>::Range,
                                )(input),
                                "minecraft:int_range" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::IntRange,
                                )(input),
                                "minecraft:float_range" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::FloatRange,
                                )(input),
                                "minecraft:item_enchantment" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::ItemEnchantment,
                                    )(input)
                                }
                                "minecraft:entity_summon" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::EntitySummon,
                                    )(input)
                                }
                                "minecraft:dimension" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Dimension,
                                )(input),
                                "minecraft:nbt_compound_tag" => {
                                    nom::combinator::map(
                                        Void::deserialize,
                                        Properties::<'a>::NbtCompoundTag,
                                    )(input)
                                }
                                "minecraft:time" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Time,
                                )(input),
                                "minecraft:resource_or_tag" => {
                                    nom::combinator::map(
                                        ResourceOrTag::<'a>::deserialize,
                                        Properties::<'a>::ResourceOrTag,
                                    )(input)
                                }
                                "minecraft:resource" => nom::combinator::map(
                                    Resource::<'a>::deserialize,
                                    Properties::<'a>::Resource,
                                )(input),
                                "minecraft:uuid" => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Uuid,
                                )(input),
                                _ => nom::combinator::map(
                                    Void::deserialize,
                                    Properties::<'a>::Default,
                                )(input),
                            })(input)?;
                        let (input, self_extra_node_data_suggestion_type) =
                            (|input| match &format!("{}", self_flags.has_custom_suggestions)[..] {
                                "1" => nom::combinator::map(
                                    PrefixedString::<'a, VarInt>::deserialize,
                                    SuggestionType::<'a>::SuggestionType1,
                                )(input),
                                _ => nom::combinator::map(
                                    Void::deserialize,
                                    SuggestionType::<'a>::Default,
                                )(input),
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
                    ExtraNodeData::<'a>::ExtraNodeData2,
                )(input),
                _ => nom::combinator::map(Void::deserialize, ExtraNodeData::<'a>::Default)(input),
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
        pub enum Params {
            Default(Void),
        }

        impl Params {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    _ => "",
                }
            }
        }
        pub struct Packet {
            name: &'static str,
            params: Params,
        }

        impl<'t> protocol_lib::Packet<'t> for Packet {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = match &self.params {
                    Params::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (|x| Ok((x, "")))(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        _ => nom::combinator::map(Void::deserialize, Params::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketSetProtocol<'a> {
            protocol_version: VarInt,
            server_host: VarString<'a>,
            server_port: u16,
            next_state: VarInt,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSetProtocol<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.protocol_version, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.server_host, w)?;
                let w = u16::serialize(&self.server_port, w)?;
                let w = VarInt::serialize(&self.next_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        u16::deserialize,
                        VarInt::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.payload, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((u8::deserialize,)), |(payload,)| {
                    PacketLegacyServerListPing { payload }
                }))(input)
            }
        }

        pub enum Params<'a> {
            SetProtocol(PacketSetProtocol<'a>),
            LegacyServerListPing(PacketLegacyServerListPing),
            Default(Void),
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::<'a>::SetProtocol(_) => "set_protocol",
                    Params::<'a>::LegacyServerListPing(_) => "legacy_server_list_ping",
                    _ => "",
                }
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "set_protocol" => "0x00",
                    "legacy_server_list_ping" => "0xfe",
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = match &self.params {
                    Params::<'a>::SetProtocol(val) => {
                        let w = PacketSetProtocol::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::LegacyServerListPing(val) => {
                        let w = PacketLegacyServerListPing::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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

                            _ => {
                                return Err(nom::Err::Error(nom::error::Error::new(
                                    input,
                                    nom::error::ErrorKind::Verify,
                                )))
                            }
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "set_protocol" => nom::combinator::map(
                            PacketSetProtocol::<'a>::deserialize,
                            Params::<'a>::SetProtocol,
                        )(input),
                        "legacy_server_list_ping" => nom::combinator::map(
                            PacketLegacyServerListPing::deserialize,
                            Params::<'a>::LegacyServerListPing,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Params::<'a>::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
}
pub mod status {
    pub mod clientbound {
        use crate::test::*;
        pub struct PacketServerInfo<'a> {
            response: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketServerInfo<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.response, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(response,)| PacketServerInfo { response },
                ))(input)
            }
        }

        pub struct PacketPing {
            time: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPing {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((i64::deserialize,)), |(time,)| {
                    PacketPing { time }
                }))(input)
            }
        }

        pub enum Params<'a> {
            ServerInfo(PacketServerInfo<'a>),
            Ping(PacketPing),
            Default(Void),
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::<'a>::ServerInfo(_) => "server_info",
                    Params::<'a>::Ping(_) => "ping",
                    _ => "",
                }
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "server_info" => "0x00",
                    "ping" => "0x01",
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = match &self.params {
                    Params::<'a>::ServerInfo(val) => {
                        let w = PacketServerInfo::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Ping(val) => {
                        let w = PacketPing::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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

                            _ => {
                                return Err(nom::Err::Error(nom::error::Error::new(
                                    input,
                                    nom::error::ErrorKind::Verify,
                                )))
                            }
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "server_info" => nom::combinator::map(
                            PacketServerInfo::<'a>::deserialize,
                            Params::<'a>::ServerInfo,
                        )(input),
                        "ping" => {
                            nom::combinator::map(PacketPing::deserialize, Params::<'a>::Ping)(input)
                        }
                        _ => nom::combinator::map(Void::deserialize, Params::<'a>::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketPingStart {}

        impl<'t> protocol_lib::Packet<'t> for PacketPingStart {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((|i| Ok((i, ())),)), |_| {
                    PacketPingStart {}
                }))(input)
            }
        }

        pub struct PacketPing {
            time: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPing {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((i64::deserialize,)), |(time,)| {
                    PacketPing { time }
                }))(input)
            }
        }

        pub enum Params {
            PingStart(PacketPingStart),
            Ping(PacketPing),
            Default(Void),
        }

        impl Params {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::PingStart(_) => "ping_start",
                    Params::Ping(_) => "ping",
                    _ => "",
                }
            }
        }
        pub struct Packet {
            name: &'static str,
            params: Params,
        }

        impl<'t> protocol_lib::Packet<'t> for Packet {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "ping_start" => "0x00",
                    "ping" => "0x01",
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = match &self.params {
                    Params::PingStart(val) => {
                        let w = PacketPingStart::serialize(&val, w)?;
                        w
                    }
                    Params::Ping(val) => {
                        let w = PacketPing::serialize(&val, w)?;
                        w
                    }
                    Params::Default(val) => Void::serialize(val, w)?,
                };

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

                            _ => {
                                return Err(nom::Err::Error(nom::error::Error::new(
                                    input,
                                    nom::error::ErrorKind::Verify,
                                )))
                            }
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "ping_start" => nom::combinator::map(
                            PacketPingStart::deserialize,
                            Params::PingStart,
                        )(input),
                        "ping" => {
                            nom::combinator::map(PacketPing::deserialize, Params::Ping)(input)
                        }
                        _ => nom::combinator::map(Void::deserialize, Params::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
}
pub mod login {
    pub mod clientbound {
        use crate::test::*;
        pub struct PacketDisconnect<'a> {
            reason: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDisconnect<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.reason, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(reason,)| PacketDisconnect { reason },
                ))(input)
            }
        }

        pub struct PacketEncryptionBegin<'a> {
            server_id: VarString<'a>,
            public_key: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEncryptionBegin<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.server_id, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.public_key, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.verify_token, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedBuffer::<'a, VarInt>::deserialize,
                        PrefixedBuffer::<'a, VarInt>::deserialize,
                    )),
                    |(server_id, public_key, verify_token)| PacketEncryptionBegin {
                        server_id,
                        public_key,
                        verify_token,
                    },
                ))(input)
            }
        }

        pub struct PacketSuccess<'a> {
            uuid: Uuid,
            username: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSuccess<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.uuid, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.username, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Uuid::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(uuid, username)| PacketSuccess { uuid, username },
                ))(input)
            }
        }

        pub struct PacketCompress {
            threshold: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCompress {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.threshold, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(threshold,)| PacketCompress { threshold },
                ))(input)
            }
        }

        pub struct PacketLoginPluginRequest<'a> {
            message_id: VarInt,
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketLoginPluginRequest<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.message_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.channel, w)?;
                let w = RestBuffer::<'a>::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        RestBuffer::<'a>::deserialize,
                    )),
                    |(message_id, channel, data)| PacketLoginPluginRequest {
                        message_id,
                        channel,
                        data,
                    },
                ))(input)
            }
        }

        pub enum Params<'a> {
            Disconnect(PacketDisconnect<'a>),
            EncryptionBegin(PacketEncryptionBegin<'a>),
            Success(PacketSuccess<'a>),
            Compress(PacketCompress),
            LoginPluginRequest(PacketLoginPluginRequest<'a>),
            Default(Void),
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::<'a>::Disconnect(_) => "disconnect",
                    Params::<'a>::EncryptionBegin(_) => "encryption_begin",
                    Params::<'a>::Success(_) => "success",
                    Params::<'a>::Compress(_) => "compress",
                    Params::<'a>::LoginPluginRequest(_) => "login_plugin_request",
                    _ => "",
                }
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "disconnect" => "0x00",
                    "encryption_begin" => "0x01",
                    "success" => "0x02",
                    "compress" => "0x03",
                    "login_plugin_request" => "0x04",
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = match &self.params {
                    Params::<'a>::Disconnect(val) => {
                        let w = PacketDisconnect::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EncryptionBegin(val) => {
                        let w = PacketEncryptionBegin::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Success(val) => {
                        let w = PacketSuccess::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Compress(val) => {
                        let w = PacketCompress::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::LoginPluginRequest(val) => {
                        let w = PacketLoginPluginRequest::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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

                            _ => {
                                return Err(nom::Err::Error(nom::error::Error::new(
                                    input,
                                    nom::error::ErrorKind::Verify,
                                )))
                            }
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "disconnect" => nom::combinator::map(
                            PacketDisconnect::<'a>::deserialize,
                            Params::<'a>::Disconnect,
                        )(input),
                        "encryption_begin" => nom::combinator::map(
                            PacketEncryptionBegin::<'a>::deserialize,
                            Params::<'a>::EncryptionBegin,
                        )(input),
                        "success" => nom::combinator::map(
                            PacketSuccess::<'a>::deserialize,
                            Params::<'a>::Success,
                        )(input),
                        "compress" => nom::combinator::map(
                            PacketCompress::deserialize,
                            Params::<'a>::Compress,
                        )(input),
                        "login_plugin_request" => nom::combinator::map(
                            PacketLoginPluginRequest::<'a>::deserialize,
                            Params::<'a>::LoginPluginRequest,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Params::<'a>::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketLoginStart<'a> {
            username: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketLoginStart<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.username, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(username,)| PacketLoginStart { username },
                ))(input)
            }
        }

        pub struct PacketEncryptionBegin<'a> {
            shared_secret: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEncryptionBegin<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.shared_secret, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.verify_token, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedBuffer::<'a, VarInt>::deserialize,
                        PrefixedBuffer::<'a, VarInt>::deserialize,
                    )),
                    |(shared_secret, verify_token)| PacketEncryptionBegin {
                        shared_secret,
                        verify_token,
                    },
                ))(input)
            }
        }

        pub struct PacketLoginPluginResponse<'a> {
            message_id: VarInt,
            data: Option<RestBuffer<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketLoginPluginResponse<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.message_id, w)?;
                let w = Option::<RestBuffer<'a>>::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        Option::<RestBuffer<'a>>::deserialize,
                    )),
                    |(message_id, data)| PacketLoginPluginResponse { message_id, data },
                ))(input)
            }
        }

        pub enum Params<'a> {
            LoginStart(PacketLoginStart<'a>),
            EncryptionBegin(PacketEncryptionBegin<'a>),
            LoginPluginResponse(PacketLoginPluginResponse<'a>),
            Default(Void),
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::<'a>::LoginStart(_) => "login_start",
                    Params::<'a>::EncryptionBegin(_) => "encryption_begin",
                    Params::<'a>::LoginPluginResponse(_) => "login_plugin_response",
                    _ => "",
                }
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let tag = match &self.name[..] {
                    "login_start" => "0x00",
                    "encryption_begin" => "0x01",
                    "login_plugin_response" => "0x02",
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = match &self.params {
                    Params::<'a>::LoginStart(val) => {
                        let w = PacketLoginStart::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EncryptionBegin(val) => {
                        let w = PacketEncryptionBegin::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::LoginPluginResponse(val) => {
                        let w = PacketLoginPluginResponse::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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

                            _ => {
                                return Err(nom::Err::Error(nom::error::Error::new(
                                    input,
                                    nom::error::ErrorKind::Verify,
                                )))
                            }
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "login_start" => nom::combinator::map(
                            PacketLoginStart::<'a>::deserialize,
                            Params::<'a>::LoginStart,
                        )(input),
                        "encryption_begin" => nom::combinator::map(
                            PacketEncryptionBegin::<'a>::deserialize,
                            Params::<'a>::EncryptionBegin,
                        )(input),
                        "login_plugin_response" => nom::combinator::map(
                            PacketLoginPluginResponse::<'a>::deserialize,
                            Params::<'a>::LoginPluginResponse,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Params::<'a>::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
}
pub mod play {
    pub mod clientbound {
        use crate::test::*;
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
                    |(
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
                    )| PacketSpawnEntity {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = i16::serialize(&self.count, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        i16::deserialize,
                    )),
                    |(entity_id, x, y, z, count)| PacketSpawnEntityExperienceOrb {
                        entity_id,
                        x,
                        y,
                        z,
                        count,
                    },
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
                    |(
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
                    )| PacketSpawnEntityLiving {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = Uuid::serialize(&self.entity_uuid, w)?;
                let w = VarInt::serialize(&self.title, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = u8::serialize(&self.direction, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        Uuid::deserialize,
                        VarInt::deserialize,
                        Position::deserialize,
                        u8::deserialize,
                    )),
                    |(entity_id, entity_uuid, title, location, direction)| {
                        PacketSpawnEntityPainting {
                            entity_id,
                            entity_uuid,
                            title,
                            location,
                            direction,
                        }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = u8::serialize(&self.animation, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, u8::deserialize)),
                    |(entity_id, animation)| PacketAnimation {
                        entity_id,
                        animation,
                    },
                ))(input)
            }
        }

        pub struct StatisticsEntry {
            category_id: VarInt,
            statistic_id: VarInt,
            value: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for StatisticsEntry {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.category_id, w)?;
                let w = VarInt::serialize(&self.statistic_id, w)?;
                let w = VarInt::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                    )),
                    |(category_id, statistic_id, value)| StatisticsEntry {
                        category_id,
                        statistic_id,
                        value,
                    },
                ))(input)
            }
        }

        pub struct PacketStatistics {
            entries: VarArray<StatisticsEntry>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketStatistics {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<StatisticsEntry, VarInt>::serialize(&self.entries, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedArray::<StatisticsEntry, VarInt>::deserialize,)),
                    |(entries,)| PacketStatistics { entries },
                ))(input)
            }
        }

        pub struct Ident8Flags {
            unused: u32,
            hidden: u8,
            show_toast: u8,
            has_background_texture: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for Ident8Flags {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = write_bits(
                    &[
                        (self.unused as u64, 29),
                        (self.hidden as u64, 1),
                        (self.show_toast as u64, 1),
                        (self.has_background_texture as u64, 1),
                    ],
                    w,
                )?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
                    nom::sequence::tuple((
                        parse_bits_unsigned(29),
                        parse_bits_unsigned(1),
                        parse_bits_unsigned(1),
                        parse_bits_unsigned(1),
                    )),
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
            Default(Void),
        }

        impl<'a> BackgroundTexture<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    BackgroundTexture::<'a>::BackgroundTexture1(_) => "1",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.title, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.description, w)?;
                let w = Slot::serialize(&self.icon, w)?;
                let w = VarInt::serialize(&self.frame_type, w)?;
                let w = Ident8Flags::serialize(&self.flags, w)?;

                let w = match &self.background_texture {
                    BackgroundTexture::<'a>::BackgroundTexture1(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    BackgroundTexture::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = f32::serialize(&self.x_cord, w)?;
                let w = f32::serialize(&self.y_cord, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_title) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_description) =
                        (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_icon) = (Slot::deserialize)(input)?;
                    let (input, self_frame_type) = (VarInt::deserialize)(input)?;
                    let (input, self_flags) = (Ident8Flags::deserialize)(input)?;
                    let (input, self_background_texture) =
                        (|input| match &format!("{}", self_flags.has_background_texture)[..] {
                            "1" => nom::combinator::map(
                                PrefixedString::<'a, VarInt>::deserialize,
                                BackgroundTexture::<'a>::BackgroundTexture1,
                            )(input),
                            _ => nom::combinator::map(
                                Void::deserialize,
                                BackgroundTexture::<'a>::Default,
                            )(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;
                let w = Void::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        Void::deserialize,
                    )),
                    |(key, value)| CriteriaItem { key, value },
                ))(input)
            }
        }

        pub struct AdvancementMappingItemValue<'a> {
            parent_id: Option<VarString<'a>>,
            display_data: Option<Ident8<'a>>,
            criteria: VarArray<CriteriaItem<'a>>,
            requirements: VarArray<VarStringArray<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for AdvancementMappingItemValue<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Option::<VarString<'a>>::serialize(&self.parent_id, w)?;
                let w = Option::<Ident8<'a>>::serialize(&self.display_data, w)?;
                let w = PrefixedArray::<CriteriaItem<'a>, VarInt>::serialize(&self.criteria, w)?;
                let w =
                    PrefixedArray::<VarStringArray<'a>, VarInt>::serialize(&self.requirements, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Option::<VarString<'a>>::deserialize,
                        Option::<Ident8<'a>>::deserialize,
                        PrefixedArray::<CriteriaItem<'a>, VarInt>::deserialize,
                        PrefixedArray::<VarStringArray<'a>, VarInt>::deserialize,
                    )),
                    |(parent_id, display_data, criteria, requirements)| {
                        AdvancementMappingItemValue {
                            parent_id,
                            display_data,
                            criteria,
                            requirements,
                        }
                    },
                ))(input)
            }
        }

        pub struct AdvancementMappingItem<'a> {
            key: VarString<'a>,
            value: AdvancementMappingItemValue<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for AdvancementMappingItem<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;
                let w = AdvancementMappingItemValue::<'a>::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        AdvancementMappingItemValue::<'a>::deserialize,
                    )),
                    |(key, value)| AdvancementMappingItem { key, value },
                ))(input)
            }
        }

        pub struct ProgressMappingItemValueItem<'a> {
            criterion_identifier: VarString<'a>,
            criterion_progress: Option<i64>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for ProgressMappingItemValueItem<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.criterion_identifier, w)?;
                let w = Option::<i64>::serialize(&self.criterion_progress, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        Option::<i64>::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;
                let w = PrefixedArray::<ProgressMappingItemValueItem<'a>, VarInt>::serialize(
                    &self.value,
                    w,
                )?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedArray::<ProgressMappingItemValueItem<'a>, VarInt>::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.reset, w)?;
                let w = PrefixedArray::<AdvancementMappingItem<'a>, VarInt>::serialize(
                    &self.advancement_mapping,
                    w,
                )?;
                let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&self.identifiers, w)?;
                let w = PrefixedArray::<ProgressMappingItem<'a>, VarInt>::serialize(
                    &self.progress_mapping,
                    w,
                )?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        bool::deserialize,
                        PrefixedArray::<AdvancementMappingItem<'a>, VarInt>::deserialize,
                        PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                        PrefixedArray::<ProgressMappingItem<'a>, VarInt>::deserialize,
                    )),
                    |(reset, advancement_mapping, identifiers, progress_mapping)| {
                        PacketAdvancements {
                            reset,
                            advancement_mapping,
                            identifiers,
                            progress_mapping,
                        }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = i8::serialize(&self.destroy_stage, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        Position::deserialize,
                        i8::deserialize,
                    )),
                    |(entity_id, location, destroy_stage)| PacketBlockBreakAnimation {
                        entity_id,
                        location,
                        destroy_stage,
                    },
                ))(input)
            }
        }

        pub struct PacketTileEntityData {
            location: Position,
            action: VarInt,
            nbt_data: OptionalNbt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketTileEntityData {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.action, w)?;
                let w = OptionalNbt::serialize(&self.nbt_data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Position::deserialize,
                        VarInt::deserialize,
                        OptionalNbt::deserialize,
                    )),
                    |(location, action, nbt_data)| PacketTileEntityData {
                        location,
                        action,
                        nbt_data,
                    },
                ))(input)
            }
        }

        pub struct PacketBlockAction {
            location: Position,
            byte1: u8,
            byte2: u8,
            block_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockAction {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = u8::serialize(&self.byte1, w)?;
                let w = u8::serialize(&self.byte2, w)?;
                let w = VarInt::serialize(&self.block_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Position::deserialize,
                        u8::deserialize,
                        u8::deserialize,
                        VarInt::deserialize,
                    )),
                    |(location, byte1, byte2, block_id)| PacketBlockAction {
                        location,
                        byte1,
                        byte2,
                        block_id,
                    },
                ))(input)
            }
        }

        pub struct PacketBlockChange {
            location: Position,
            r_type: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockChange {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.r_type, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((Position::deserialize, VarInt::deserialize)),
                    |(location, r_type)| PacketBlockChange { location, r_type },
                ))(input)
            }
        }

        pub enum BossBarTitle<'a> {
            BossBarTitle0(VarString<'a>),
            BossBarTitle3(VarString<'a>),
            Default(Void),
        }

        impl<'a> BossBarTitle<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    BossBarTitle::<'a>::BossBarTitle0(_) => "0",
                    BossBarTitle::<'a>::BossBarTitle3(_) => "3",
                    _ => "",
                }
            }
        }
        pub enum Health {
            Health0(f32),
            Health2(f32),
            Default(Void),
        }

        impl Health {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Health::Health0(_) => "0",
                    Health::Health2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum Color {
            Color0(VarInt),
            Color4(VarInt),
            Default(Void),
        }

        impl Color {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Color::Color0(_) => "0",
                    Color::Color4(_) => "4",
                    _ => "",
                }
            }
        }
        pub enum Dividers {
            Dividers0(VarInt),
            Dividers4(VarInt),
            Default(Void),
        }

        impl Dividers {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Dividers::Dividers0(_) => "0",
                    Dividers::Dividers4(_) => "4",
                    _ => "",
                }
            }
        }
        pub enum BossBarFlags {
            BossBarFlags0(u8),
            BossBarFlags5(u8),
            Default(Void),
        }

        impl BossBarFlags {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    BossBarFlags::BossBarFlags0(_) => "0",
                    BossBarFlags::BossBarFlags5(_) => "5",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.entity_uuid, w)?;
                let w = VarInt::serialize(&self.action, w)?;

                let w = match &self.title {
                    BossBarTitle::<'a>::BossBarTitle0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    BossBarTitle::<'a>::BossBarTitle3(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    BossBarTitle::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.health {
                    Health::Health0(val) => {
                        let w = f32::serialize(&val, w)?;
                        w
                    }
                    Health::Health2(val) => {
                        let w = f32::serialize(&val, w)?;
                        w
                    }
                    Health::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.color {
                    Color::Color0(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Color::Color4(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Color::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.dividers {
                    Dividers::Dividers0(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Dividers::Dividers4(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Dividers::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.flags {
                    BossBarFlags::BossBarFlags0(val) => {
                        let w = u8::serialize(&val, w)?;
                        w
                    }
                    BossBarFlags::BossBarFlags5(val) => {
                        let w = u8::serialize(&val, w)?;
                        w
                    }
                    BossBarFlags::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_entity_uuid) = (Uuid::deserialize)(input)?;
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_title) = (|input| match &format!("{}", self_action)[..] {
                        "0" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            BossBarTitle::<'a>::BossBarTitle0,
                        )(input),
                        "3" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            BossBarTitle::<'a>::BossBarTitle3,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, BossBarTitle::<'a>::Default)(
                            input,
                        ),
                    })(input)?;
                    let (input, self_health) = (|input| match &format!("{}", self_action)[..] {
                        "0" => nom::combinator::map(f32::deserialize, Health::Health0)(input),
                        "2" => nom::combinator::map(f32::deserialize, Health::Health2)(input),
                        _ => nom::combinator::map(Void::deserialize, Health::Default)(input),
                    })(input)?;
                    let (input, self_color) = (|input| match &format!("{}", self_action)[..] {
                        "0" => nom::combinator::map(VarInt::deserialize, Color::Color0)(input),
                        "4" => nom::combinator::map(VarInt::deserialize, Color::Color4)(input),
                        _ => nom::combinator::map(Void::deserialize, Color::Default)(input),
                    })(input)?;
                    let (input, self_dividers) = (|input| match &format!("{}", self_action)[..] {
                        "0" => {
                            nom::combinator::map(VarInt::deserialize, Dividers::Dividers0)(input)
                        }
                        "4" => {
                            nom::combinator::map(VarInt::deserialize, Dividers::Dividers4)(input)
                        }
                        _ => nom::combinator::map(Void::deserialize, Dividers::Default)(input),
                    })(input)?;
                    let (input, self_flags) = (|input| match &format!("{}", self_action)[..] {
                        "0" => nom::combinator::map(u8::deserialize, BossBarFlags::BossBarFlags0)(
                            input,
                        ),
                        "5" => nom::combinator::map(u8::deserialize, BossBarFlags::BossBarFlags5)(
                            input,
                        ),
                        _ => nom::combinator::map(Void::deserialize, BossBarFlags::Default)(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.difficulty, w)?;
                let w = bool::serialize(&self.difficulty_locked, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((u8::deserialize, bool::deserialize)),
                    |(difficulty, difficulty_locked)| PacketDifficulty {
                        difficulty,
                        difficulty_locked,
                    },
                ))(input)
            }
        }

        pub struct Matche<'a> {
            r_match: VarString<'a>,
            tooltip: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Matche<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.r_match, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.tooltip, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        Option::<VarString<'a>>::deserialize,
                    )),
                    |(r_match, tooltip)| Matche { r_match, tooltip },
                ))(input)
            }
        }

        pub struct PacketTabComplete<'a> {
            transaction_id: VarInt,
            start: VarInt,
            length: VarInt,
            matches: VarArray<Matche<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketTabComplete<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = VarInt::serialize(&self.start, w)?;
                let w = VarInt::serialize(&self.length, w)?;
                let w = PrefixedArray::<Matche<'a>, VarInt>::serialize(&self.matches, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        PrefixedArray::<Matche<'a>, VarInt>::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<CommandNode<'a>, VarInt>::serialize(&self.nodes, w)?;
                let w = VarInt::serialize(&self.root_index, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedArray::<CommandNode<'a>, VarInt>::deserialize,
                        VarInt::deserialize,
                    )),
                    |(nodes, root_index)| PacketDeclareCommands { nodes, root_index },
                ))(input)
            }
        }

        pub enum FacePlayerEntityId {
            FacePlayerEntityIdTrue(VarInt),
            Default(Void),
        }

        impl FacePlayerEntityId {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    FacePlayerEntityId::FacePlayerEntityIdTrue(_) => "true",
                    _ => "",
                }
            }
        }
        pub enum EntityFeetEyes<'a> {
            EntityFeetEyesTrue(VarString<'a>),
            Default(Void),
        }

        impl<'a> EntityFeetEyes<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    EntityFeetEyes::<'a>::EntityFeetEyesTrue(_) => "true",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.feet_eyes, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = bool::serialize(&self.is_entity, w)?;

                let w = match &self.entity_id {
                    FacePlayerEntityId::FacePlayerEntityIdTrue(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    FacePlayerEntityId::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.entity_feet_eyes {
                    EntityFeetEyes::<'a>::EntityFeetEyesTrue(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    EntityFeetEyes::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_feet_eyes) = (VarInt::deserialize)(input)?;
                    let (input, self_x) = (f64::deserialize)(input)?;
                    let (input, self_y) = (f64::deserialize)(input)?;
                    let (input, self_z) = (f64::deserialize)(input)?;
                    let (input, self_is_entity) = (bool::deserialize)(input)?;
                    let (input, self_entity_id) = (|input| match &format!("{}", self_is_entity)[..]
                    {
                        "true" => nom::combinator::map(
                            VarInt::deserialize,
                            FacePlayerEntityId::FacePlayerEntityIdTrue,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, FacePlayerEntityId::Default)(
                            input,
                        ),
                    })(input)?;
                    let (input, self_entity_feet_eyes) =
                        (|input| match &format!("{}", self_is_entity)[..] {
                            "true" => nom::combinator::map(
                                PrefixedString::<'a, VarInt>::deserialize,
                                EntityFeetEyes::<'a>::EntityFeetEyesTrue,
                            )(input),
                            _ => nom::combinator::map(
                                Void::deserialize,
                                EntityFeetEyes::<'a>::Default,
                            )(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = OptionalNbt::serialize(&self.nbt, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, OptionalNbt::deserialize)),
                    |(transaction_id, nbt)| PacketNbtQueryResponse {
                        transaction_id,
                        nbt,
                    },
                ))(input)
            }
        }

        pub struct PacketChat<'a> {
            message: VarString<'a>,
            position: i8,
            sender: Uuid,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketChat<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.message, w)?;
                let w = i8::serialize(&self.position, w)?;
                let w = Uuid::serialize(&self.sender, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        i8::deserialize,
                        Uuid::deserialize,
                    )),
                    |(message, position, sender)| PacketChat {
                        message,
                        position,
                        sender,
                    },
                ))(input)
            }
        }

        pub struct ChunkCoordinates {
            x: i32,
            z: i32,
            y: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for ChunkCoordinates {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(nom::combinator::map(
                    nom::sequence::tuple((
                        parse_bits_signed(22),
                        parse_bits_signed(22),
                        parse_bits_signed(20),
                    )),
                    |(x, z, y)| ChunkCoordinates { x, z, y },
                )))(input)
            }
        }

        pub struct PacketMultiBlockChange {
            chunk_coordinates: ChunkCoordinates,
            not_trust_edges: bool,
            records: VarArray<VarLong>,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketMultiBlockChange {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = ChunkCoordinates::serialize(&self.chunk_coordinates, w)?;
                let w = bool::serialize(&self.not_trust_edges, w)?;
                let w = PrefixedArray::<VarLong, VarInt>::serialize(&self.records, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        ChunkCoordinates::deserialize,
                        bool::deserialize,
                        PrefixedArray::<VarLong, VarInt>::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((u8::deserialize,)), |(window_id,)| {
                    PacketCloseWindow { window_id }
                }))(input)
            }
        }

        pub struct PacketOpenWindow<'a> {
            window_id: VarInt,
            inventory_type: VarInt,
            window_title: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketOpenWindow<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.inventory_type, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.window_title, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.state_id, w)?;
                let w = PrefixedArray::<Slot, VarInt>::serialize(&self.items, w)?;
                let w = Slot::serialize(&self.carried_item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        u8::deserialize,
                        VarInt::deserialize,
                        PrefixedArray::<Slot, VarInt>::deserialize,
                        Slot::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = i16::serialize(&self.property, w)?;
                let w = i16::serialize(&self.value, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((u8::deserialize, i16::deserialize, i16::deserialize)),
                    |(window_id, property, value)| PacketCraftProgressBar {
                        window_id,
                        property,
                        value,
                    },
                ))(input)
            }
        }

        pub struct PacketSetSlot {
            window_id: i8,
            state_id: VarInt,
            slot: i16,
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetSlot {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.state_id, w)?;
                let w = i16::serialize(&self.slot, w)?;
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        i8::deserialize,
                        VarInt::deserialize,
                        i16::deserialize,
                        Slot::deserialize,
                    )),
                    |(window_id, state_id, slot, item)| PacketSetSlot {
                        window_id,
                        state_id,
                        slot,
                        item,
                    },
                ))(input)
            }
        }

        pub struct PacketSetCooldown {
            item_id: VarInt,
            cooldown_ticks: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetCooldown {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.item_id, w)?;
                let w = VarInt::serialize(&self.cooldown_ticks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, VarInt::deserialize)),
                    |(item_id, cooldown_ticks)| PacketSetCooldown {
                        item_id,
                        cooldown_ticks,
                    },
                ))(input)
            }
        }

        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCustomPayload<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.channel, w)?;
                let w = RestBuffer::<'a>::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        RestBuffer::<'a>::deserialize,
                    )),
                    |(channel, data)| PacketCustomPayload { channel, data },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.reason, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(reason,)| PacketKickDisconnect { reason },
                ))(input)
            }
        }

        pub struct PacketEntityStatus {
            entity_id: i32,
            entity_status: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityStatus {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.entity_status, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i32::deserialize, i8::deserialize)),
                    |(entity_id, entity_status)| PacketEntityStatus {
                        entity_id,
                        entity_status,
                    },
                ))(input)
            }
        }

        pub struct AffectedBlockOffset {
            x: i8,
            y: i8,
            z: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for AffectedBlockOffset {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.x, w)?;
                let w = i8::serialize(&self.y, w)?;
                let w = i8::serialize(&self.z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i8::deserialize, i8::deserialize, i8::deserialize)),
                    |(x, y, z)| AffectedBlockOffset { x, y, z },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.x, w)?;
                let w = f32::serialize(&self.y, w)?;
                let w = f32::serialize(&self.z, w)?;
                let w = f32::serialize(&self.radius, w)?;
                let w = PrefixedArray::<AffectedBlockOffset, VarInt>::serialize(
                    &self.affected_block_offsets,
                    w,
                )?;
                let w = f32::serialize(&self.player_motion_x, w)?;
                let w = f32::serialize(&self.player_motion_y, w)?;
                let w = f32::serialize(&self.player_motion_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        PrefixedArray::<AffectedBlockOffset, VarInt>::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(
                        x,
                        y,
                        z,
                        radius,
                        affected_block_offsets,
                        player_motion_x,
                        player_motion_y,
                        player_motion_z,
                    )| PacketExplosion {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.chunk_x, w)?;
                let w = i32::serialize(&self.chunk_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i32::deserialize, i32::deserialize)),
                    |(chunk_x, chunk_z)| PacketUnloadChunk { chunk_x, chunk_z },
                ))(input)
            }
        }

        pub struct PacketGameStateChange {
            reason: u8,
            game_mode: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketGameStateChange {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.reason, w)?;
                let w = f32::serialize(&self.game_mode, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((u8::deserialize, f32::deserialize)),
                    |(reason, game_mode)| PacketGameStateChange { reason, game_mode },
                ))(input)
            }
        }

        pub struct PacketOpenHorseWindow {
            window_id: u8,
            nb_slots: VarInt,
            entity_id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketOpenHorseWindow {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.nb_slots, w)?;
                let w = i32::serialize(&self.entity_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((u8::deserialize, VarInt::deserialize, i32::deserialize)),
                    |(window_id, nb_slots, entity_id)| PacketOpenHorseWindow {
                        window_id,
                        nb_slots,
                        entity_id,
                    },
                ))(input)
            }
        }

        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketKeepAlive {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.keep_alive_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i64::deserialize,)),
                    |(keep_alive_id,)| PacketKeepAlive { keep_alive_id },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.x, w)?;
                let w = i32::serialize(&self.z, w)?;
                let w = Nbt::serialize(&self.heightmaps, w)?;
                let w = PrefixedBuffer::<'a, VarInt>::serialize(&self.chunk_data, w)?;
                let w =
                    PrefixedArray::<ChunkBlockEntity, VarInt>::serialize(&self.block_entities, w)?;
                let w = bool::serialize(&self.trust_edges, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.sky_light_mask, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.block_light_mask, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.empty_sky_light_mask, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.empty_block_light_mask, w)?;
                let w = PrefixedArray::<VarArray<u8>, VarInt>::serialize(&self.sky_light, w)?;
                let w = PrefixedArray::<VarArray<u8>, VarInt>::serialize(&self.block_light, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
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
                    |(
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
                    )| PacketMapChunk {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.effect_id, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = i32::serialize(&self.data, w)?;
                let w = bool::serialize(&self.global, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        i32::deserialize,
                        Position::deserialize,
                        i32::deserialize,
                        bool::deserialize,
                    )),
                    |(effect_id, location, data, global)| PacketWorldEvent {
                        effect_id,
                        location,
                        data,
                        global,
                    },
                ))(input)
            }
        }

        pub struct WorldParticlesData2 {
            block_state: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData2 {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.block_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(block_state,)| WorldParticlesData2 { block_state },
                ))(input)
            }
        }

        pub struct WorldParticlesData3 {
            block_state: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData3 {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.block_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(block_state,)| WorldParticlesData3 { block_state },
                ))(input)
            }
        }

        pub struct WorldParticlesData14 {
            red: f32,
            green: f32,
            blue: f32,
            scale: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData14 {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.red, w)?;
                let w = f32::serialize(&self.green, w)?;
                let w = f32::serialize(&self.blue, w)?;
                let w = f32::serialize(&self.scale, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(red, green, blue, scale)| WorldParticlesData14 {
                        red,
                        green,
                        blue,
                        scale,
                    },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(from_red, from_green, from_blue, scale, to_red, to_green, to_blue)| {
                        WorldParticlesData15 {
                            from_red,
                            from_green,
                            from_blue,
                            scale,
                            to_red,
                            to_green,
                            to_blue,
                        }
                    },
                ))(input)
            }
        }

        pub struct WorldParticlesData24 {
            block_state: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData24 {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.block_state, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(block_state,)| WorldParticlesData24 { block_state },
                ))(input)
            }
        }

        pub struct WorldParticlesData35 {
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for WorldParticlesData35 {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((Slot::deserialize,)), |(item,)| {
                    WorldParticlesData35 { item }
                }))(input)
            }
        }

        pub enum WorldParticlesData36Destination {
            MinecraftBlock(Position),
            WorldParticlesData36DestinationEntity(VarInt),
            Default(Void),
        }

        impl WorldParticlesData36Destination {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    WorldParticlesData36Destination::MinecraftBlock(_) => "minecraft:block",
                    WorldParticlesData36Destination::WorldParticlesData36DestinationEntity(_) => {
                        "minecraft:entity"
                    }
                    _ => "",
                }
            }
        }
        pub struct WorldParticlesData36<'a> {
            origin: Position,
            position_type: VarString<'a>,
            destination: WorldParticlesData36Destination,
            ticks: VarInt,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for WorldParticlesData36<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.origin, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.position_type, w)?;

                let w = match &self.destination {
                    WorldParticlesData36Destination::MinecraftBlock(val) => {
                        let w = Position::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData36Destination::WorldParticlesData36DestinationEntity(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData36Destination::Default(val) => Void::serialize(val, w)?,
                };

                let w = VarInt::serialize(&self.ticks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_origin) = (Position::deserialize)(input)?;
                    let (input, self_position_type) =
                        (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_destination) = (|input| match &format!(
                        "{}",
                        self_position_type
                    )[..]
                    {
                        "minecraft:block" => nom::combinator::map(
                            Position::deserialize,
                            WorldParticlesData36Destination::MinecraftBlock,
                        )(input),
                        "minecraft:entity" => nom::combinator::map(
                            VarInt::deserialize,
                            WorldParticlesData36Destination::WorldParticlesData36DestinationEntity,
                        )(input),
                        _ => nom::combinator::map(
                            Void::deserialize,
                            WorldParticlesData36Destination::Default,
                        )(input),
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
            Default(Void),
        }

        impl<'a> WorldParticlesData<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    WorldParticlesData::<'a>::WorldParticlesData2(_) => "2",
                    WorldParticlesData::<'a>::WorldParticlesData3(_) => "3",
                    WorldParticlesData::<'a>::WorldParticlesData14(_) => "14",
                    WorldParticlesData::<'a>::WorldParticlesData15(_) => "15",
                    WorldParticlesData::<'a>::WorldParticlesData24(_) => "24",
                    WorldParticlesData::<'a>::WorldParticlesData35(_) => "35",
                    WorldParticlesData::<'a>::WorldParticlesData36(_) => "36",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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

                let w = match &self.data {
                    WorldParticlesData::<'a>::WorldParticlesData2(val) => {
                        let w = WorldParticlesData2::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData::<'a>::WorldParticlesData3(val) => {
                        let w = WorldParticlesData3::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData::<'a>::WorldParticlesData14(val) => {
                        let w = WorldParticlesData14::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData::<'a>::WorldParticlesData15(val) => {
                        let w = WorldParticlesData15::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData::<'a>::WorldParticlesData24(val) => {
                        let w = WorldParticlesData24::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData::<'a>::WorldParticlesData35(val) => {
                        let w = WorldParticlesData35::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData::<'a>::WorldParticlesData36(val) => {
                        let w = WorldParticlesData36::<'a>::serialize(&val, w)?;
                        w
                    }
                    WorldParticlesData::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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
                        "2" => nom::combinator::map(
                            WorldParticlesData2::deserialize,
                            WorldParticlesData::<'a>::WorldParticlesData2,
                        )(input),
                        "3" => nom::combinator::map(
                            WorldParticlesData3::deserialize,
                            WorldParticlesData::<'a>::WorldParticlesData3,
                        )(input),
                        "14" => nom::combinator::map(
                            WorldParticlesData14::deserialize,
                            WorldParticlesData::<'a>::WorldParticlesData14,
                        )(input),
                        "15" => nom::combinator::map(
                            WorldParticlesData15::deserialize,
                            WorldParticlesData::<'a>::WorldParticlesData15,
                        )(input),
                        "24" => nom::combinator::map(
                            WorldParticlesData24::deserialize,
                            WorldParticlesData::<'a>::WorldParticlesData24,
                        )(input),
                        "35" => nom::combinator::map(
                            WorldParticlesData35::deserialize,
                            WorldParticlesData::<'a>::WorldParticlesData35,
                        )(input),
                        "36" => nom::combinator::map(
                            WorldParticlesData36::<'a>::deserialize,
                            WorldParticlesData::<'a>::WorldParticlesData36,
                        )(input),
                        _ => nom::combinator::map(
                            Void::deserialize,
                            WorldParticlesData::<'a>::Default,
                        )(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.chunk_x, w)?;
                let w = VarInt::serialize(&self.chunk_z, w)?;
                let w = bool::serialize(&self.trust_edges, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.sky_light_mask, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.block_light_mask, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.empty_sky_light_mask, w)?;
                let w = PrefixedArray::<i64, VarInt>::serialize(&self.empty_block_light_mask, w)?;
                let w = PrefixedArray::<VarArray<u8>, VarInt>::serialize(&self.sky_light, w)?;
                let w = PrefixedArray::<VarArray<u8>, VarInt>::serialize(&self.block_light, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
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
                    |(
                        chunk_x,
                        chunk_z,
                        trust_edges,
                        sky_light_mask,
                        block_light_mask,
                        empty_sky_light_mask,
                        empty_block_light_mask,
                        sky_light,
                        block_light,
                    )| PacketUpdateLight {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.entity_id, w)?;
                let w = bool::serialize(&self.is_hardcore, w)?;
                let w = u8::serialize(&self.game_mode, w)?;
                let w = i8::serialize(&self.previous_game_mode, w)?;
                let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&self.world_names, w)?;
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.r_type, w)?;
                let w = i8::serialize(&self.x, w)?;
                let w = i8::serialize(&self.z, w)?;
                let w = u8::serialize(&self.direction, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.display_name, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        u8::deserialize,
                        Option::<VarString<'a>>::deserialize,
                    )),
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
            Rows0(Void),
            Default(u8),
        }

        impl Rows {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Rows::Rows0(_) => "0",
                    _ => "",
                }
            }
        }
        pub enum MapX {
            MapX0(Void),
            Default(u8),
        }

        impl MapX {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    MapX::MapX0(_) => "0",
                    _ => "",
                }
            }
        }
        pub enum MapY {
            MapY0(Void),
            Default(u8),
        }

        impl MapY {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    MapY::MapY0(_) => "0",
                    _ => "",
                }
            }
        }
        pub enum MapData<'a> {
            MapData0(Void),
            Default(VarBuffer<'a>),
        }

        impl<'a> MapData<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    MapData::<'a>::MapData0(_) => "0",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.item_damage, w)?;
                let w = i8::serialize(&self.scale, w)?;
                let w = bool::serialize(&self.locked, w)?;
                let w = Option::<PrefixedArray<Ident11<'a>, VarInt>>::serialize(&self.icons, w)?;
                let w = u8::serialize(&self.columns, w)?;

                let w = match &self.rows {
                    Rows::Rows0(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    Rows::Default(val) => u8::serialize(val, w)?,
                };

                let w = match &self.x {
                    MapX::MapX0(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    MapX::Default(val) => u8::serialize(val, w)?,
                };

                let w = match &self.y {
                    MapY::MapY0(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    MapY::Default(val) => u8::serialize(val, w)?,
                };

                let w = match &self.data {
                    MapData::<'a>::MapData0(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    MapData::<'a>::Default(val) => PrefixedBuffer::<'a, VarInt>::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_item_damage) = (VarInt::deserialize)(input)?;
                    let (input, self_scale) = (i8::deserialize)(input)?;
                    let (input, self_locked) = (bool::deserialize)(input)?;
                    let (input, self_icons) =
                        (Option::<PrefixedArray<Ident11<'a>, VarInt>>::deserialize)(input)?;
                    let (input, self_columns) = (u8::deserialize)(input)?;
                    let (input, self_rows) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => nom::combinator::map(Void::deserialize, Rows::Rows0)(input),
                        _ => nom::combinator::map(u8::deserialize, Rows::Default)(input),
                    })(input)?;
                    let (input, self_x) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => nom::combinator::map(Void::deserialize, MapX::MapX0)(input),
                        _ => nom::combinator::map(u8::deserialize, MapX::Default)(input),
                    })(input)?;
                    let (input, self_y) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => nom::combinator::map(Void::deserialize, MapY::MapY0)(input),
                        _ => nom::combinator::map(u8::deserialize, MapY::Default)(input),
                    })(input)?;
                    let (input, self_data) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => {
                            nom::combinator::map(Void::deserialize, MapData::<'a>::MapData0)(input)
                        }
                        _ => nom::combinator::map(
                            PrefixedBuffer::<'a, VarInt>::deserialize,
                            MapData::<'a>::Default,
                        )(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
                    |(
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
                    )| Trade {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.window_id, w)?;
                let w = PrefixedArray::<Trade, u8>::serialize(&self.trades, w)?;
                let w = VarInt::serialize(&self.villager_level, w)?;
                let w = VarInt::serialize(&self.experience, w)?;
                let w = bool::serialize(&self.is_regular_villager, w)?;
                let w = bool::serialize(&self.can_restock, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedArray::<Trade, u8>::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(
                        window_id,
                        trades,
                        villager_level,
                        experience,
                        is_regular_villager,
                        can_restock,
                    )| PacketTradeList {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i16::serialize(&self.d_x, w)?;
                let w = i16::serialize(&self.d_y, w)?;
                let w = i16::serialize(&self.d_z, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                        bool::deserialize,
                    )),
                    |(entity_id, d_x, d_y, d_z, on_ground)| PacketRelEntityMove {
                        entity_id,
                        d_x,
                        d_y,
                        d_z,
                        on_ground,
                    },
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.yaw, w)?;
                let w = i8::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        bool::deserialize,
                    )),
                    |(entity_id, yaw, pitch, on_ground)| PacketEntityLook {
                        entity_id,
                        yaw,
                        pitch,
                        on_ground,
                    },
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(x, y, z, yaw, pitch)| PacketVehicleMove {
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                    },
                ))(input)
            }
        }

        pub struct PacketOpenBook {
            hand: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketOpenBook {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((VarInt::deserialize,)), |(hand,)| {
                    PacketOpenBook { hand }
                }))(input)
            }
        }

        pub struct PacketOpenSignEntity {
            location: Position,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketOpenSignEntity {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((Position::deserialize,)),
                    |(location,)| PacketOpenSignEntity { location },
                ))(input)
            }
        }

        pub struct PacketCraftRecipeResponse<'a> {
            window_id: i8,
            recipe: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCraftRecipeResponse<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        i8::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(window_id, recipe)| PacketCraftRecipeResponse { window_id, recipe },
                ))(input)
            }
        }

        pub struct PacketAbilities {
            flags: i8,
            flying_speed: f32,
            walking_speed: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAbilities {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.flags, w)?;
                let w = f32::serialize(&self.flying_speed, w)?;
                let w = f32::serialize(&self.walking_speed, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i8::deserialize, f32::deserialize, f32::deserialize)),
                    |(flags, flying_speed, walking_speed)| PacketAbilities {
                        flags,
                        flying_speed,
                        walking_speed,
                    },
                ))(input)
            }
        }

        pub struct PacketEndCombatEvent {
            duration: VarInt,
            entity_id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEndCombatEvent {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.duration, w)?;
                let w = i32::serialize(&self.entity_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, i32::deserialize)),
                    |(duration, entity_id)| PacketEndCombatEvent {
                        duration,
                        entity_id,
                    },
                ))(input)
            }
        }

        pub struct PacketEnterCombatEvent {}

        impl<'t> protocol_lib::Packet<'t> for PacketEnterCombatEvent {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((|i| Ok((i, ())),)), |_| {
                    PacketEnterCombatEvent {}
                }))(input)
            }
        }

        pub struct PacketDeathCombatEvent<'a> {
            player_id: VarInt,
            entity_id: i32,
            message: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDeathCombatEvent<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.player_id, w)?;
                let w = i32::serialize(&self.entity_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.message, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        i32::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(player_id, entity_id, message)| PacketDeathCombatEvent {
                        player_id,
                        entity_id,
                        message,
                    },
                ))(input)
            }
        }

        pub enum PlayerInfoDataItemName<'a> {
            PlayerInfoDataItemName0(VarString<'a>),
            Default(Void),
        }

        impl<'a> PlayerInfoDataItemName<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    PlayerInfoDataItemName::<'a>::PlayerInfoDataItemName0(_) => "0",
                    _ => "",
                }
            }
        }
        pub struct PlayerInfoDataItemProperties0Item<'a> {
            name: VarString<'a>,
            value: VarString<'a>,
            signature: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PlayerInfoDataItemProperties0Item<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.value, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.signature, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        Option::<VarString<'a>>::deserialize,
                    )),
                    |(name, value, signature)| PlayerInfoDataItemProperties0Item {
                        name,
                        value,
                        signature,
                    },
                ))(input)
            }
        }

        pub enum PlayerInfoDataItemProperties<'a> {
            PlayerInfoDataItemProperties0(
                PrefixedArray<PlayerInfoDataItemProperties0Item<'a>, VarInt>,
            ),
            Default(Void),
        }

        impl<'a> PlayerInfoDataItemProperties<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    PlayerInfoDataItemProperties::<'a>::PlayerInfoDataItemProperties0(_) => "0",
                    _ => "",
                }
            }
        }
        pub enum Gamemode {
            Gamemode0(VarInt),
            Gamemode1(VarInt),
            Default(Void),
        }

        impl Gamemode {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Gamemode::Gamemode0(_) => "0",
                    Gamemode::Gamemode1(_) => "1",
                    _ => "",
                }
            }
        }
        pub enum Ping {
            Ping0(VarInt),
            Ping2(VarInt),
            Default(Void),
        }

        impl Ping {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Ping::Ping0(_) => "0",
                    Ping::Ping2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum PlayerInfoDataItemDisplayName<'a> {
            PlayerInfoDataItemDisplayName0(Option<VarString<'a>>),
            PlayerInfoDataItemDisplayName3(Option<VarString<'a>>),
            Default(Void),
        }

        impl<'a> PlayerInfoDataItemDisplayName<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    PlayerInfoDataItemDisplayName::<'a>::PlayerInfoDataItemDisplayName0(_) => "0",
                    PlayerInfoDataItemDisplayName::<'a>::PlayerInfoDataItemDisplayName3(_) => "3",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action, w)?;
                let w = PrefixedArray::<PlayerInfoDataItem<'a>, VarInt>::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedArray::<PlayerInfoDataItem<'a>, VarInt>::deserialize,
                    )),
                    |(action, data)| PacketPlayerInfo { action, data },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
            Default(Void),
        }

        impl<'a> Recipes2<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Recipes2::<'a>::Recipes20(_) => "0",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action, w)?;
                let w = bool::serialize(&self.crafting_book_open, w)?;
                let w = bool::serialize(&self.filtering_craftable, w)?;
                let w = bool::serialize(&self.smelting_book_open, w)?;
                let w = bool::serialize(&self.filtering_smeltable, w)?;
                let w = bool::serialize(&self.blast_furnace_open, w)?;
                let w = bool::serialize(&self.filtering_blast_furnace, w)?;
                let w = bool::serialize(&self.smoker_book_open, w)?;
                let w = bool::serialize(&self.filtering_smoker, w)?;
                let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&self.recipes1, w)?;

                let w = match &self.recipes2 {
                    Recipes2::<'a>::Recipes20(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Recipes2::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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
                    let (input, self_recipes1) =
                        (PrefixedArray::<VarString<'a>, VarInt>::deserialize)(input)?;
                    let (input, self_recipes2) = (|input| match &format!("{}", self_action)[..] {
                        "0" => nom::combinator::map(
                            PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                            Recipes2::<'a>::Recipes20,
                        )(input),
                        _ => {
                            nom::combinator::map(Void::deserialize, Recipes2::<'a>::Default)(input)
                        }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<VarInt, VarInt>::serialize(&self.entity_ids, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedArray::<VarInt, VarInt>::deserialize,)),
                    |(entity_ids,)| PacketEntityDestroy { entity_ids },
                ))(input)
            }
        }

        pub struct PacketRemoveEntityEffect {
            entity_id: VarInt,
            effect_id: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketRemoveEntityEffect {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.effect_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, i8::deserialize)),
                    |(entity_id, effect_id)| PacketRemoveEntityEffect {
                        entity_id,
                        effect_id,
                    },
                ))(input)
            }
        }

        pub struct PacketResourcePackSend<'a> {
            url: VarString<'a>,
            hash: VarString<'a>,
            forced: bool,
            prompt_message: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketResourcePackSend<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.url, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.hash, w)?;
                let w = bool::serialize(&self.forced, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.prompt_message, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        bool::deserialize,
                        Option::<VarString<'a>>::deserialize,
                    )),
                    |(url, hash, forced, prompt_message)| PacketResourcePackSend {
                        url,
                        hash,
                        forced,
                        prompt_message,
                    },
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Nbt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        i64::deserialize,
                        u8::deserialize,
                        u8::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(
                        dimension,
                        world_name,
                        hashed_seed,
                        gamemode,
                        previous_gamemode,
                        is_debug,
                        is_flat,
                        copy_metadata,
                    )| PacketRespawn {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.head_yaw, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, i8::deserialize)),
                    |(entity_id, head_yaw)| PacketEntityHeadRotation {
                        entity_id,
                        head_yaw,
                    },
                ))(input)
            }
        }

        pub struct PacketCamera {
            camera_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCamera {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.camera_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(camera_id,)| PacketCamera { camera_id },
                ))(input)
            }
        }

        pub struct PacketHeldItemSlot {
            slot: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketHeldItemSlot {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.slot, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((i8::deserialize,)), |(slot,)| {
                    PacketHeldItemSlot { slot }
                }))(input)
            }
        }

        pub struct PacketUpdateViewPosition {
            chunk_x: VarInt,
            chunk_z: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateViewPosition {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.chunk_x, w)?;
                let w = VarInt::serialize(&self.chunk_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, VarInt::deserialize)),
                    |(chunk_x, chunk_z)| PacketUpdateViewPosition { chunk_x, chunk_z },
                ))(input)
            }
        }

        pub struct PacketUpdateViewDistance {
            view_distance: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateViewDistance {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.view_distance, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(view_distance,)| PacketUpdateViewDistance { view_distance },
                ))(input)
            }
        }

        pub struct PacketScoreboardDisplayObjective<'a> {
            position: i8,
            name: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketScoreboardDisplayObjective<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.position, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        i8::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(position, name)| PacketScoreboardDisplayObjective { position, name },
                ))(input)
            }
        }

        pub struct PacketEntityMetadata<'a> {
            entity_id: VarInt,
            metadata: Vec<Value<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEntityMetadata<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;

                let mut w = w;
                for (index, item) in self.metadata.iter().enumerate() {
                    w = u8::serialize(
                        &if index == self.metadata.len() - 1 {
                            255
                        } else {
                            index as u8
                        },
                        w,
                    )?;
                    w = str::parse::<VarInt>(item.discriminant())
                        .unwrap()
                        .serialize(w)?;
                    w = {
                        let w = match &item {
                            Value::<'a>::Value0(val) => {
                                let w = i8::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value1(val) => {
                                let w = VarInt::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value2(val) => {
                                let w = f32::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value3(val) => {
                                let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value4(val) => {
                                let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value5(val) => {
                                let w = Option::<VarString<'a>>::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value6(val) => {
                                let w = Slot::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value7(val) => {
                                let w = bool::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value8(val) => {
                                let w = Value8::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value9(val) => {
                                let w = Position::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value10(val) => {
                                let w = Option::<Position>::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value11(val) => {
                                let w = VarInt::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value12(val) => {
                                let w = Option::<Uuid>::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value13(val) => {
                                let w = VarInt::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value14(val) => {
                                let w = Nbt::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value15(val) => {
                                let w = Particle::<'a>::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value16(val) => {
                                let w = Value16::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value17(val) => {
                                let w = VarInt::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Value18(val) => {
                                let w = VarInt::serialize(&val, w)?;
                                w
                            }
                            Value::<'a>::Default(val) => Void::serialize(val, w)?,
                        };

                        w
                    }
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, |mut input| {
                        let mut accum = vec![];
                        loop {
                            let (i, item) = EntityMetadata::<'a>::deserialize(input)?;
                            input = i;
                            let index = item.key;
                            accum.push(item.value);
                            if index == 0xFF {
                                break;
                            }
                        }
                        Ok((input, accum))
                    })),
                    |(entity_id, metadata)| PacketEntityMetadata {
                        entity_id,
                        metadata,
                    },
                ))(input)
            }
        }

        pub struct PacketAttachEntity {
            entity_id: i32,
            vehicle_id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAttachEntity {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.entity_id, w)?;
                let w = i32::serialize(&self.vehicle_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i32::deserialize, i32::deserialize)),
                    |(entity_id, vehicle_id)| PacketAttachEntity {
                        entity_id,
                        vehicle_id,
                    },
                ))(input)
            }
        }

        pub struct PacketEntityVelocity {
            entity_id: VarInt,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityVelocity {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i16::serialize(&self.velocity_x, w)?;
                let w = i16::serialize(&self.velocity_y, w)?;
                let w = i16::serialize(&self.velocity_z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                        i16::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;

                let mut w = w;
                for (i, (k, v)) in self.equipments.iter().enumerate() {
                    let k = if i == self.equipments.len() - 1 {
                        *k | (1i8 << 7)
                    } else {
                        *k
                    };
                    let ww = i8::serialize(&k, w)?;
                    w = v.serialize(ww)?;
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, |mut input| {
                        let mut val = std::collections::HashMap::new();
                        let mut i = input;
                        loop {
                            let (i, (k_, v)) =
                                nom::sequence::tuple((i8::deserialize, Slot::deserialize))(i)?;
                            input = i;
                            let k = k_ & 0x7F;
                            val.insert(k, v);
                            if k != k_ {
                                break;
                            }
                        }
                        let input = i;
                        Ok((input, val))
                    })),
                    |(entity_id, equipments)| PacketEntityEquipment {
                        entity_id,
                        equipments,
                    },
                ))(input)
            }
        }

        pub struct PacketExperience {
            experience_bar: f32,
            level: VarInt,
            total_experience: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketExperience {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.experience_bar, w)?;
                let w = VarInt::serialize(&self.level, w)?;
                let w = VarInt::serialize(&self.total_experience, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f32::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                    )),
                    |(experience_bar, level, total_experience)| PacketExperience {
                        experience_bar,
                        level,
                        total_experience,
                    },
                ))(input)
            }
        }

        pub struct PacketUpdateHealth {
            health: f32,
            food: VarInt,
            food_saturation: f32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateHealth {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.health, w)?;
                let w = VarInt::serialize(&self.food, w)?;
                let w = f32::serialize(&self.food_saturation, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((f32::deserialize, VarInt::deserialize, f32::deserialize)),
                    |(health, food, food_saturation)| PacketUpdateHealth {
                        health,
                        food,
                        food_saturation,
                    },
                ))(input)
            }
        }

        pub enum DisplayText<'a> {
            DisplayText0(VarString<'a>),
            DisplayText2(VarString<'a>),
            Default(Void),
        }

        impl<'a> DisplayText<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    DisplayText::<'a>::DisplayText0(_) => "0",
                    DisplayText::<'a>::DisplayText2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum ScoreboardObjectiveType {
            ScoreboardObjectiveType0(VarInt),
            ScoreboardObjectiveType2(VarInt),
            Default(Void),
        }

        impl ScoreboardObjectiveType {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    ScoreboardObjectiveType::ScoreboardObjectiveType0(_) => "0",
                    ScoreboardObjectiveType::ScoreboardObjectiveType2(_) => "2",
                    _ => "",
                }
            }
        }
        pub struct PacketScoreboardObjective<'a> {
            name: VarString<'a>,
            action: i8,
            display_text: DisplayText<'a>,
            r_type: ScoreboardObjectiveType,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketScoreboardObjective<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = i8::serialize(&self.action, w)?;

                let w = match &self.display_text {
                    DisplayText::<'a>::DisplayText0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    DisplayText::<'a>::DisplayText2(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    DisplayText::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.r_type {
                    ScoreboardObjectiveType::ScoreboardObjectiveType0(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    ScoreboardObjectiveType::ScoreboardObjectiveType2(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    ScoreboardObjectiveType::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_action) = (i8::deserialize)(input)?;
                    let (input, self_display_text) = (|input| match &format!("{}", self_action)[..]
                    {
                        "0" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            DisplayText::<'a>::DisplayText0,
                        )(input),
                        "2" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            DisplayText::<'a>::DisplayText2,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, DisplayText::<'a>::Default)(
                            input,
                        ),
                    })(input)?;
                    let (input, self_r_type) = (|input| match &format!("{}", self_action)[..] {
                        "0" => nom::combinator::map(
                            VarInt::deserialize,
                            ScoreboardObjectiveType::ScoreboardObjectiveType0,
                        )(input),
                        "2" => nom::combinator::map(
                            VarInt::deserialize,
                            ScoreboardObjectiveType::ScoreboardObjectiveType2,
                        )(input),
                        _ => nom::combinator::map(
                            Void::deserialize,
                            ScoreboardObjectiveType::Default,
                        )(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = PrefixedArray::<VarInt, VarInt>::serialize(&self.passengers, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedArray::<VarInt, VarInt>::deserialize,
                    )),
                    |(entity_id, passengers)| PacketSetPassengers {
                        entity_id,
                        passengers,
                    },
                ))(input)
            }
        }

        pub enum TeamsName<'a> {
            TeamsName0(VarString<'a>),
            TeamsName2(VarString<'a>),
            Default(Void),
        }

        impl<'a> TeamsName<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    TeamsName::<'a>::TeamsName0(_) => "0",
                    TeamsName::<'a>::TeamsName2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum FriendlyFire {
            FriendlyFire0(i8),
            FriendlyFire2(i8),
            Default(Void),
        }

        impl FriendlyFire {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    FriendlyFire::FriendlyFire0(_) => "0",
                    FriendlyFire::FriendlyFire2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum NameTagVisibility<'a> {
            NameTagVisibility0(VarString<'a>),
            NameTagVisibility2(VarString<'a>),
            Default(Void),
        }

        impl<'a> NameTagVisibility<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    NameTagVisibility::<'a>::NameTagVisibility0(_) => "0",
                    NameTagVisibility::<'a>::NameTagVisibility2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum CollisionRule<'a> {
            CollisionRule0(VarString<'a>),
            CollisionRule2(VarString<'a>),
            Default(Void),
        }

        impl<'a> CollisionRule<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    CollisionRule::<'a>::CollisionRule0(_) => "0",
                    CollisionRule::<'a>::CollisionRule2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum Formatting {
            Formatting0(VarInt),
            Formatting2(VarInt),
            Default(Void),
        }

        impl Formatting {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Formatting::Formatting0(_) => "0",
                    Formatting::Formatting2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum Prefix<'a> {
            Prefix0(VarString<'a>),
            Prefix2(VarString<'a>),
            Default(Void),
        }

        impl<'a> Prefix<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Prefix::<'a>::Prefix0(_) => "0",
                    Prefix::<'a>::Prefix2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum Suffix<'a> {
            Suffix0(VarString<'a>),
            Suffix2(VarString<'a>),
            Default(Void),
        }

        impl<'a> Suffix<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Suffix::<'a>::Suffix0(_) => "0",
                    Suffix::<'a>::Suffix2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum Players<'a> {
            Players0(VarStringArray<'a>),
            Players3(VarStringArray<'a>),
            Players4(VarStringArray<'a>),
            Default(Void),
        }

        impl<'a> Players<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Players::<'a>::Players0(_) => "0",
                    Players::<'a>::Players3(_) => "3",
                    Players::<'a>::Players4(_) => "4",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.team, w)?;
                let w = i8::serialize(&self.mode, w)?;

                let w = match &self.name {
                    TeamsName::<'a>::TeamsName0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    TeamsName::<'a>::TeamsName2(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    TeamsName::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.friendly_fire {
                    FriendlyFire::FriendlyFire0(val) => {
                        let w = i8::serialize(&val, w)?;
                        w
                    }
                    FriendlyFire::FriendlyFire2(val) => {
                        let w = i8::serialize(&val, w)?;
                        w
                    }
                    FriendlyFire::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.name_tag_visibility {
                    NameTagVisibility::<'a>::NameTagVisibility0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    NameTagVisibility::<'a>::NameTagVisibility2(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    NameTagVisibility::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.collision_rule {
                    CollisionRule::<'a>::CollisionRule0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    CollisionRule::<'a>::CollisionRule2(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    CollisionRule::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.formatting {
                    Formatting::Formatting0(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Formatting::Formatting2(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Formatting::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.prefix {
                    Prefix::<'a>::Prefix0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Prefix::<'a>::Prefix2(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Prefix::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.suffix {
                    Suffix::<'a>::Suffix0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Suffix::<'a>::Suffix2(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Suffix::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.players {
                    Players::<'a>::Players0(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Players::<'a>::Players3(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Players::<'a>::Players4(val) => {
                        let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Players::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_team) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_mode) = (i8::deserialize)(input)?;
                    let (input, self_name) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            TeamsName::<'a>::TeamsName0,
                        )(input),
                        "2" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            TeamsName::<'a>::TeamsName2,
                        )(input),
                        _ => {
                            nom::combinator::map(Void::deserialize, TeamsName::<'a>::Default)(input)
                        }
                    })(input)?;
                    let (input, self_friendly_fire) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => nom::combinator::map(i8::deserialize, FriendlyFire::FriendlyFire0)(
                            input,
                        ),
                        "2" => nom::combinator::map(i8::deserialize, FriendlyFire::FriendlyFire2)(
                            input,
                        ),
                        _ => nom::combinator::map(Void::deserialize, FriendlyFire::Default)(input),
                    })(input)?;
                    let (input, self_name_tag_visibility) =
                        (|input| match &format!("{}", self_mode)[..] {
                            "0" => nom::combinator::map(
                                PrefixedString::<'a, VarInt>::deserialize,
                                NameTagVisibility::<'a>::NameTagVisibility0,
                            )(input),
                            "2" => nom::combinator::map(
                                PrefixedString::<'a, VarInt>::deserialize,
                                NameTagVisibility::<'a>::NameTagVisibility2,
                            )(input),
                            _ => nom::combinator::map(
                                Void::deserialize,
                                NameTagVisibility::<'a>::Default,
                            )(input),
                        })(input)?;
                    let (input, self_collision_rule) = (|input| match &format!("{}", self_mode)[..]
                    {
                        "0" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            CollisionRule::<'a>::CollisionRule0,
                        )(input),
                        "2" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            CollisionRule::<'a>::CollisionRule2,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, CollisionRule::<'a>::Default)(
                            input,
                        ),
                    })(input)?;
                    let (input, self_formatting) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => nom::combinator::map(VarInt::deserialize, Formatting::Formatting0)(
                            input,
                        ),
                        "2" => nom::combinator::map(VarInt::deserialize, Formatting::Formatting2)(
                            input,
                        ),
                        _ => nom::combinator::map(Void::deserialize, Formatting::Default)(input),
                    })(input)?;
                    let (input, self_prefix) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            Prefix::<'a>::Prefix0,
                        )(input),
                        "2" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            Prefix::<'a>::Prefix2,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Prefix::<'a>::Default)(input),
                    })(input)?;
                    let (input, self_suffix) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            Suffix::<'a>::Suffix0,
                        )(input),
                        "2" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            Suffix::<'a>::Suffix2,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Suffix::<'a>::Default)(input),
                    })(input)?;
                    let (input, self_players) = (|input| match &format!("{}", self_mode)[..] {
                        "0" => nom::combinator::map(
                            PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                            Players::<'a>::Players0,
                        )(input),
                        "3" => nom::combinator::map(
                            PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                            Players::<'a>::Players3,
                        )(input),
                        "4" => nom::combinator::map(
                            PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                            Players::<'a>::Players4,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Players::<'a>::Default)(input),
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
            ScoreboardScoreValue1(Void),
            Default(VarInt),
        }

        impl ScoreboardScoreValue {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    ScoreboardScoreValue::ScoreboardScoreValue1(_) => "1",
                    _ => "",
                }
            }
        }
        pub struct PacketScoreboardScore<'a> {
            item_name: VarString<'a>,
            action: VarInt,
            score_name: VarString<'a>,
            value: ScoreboardScoreValue,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketScoreboardScore<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.item_name, w)?;
                let w = VarInt::serialize(&self.action, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.score_name, w)?;

                let w = match &self.value {
                    ScoreboardScoreValue::ScoreboardScoreValue1(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    ScoreboardScoreValue::Default(val) => VarInt::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_item_name) =
                        (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_score_name) =
                        (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_value) = (|input| match &format!("{}", self_action)[..] {
                        "1" => nom::combinator::map(
                            Void::deserialize,
                            ScoreboardScoreValue::ScoreboardScoreValue1,
                        )(input),
                        _ => nom::combinator::map(
                            VarInt::deserialize,
                            ScoreboardScoreValue::Default,
                        )(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = f32::serialize(&self.angle, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((Position::deserialize, f32::deserialize)),
                    |(location, angle)| PacketSpawnPosition { location, angle },
                ))(input)
            }
        }

        pub struct PacketUpdateTime {
            age: i64,
            time: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUpdateTime {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.age, w)?;
                let w = i64::serialize(&self.time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i64::deserialize, i64::deserialize)),
                    |(age, time)| PacketUpdateTime { age, time },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.sound_id, w)?;
                let w = VarInt::serialize(&self.sound_category, w)?;
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = f32::serialize(&self.volume, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(sound_id, sound_category, entity_id, volume, pitch)| {
                        PacketEntitySoundEffect {
                            sound_id,
                            sound_category,
                            entity_id,
                            volume,
                            pitch,
                        }
                    },
                ))(input)
            }
        }

        pub enum Source {
            Source3(VarInt),
            Source1(VarInt),
            Default(Void),
        }

        impl Source {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Source::Source3(_) => "3",
                    Source::Source1(_) => "1",
                    _ => "",
                }
            }
        }
        pub enum Sound<'a> {
            Sound3(VarString<'a>),
            Sound2(VarString<'a>),
            Default(Void),
        }

        impl<'a> Sound<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Sound::<'a>::Sound3(_) => "3",
                    Sound::<'a>::Sound2(_) => "2",
                    _ => "",
                }
            }
        }
        pub struct PacketStopSound<'a> {
            flags: i8,
            source: Source,
            sound: Sound<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketStopSound<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.flags, w)?;

                let w = match &self.source {
                    Source::Source3(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Source::Source1(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    Source::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.sound {
                    Sound::<'a>::Sound3(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Sound::<'a>::Sound2(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    Sound::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_flags) = (i8::deserialize)(input)?;
                    let (input, self_source) = (|input| match &format!("{}", self_flags)[..] {
                        "3" => nom::combinator::map(VarInt::deserialize, Source::Source3)(input),
                        "1" => nom::combinator::map(VarInt::deserialize, Source::Source1)(input),
                        _ => nom::combinator::map(Void::deserialize, Source::Default)(input),
                    })(input)?;
                    let (input, self_sound) = (|input| match &format!("{}", self_flags)[..] {
                        "3" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            Sound::<'a>::Sound3,
                        )(input),
                        "2" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            Sound::<'a>::Sound2,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Sound::<'a>::Default)(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.header, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.footer, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(header, footer)| PacketPlayerlistHeader { header, footer },
                ))(input)
            }
        }

        pub struct PacketCollect {
            collected_entity_id: VarInt,
            collector_entity_id: VarInt,
            pickup_item_count: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketCollect {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.collected_entity_id, w)?;
                let w = VarInt::serialize(&self.collector_entity_id, w)?;
                let w = VarInt::serialize(&self.pickup_item_count, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                    )),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.uuid, w)?;
                let w = f64::serialize(&self.amount, w)?;
                let w = i8::serialize(&self.operation, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((Uuid::deserialize, f64::deserialize, i8::deserialize)),
                    |(uuid, amount, operation)| Modifier {
                        uuid,
                        amount,
                        operation,
                    },
                ))(input)
            }
        }

        pub struct EntityUpdateAttrsProperty<'a> {
            key: VarString<'a>,
            value: f64,
            modifiers: VarArray<Modifier>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for EntityUpdateAttrsProperty<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.key, w)?;
                let w = f64::serialize(&self.value, w)?;
                let w = PrefixedArray::<Modifier, VarInt>::serialize(&self.modifiers, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        f64::deserialize,
                        PrefixedArray::<Modifier, VarInt>::deserialize,
                    )),
                    |(key, value, modifiers)| EntityUpdateAttrsProperty {
                        key,
                        value,
                        modifiers,
                    },
                ))(input)
            }
        }

        pub struct PacketEntityUpdateAttributes<'a> {
            entity_id: VarInt,
            properties: VarArray<EntityUpdateAttrsProperty<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEntityUpdateAttributes<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = PrefixedArray::<EntityUpdateAttrsProperty<'a>, VarInt>::serialize(
                    &self.properties,
                    w,
                )?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedArray::<EntityUpdateAttrsProperty<'a>, VarInt>::deserialize,
                    )),
                    |(entity_id, properties)| PacketEntityUpdateAttributes {
                        entity_id,
                        properties,
                    },
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = i8::serialize(&self.effect_id, w)?;
                let w = i8::serialize(&self.amplifier, w)?;
                let w = VarInt::serialize(&self.duration, w)?;
                let w = i8::serialize(&self.hide_particles, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        i8::deserialize,
                        i8::deserialize,
                        VarInt::deserialize,
                        i8::deserialize,
                    )),
                    |(entity_id, effect_id, amplifier, duration, hide_particles)| {
                        PacketEntityEffect {
                            entity_id,
                            effect_id,
                            amplifier,
                            duration,
                            hide_particles,
                        }
                    },
                ))(input)
            }
        }

        pub struct PacketSelectAdvancementTab<'a> {
            id: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSelectAdvancementTab<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Option::<VarString<'a>>::serialize(&self.id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((Option::<VarString<'a>>::deserialize,)),
                    |(id,)| PacketSelectAdvancementTab { id },
                ))(input)
            }
        }

        pub struct CraftingShapeless<'a> {
            group: VarString<'a>,
            ingredients: VarArray<VarArray<Slot>>,
            result: Slot,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for CraftingShapeless<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;
                let w = PrefixedArray::<VarArray<Slot>, VarInt>::serialize(&self.ingredients, w)?;
                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedArray::<VarArray<Slot>, VarInt>::deserialize,
                        Slot::deserialize,
                    )),
                    |(group, ingredients, result)| CraftingShapeless {
                        group,
                        ingredients,
                        result,
                    },
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.width, w)?;
                let w = VarInt::serialize(&self.height, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;

                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        |_| todo!(),
                        Slot::deserialize,
                    )),
                    |(width, height, group, ingredients, result)| CraftingShaped {
                        width,
                        height,
                        group,
                        ingredients,
                        result,
                    },
                ))(input)
            }
        }

        pub struct Stonecutting<'a> {
            group: VarString<'a>,
            ingredient: VarArray<Slot>,
            result: Slot,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Stonecutting<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.group, w)?;
                let w = PrefixedArray::<Slot, VarInt>::serialize(&self.ingredient, w)?;
                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedArray::<Slot, VarInt>::deserialize,
                        Slot::deserialize,
                    )),
                    |(group, ingredient, result)| Stonecutting {
                        group,
                        ingredient,
                        result,
                    },
                ))(input)
            }
        }

        pub struct Smithing {
            base: VarArray<Slot>,
            addition: VarArray<Slot>,
            result: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for Smithing {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<Slot, VarInt>::serialize(&self.base, w)?;
                let w = PrefixedArray::<Slot, VarInt>::serialize(&self.addition, w)?;
                let w = Slot::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedArray::<Slot, VarInt>::deserialize,
                        PrefixedArray::<Slot, VarInt>::deserialize,
                        Slot::deserialize,
                    )),
                    |(base, addition, result)| Smithing {
                        base,
                        addition,
                        result,
                    },
                ))(input)
            }
        }

        pub enum RecipeData<'a> {
            CraftingShapeless(CraftingShapeless<'a>),
            CraftingShaped(CraftingShaped<'a>),
            CraftingSpecialArmordye(Void),
            CraftingSpecialBookcloning(Void),
            CraftingSpecialMapcloning(Void),
            CraftingSpecialMapextending(Void),
            CraftingSpecialFireworkRocket(Void),
            CraftingSpecialFireworkStar(Void),
            CraftingSpecialFireworkStarFade(Void),
            CraftingSpecialRepairitem(Void),
            CraftingSpecialTippedarrow(Void),
            CraftingSpecialBannerduplicate(Void),
            CraftingSpecialBanneraddpattern(Void),
            CraftingSpecialShielddecoration(Void),
            CraftingSpecialShulkerboxcoloring(Void),
            CraftingSpecialSuspiciousstew(Void),
            Smelting(MinecraftSmeltingFormat<'a>),
            Blasting(MinecraftSmeltingFormat<'a>),
            Smoking(MinecraftSmeltingFormat<'a>),
            CampfireCooking(MinecraftSmeltingFormat<'a>),
            Stonecutting(Stonecutting<'a>),
            Smithing(Smithing),
            Default(Void),
        }

        impl<'a> RecipeData<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    RecipeData::<'a>::CraftingShapeless(_) => "minecraft:crafting_shapeless",
                    RecipeData::<'a>::CraftingShaped(_) => "minecraft:crafting_shaped",
                    RecipeData::<'a>::CraftingSpecialArmordye(_) => {
                        "minecraft:crafting_special_armordye"
                    }
                    RecipeData::<'a>::CraftingSpecialBookcloning(_) => {
                        "minecraft:crafting_special_bookcloning"
                    }
                    RecipeData::<'a>::CraftingSpecialMapcloning(_) => {
                        "minecraft:crafting_special_mapcloning"
                    }
                    RecipeData::<'a>::CraftingSpecialMapextending(_) => {
                        "minecraft:crafting_special_mapextending"
                    }
                    RecipeData::<'a>::CraftingSpecialFireworkRocket(_) => {
                        "minecraft:crafting_special_firework_rocket"
                    }
                    RecipeData::<'a>::CraftingSpecialFireworkStar(_) => {
                        "minecraft:crafting_special_firework_star"
                    }
                    RecipeData::<'a>::CraftingSpecialFireworkStarFade(_) => {
                        "minecraft:crafting_special_firework_star_fade"
                    }
                    RecipeData::<'a>::CraftingSpecialRepairitem(_) => {
                        "minecraft:crafting_special_repairitem"
                    }
                    RecipeData::<'a>::CraftingSpecialTippedarrow(_) => {
                        "minecraft:crafting_special_tippedarrow"
                    }
                    RecipeData::<'a>::CraftingSpecialBannerduplicate(_) => {
                        "minecraft:crafting_special_bannerduplicate"
                    }
                    RecipeData::<'a>::CraftingSpecialBanneraddpattern(_) => {
                        "minecraft:crafting_special_banneraddpattern"
                    }
                    RecipeData::<'a>::CraftingSpecialShielddecoration(_) => {
                        "minecraft:crafting_special_shielddecoration"
                    }
                    RecipeData::<'a>::CraftingSpecialShulkerboxcoloring(_) => {
                        "minecraft:crafting_special_shulkerboxcoloring"
                    }
                    RecipeData::<'a>::CraftingSpecialSuspiciousstew(_) => {
                        "minecraft:crafting_special_suspiciousstew"
                    }
                    RecipeData::<'a>::Smelting(_) => "minecraft:smelting",
                    RecipeData::<'a>::Blasting(_) => "minecraft:blasting",
                    RecipeData::<'a>::Smoking(_) => "minecraft:smoking",
                    RecipeData::<'a>::CampfireCooking(_) => "minecraft:campfire_cooking",
                    RecipeData::<'a>::Stonecutting(_) => "minecraft:stonecutting",
                    RecipeData::<'a>::Smithing(_) => "minecraft:smithing",
                    _ => "",
                }
            }
        }
        pub struct RecipesItem<'a> {
            r_type: VarString<'a>,
            recipe_id: VarString<'a>,
            data: RecipeData<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for RecipesItem<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.r_type, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe_id, w)?;

                let w = match &self.data {
                    RecipeData::<'a>::CraftingShapeless(val) => {
                        let w = CraftingShapeless::<'a>::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingShaped(val) => {
                        let w = CraftingShaped::<'a>::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialArmordye(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialBookcloning(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialMapcloning(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialMapextending(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialFireworkRocket(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialFireworkStar(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialFireworkStarFade(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialRepairitem(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialTippedarrow(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialBannerduplicate(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialBanneraddpattern(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialShielddecoration(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialShulkerboxcoloring(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CraftingSpecialSuspiciousstew(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::Smelting(val) => {
                        let w = MinecraftSmeltingFormat::<'a>::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::Blasting(val) => {
                        let w = MinecraftSmeltingFormat::<'a>::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::Smoking(val) => {
                        let w = MinecraftSmeltingFormat::<'a>::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::CampfireCooking(val) => {
                        let w = MinecraftSmeltingFormat::<'a>::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::Stonecutting(val) => {
                        let w = Stonecutting::<'a>::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::Smithing(val) => {
                        let w = Smithing::serialize(&val, w)?;
                        w
                    }
                    RecipeData::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_r_type) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_recipe_id) =
                        (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_data) = (|input| match &format!("{}", self_r_type)[..] {
                        "minecraft:crafting_shapeless" => nom::combinator::map(
                            CraftingShapeless::<'a>::deserialize,
                            RecipeData::<'a>::CraftingShapeless,
                        )(input),
                        "minecraft:crafting_shaped" => nom::combinator::map(
                            CraftingShaped::<'a>::deserialize,
                            RecipeData::<'a>::CraftingShaped,
                        )(input),
                        "minecraft:crafting_special_armordye" => nom::combinator::map(
                            Void::deserialize,
                            RecipeData::<'a>::CraftingSpecialArmordye,
                        )(input),
                        "minecraft:crafting_special_bookcloning" => nom::combinator::map(
                            Void::deserialize,
                            RecipeData::<'a>::CraftingSpecialBookcloning,
                        )(input),
                        "minecraft:crafting_special_mapcloning" => nom::combinator::map(
                            Void::deserialize,
                            RecipeData::<'a>::CraftingSpecialMapcloning,
                        )(input),
                        "minecraft:crafting_special_mapextending" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialMapextending,
                            )(input)
                        }
                        "minecraft:crafting_special_firework_rocket" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialFireworkRocket,
                            )(input)
                        }
                        "minecraft:crafting_special_firework_star" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialFireworkStar,
                            )(input)
                        }
                        "minecraft:crafting_special_firework_star_fade" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialFireworkStarFade,
                            )(input)
                        }
                        "minecraft:crafting_special_repairitem" => nom::combinator::map(
                            Void::deserialize,
                            RecipeData::<'a>::CraftingSpecialRepairitem,
                        )(input),
                        "minecraft:crafting_special_tippedarrow" => nom::combinator::map(
                            Void::deserialize,
                            RecipeData::<'a>::CraftingSpecialTippedarrow,
                        )(input),
                        "minecraft:crafting_special_bannerduplicate" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialBannerduplicate,
                            )(input)
                        }
                        "minecraft:crafting_special_banneraddpattern" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialBanneraddpattern,
                            )(input)
                        }
                        "minecraft:crafting_special_shielddecoration" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialShielddecoration,
                            )(input)
                        }
                        "minecraft:crafting_special_shulkerboxcoloring" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialShulkerboxcoloring,
                            )(input)
                        }
                        "minecraft:crafting_special_suspiciousstew" => {
                            nom::combinator::map(
                                Void::deserialize,
                                RecipeData::<'a>::CraftingSpecialSuspiciousstew,
                            )(input)
                        }
                        "minecraft:smelting" => nom::combinator::map(
                            MinecraftSmeltingFormat::<'a>::deserialize,
                            RecipeData::<'a>::Smelting,
                        )(input),
                        "minecraft:blasting" => nom::combinator::map(
                            MinecraftSmeltingFormat::<'a>::deserialize,
                            RecipeData::<'a>::Blasting,
                        )(input),
                        "minecraft:smoking" => nom::combinator::map(
                            MinecraftSmeltingFormat::<'a>::deserialize,
                            RecipeData::<'a>::Smoking,
                        )(input),
                        "minecraft:campfire_cooking" => nom::combinator::map(
                            MinecraftSmeltingFormat::<'a>::deserialize,
                            RecipeData::<'a>::CampfireCooking,
                        )(input),
                        "minecraft:stonecutting" => nom::combinator::map(
                            Stonecutting::<'a>::deserialize,
                            RecipeData::<'a>::Stonecutting,
                        )(input),
                        "minecraft:smithing" => nom::combinator::map(
                            Smithing::deserialize,
                            RecipeData::<'a>::Smithing,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, RecipeData::<'a>::Default)(
                            input,
                        ),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<RecipesItem<'a>, VarInt>::serialize(&self.recipes, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedArray::<RecipesItem<'a>, VarInt>::deserialize,)),
                    |(recipes,)| PacketDeclareRecipes { recipes },
                ))(input)
            }
        }

        pub struct TagsTag<'a> {
            tag_type: VarString<'a>,
            tags: VarArray<Tag<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for TagsTag<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.tag_type, w)?;
                let w = PrefixedArray::<Tag<'a>, VarInt>::serialize(&self.tags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedArray::<Tag<'a>, VarInt>::deserialize,
                    )),
                    |(tag_type, tags)| TagsTag { tag_type, tags },
                ))(input)
            }
        }

        pub struct PacketTags<'a> {
            tags: VarArray<TagsTag<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketTags<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedArray::<TagsTag<'a>, VarInt>::serialize(&self.tags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedArray::<TagsTag<'a>, VarInt>::deserialize,)),
                    |(tags,)| PacketTags { tags },
                ))(input)
            }
        }

        pub struct PacketAcknowledgePlayerDigging {
            location: Position,
            block: VarInt,
            status: VarInt,
            successful: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAcknowledgePlayerDigging {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.block, w)?;
                let w = VarInt::serialize(&self.status, w)?;
                let w = bool::serialize(&self.successful, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Position::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                    )),
                    |(location, block, status, successful)| PacketAcknowledgePlayerDigging {
                        location,
                        block,
                        status,
                        successful,
                    },
                ))(input)
            }
        }

        pub enum SculkVibrationSignalDestination {
            SculkVibrationSignalDestinationBlock(Position),
            SculkVibrationSignalDestinationEntityId(VarInt),
            Default(Void),
        }

        impl SculkVibrationSignalDestination {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    SculkVibrationSignalDestination::SculkVibrationSignalDestinationBlock(_) => {
                        "block"
                    }
                    SculkVibrationSignalDestination::SculkVibrationSignalDestinationEntityId(_) => {
                        "entityId"
                    }
                    _ => "",
                }
            }
        }
        pub struct PacketSculkVibrationSignal<'a> {
            source_position: Position,
            destination_identifier: VarString<'a>,
            destination: SculkVibrationSignalDestination,
            arrival_ticks: VarInt,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSculkVibrationSignal<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.source_position, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.destination_identifier, w)?;

                let w = match &self.destination {
                    SculkVibrationSignalDestination::SculkVibrationSignalDestinationBlock(val) => {
                        let w = Position::serialize(&val, w)?;
                        w
                    }
                    SculkVibrationSignalDestination::SculkVibrationSignalDestinationEntityId(
                        val,
                    ) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    SculkVibrationSignalDestination::Default(val) => Void::serialize(val, w)?,
                };

                let w = VarInt::serialize(&self.arrival_ticks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_source_position) = (Position::deserialize)(input)?;
                    let (input, self_destination_identifier) =
                        (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                    let (input, self_destination) = (|input| {
                        match &format!("{}", self_destination_identifier)[..] {
"block" => nom::combinator::map(Position::deserialize, SculkVibrationSignalDestination::SculkVibrationSignalDestinationBlock)(input),
"entityId" => nom::combinator::map(VarInt::deserialize, SculkVibrationSignalDestination::SculkVibrationSignalDestinationEntityId)(input),
 _ => nom::combinator::map(Void::deserialize, SculkVibrationSignalDestination::Default)(input)}
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.reset, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((bool::deserialize,)), |(reset,)| {
                    PacketClearTitles { reset }
                }))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        VarLong::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                    )),
                    |(
                        x,
                        z,
                        old_diameter,
                        new_diameter,
                        speed,
                        portal_teleport_boundary,
                        warning_blocks,
                        warning_time,
                    )| PacketInitializeWorldBorder {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(text,)| PacketActionBar { text },
                ))(input)
            }
        }

        pub struct PacketWorldBorderCenter {
            x: f64,
            z: f64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderCenter {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.z, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((f64::deserialize, f64::deserialize)),
                    |(x, z)| PacketWorldBorderCenter { x, z },
                ))(input)
            }
        }

        pub struct PacketWorldBorderLerpSize {
            old_diameter: f64,
            new_diameter: f64,
            speed: VarLong,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderLerpSize {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.old_diameter, w)?;
                let w = f64::serialize(&self.new_diameter, w)?;
                let w = VarLong::serialize(&self.speed, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f64::deserialize,
                        f64::deserialize,
                        VarLong::deserialize,
                    )),
                    |(old_diameter, new_diameter, speed)| PacketWorldBorderLerpSize {
                        old_diameter,
                        new_diameter,
                        speed,
                    },
                ))(input)
            }
        }

        pub struct PacketWorldBorderSize {
            diameter: f64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderSize {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.diameter, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((f64::deserialize,)), |(diameter,)| {
                    PacketWorldBorderSize { diameter }
                }))(input)
            }
        }

        pub struct PacketWorldBorderWarningDelay {
            warning_time: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderWarningDelay {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.warning_time, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(warning_time,)| PacketWorldBorderWarningDelay { warning_time },
                ))(input)
            }
        }

        pub struct PacketWorldBorderWarningReach {
            warning_blocks: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketWorldBorderWarningReach {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.warning_blocks, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(warning_blocks,)| PacketWorldBorderWarningReach { warning_blocks },
                ))(input)
            }
        }

        pub struct PacketPing {
            id: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPing {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((i32::deserialize,)), |(id,)| {
                    PacketPing { id }
                }))(input)
            }
        }

        pub struct PacketSetTitleSubtitle<'a> {
            text: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSetTitleSubtitle<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(text,)| PacketSetTitleSubtitle { text },
                ))(input)
            }
        }

        pub struct PacketSetTitleText<'a> {
            text: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketSetTitleText<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(text,)| PacketSetTitleText { text },
                ))(input)
            }
        }

        pub struct PacketSetTitleTime {
            fade_in: i32,
            stay: i32,
            fade_out: i32,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetTitleTime {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.fade_in, w)?;
                let w = i32::serialize(&self.stay, w)?;
                let w = i32::serialize(&self.fade_out, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i32::deserialize, i32::deserialize, i32::deserialize)),
                    |(fade_in, stay, fade_out)| PacketSetTitleTime {
                        fade_in,
                        stay,
                        fade_out,
                    },
                ))(input)
            }
        }

        pub struct PacketSimulationDistance {
            distance: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSimulationDistance {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.distance, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(distance,)| PacketSimulationDistance { distance },
                ))(input)
            }
        }

        pub enum Params<'a> {
            SpawnEntity(PacketSpawnEntity),
            SpawnEntityExperienceOrb(PacketSpawnEntityExperienceOrb),
            SpawnEntityLiving(PacketSpawnEntityLiving),
            SpawnEntityPainting(PacketSpawnEntityPainting),
            NamedEntitySpawn(PacketNamedEntitySpawn),
            ParamsAnimation(PacketAnimation),
            Statistics(PacketStatistics),
            Advancements(PacketAdvancements<'a>),
            BlockBreakAnimation(PacketBlockBreakAnimation),
            TileEntityData(PacketTileEntityData),
            BlockAction(PacketBlockAction),
            BlockChange(PacketBlockChange),
            BossBar(PacketBossBar<'a>),
            ParamsDifficulty(PacketDifficulty),
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
            ParamsEntityStatus(PacketEntityStatus),
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
            ParamsPosition(PacketPosition),
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
            ParamsEntityMetadata(PacketEntityMetadata<'a>),
            AttachEntity(PacketAttachEntity),
            EntityVelocity(PacketEntityVelocity),
            EntityEquipment(PacketEntityEquipment),
            ParamsExperience(PacketExperience),
            UpdateHealth(PacketUpdateHealth),
            ScoreboardObjective(PacketScoreboardObjective<'a>),
            SetPassengers(PacketSetPassengers),
            Teams(PacketTeams<'a>),
            ScoreboardScore(PacketScoreboardScore<'a>),
            ParamsSimulationDistance(PacketSimulationDistance),
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
            ParamsTags(PacketTags<'a>),
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
            ParamsPing(PacketPing),
            SetTitleSubtitle(PacketSetTitleSubtitle<'a>),
            SetTitleText(PacketSetTitleText<'a>),
            SetTitleTime(PacketSetTitleTime),
            Default(Void),
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::<'a>::SpawnEntity(_) => "spawn_entity",
                    Params::<'a>::SpawnEntityExperienceOrb(_) => "spawn_entity_experience_orb",
                    Params::<'a>::SpawnEntityLiving(_) => "spawn_entity_living",
                    Params::<'a>::SpawnEntityPainting(_) => "spawn_entity_painting",
                    Params::<'a>::NamedEntitySpawn(_) => "named_entity_spawn",
                    Params::<'a>::ParamsAnimation(_) => "animation",
                    Params::<'a>::Statistics(_) => "statistics",
                    Params::<'a>::Advancements(_) => "advancements",
                    Params::<'a>::BlockBreakAnimation(_) => "block_break_animation",
                    Params::<'a>::TileEntityData(_) => "tile_entity_data",
                    Params::<'a>::BlockAction(_) => "block_action",
                    Params::<'a>::BlockChange(_) => "block_change",
                    Params::<'a>::BossBar(_) => "boss_bar",
                    Params::<'a>::ParamsDifficulty(_) => "difficulty",
                    Params::<'a>::TabComplete(_) => "tab_complete",
                    Params::<'a>::DeclareCommands(_) => "declare_commands",
                    Params::<'a>::FacePlayer(_) => "face_player",
                    Params::<'a>::NbtQueryResponse(_) => "nbt_query_response",
                    Params::<'a>::Chat(_) => "chat",
                    Params::<'a>::MultiBlockChange(_) => "multi_block_change",
                    Params::<'a>::CloseWindow(_) => "close_window",
                    Params::<'a>::OpenWindow(_) => "open_window",
                    Params::<'a>::WindowItems(_) => "window_items",
                    Params::<'a>::CraftProgressBar(_) => "craft_progress_bar",
                    Params::<'a>::SetSlot(_) => "set_slot",
                    Params::<'a>::SetCooldown(_) => "set_cooldown",
                    Params::<'a>::CustomPayload(_) => "custom_payload",
                    Params::<'a>::NamedSoundEffect(_) => "named_sound_effect",
                    Params::<'a>::KickDisconnect(_) => "kick_disconnect",
                    Params::<'a>::ParamsEntityStatus(_) => "entity_status",
                    Params::<'a>::Explosion(_) => "explosion",
                    Params::<'a>::UnloadChunk(_) => "unload_chunk",
                    Params::<'a>::GameStateChange(_) => "game_state_change",
                    Params::<'a>::OpenHorseWindow(_) => "open_horse_window",
                    Params::<'a>::KeepAlive(_) => "keep_alive",
                    Params::<'a>::MapChunk(_) => "map_chunk",
                    Params::<'a>::WorldEvent(_) => "world_event",
                    Params::<'a>::WorldParticles(_) => "world_particles",
                    Params::<'a>::UpdateLight(_) => "update_light",
                    Params::<'a>::Login(_) => "login",
                    Params::<'a>::Map(_) => "map",
                    Params::<'a>::TradeList(_) => "trade_list",
                    Params::<'a>::RelEntityMove(_) => "rel_entity_move",
                    Params::<'a>::EntityMoveLook(_) => "entity_move_look",
                    Params::<'a>::EntityLook(_) => "entity_look",
                    Params::<'a>::VehicleMove(_) => "vehicle_move",
                    Params::<'a>::OpenBook(_) => "open_book",
                    Params::<'a>::OpenSignEntity(_) => "open_sign_entity",
                    Params::<'a>::CraftRecipeResponse(_) => "craft_recipe_response",
                    Params::<'a>::Abilities(_) => "abilities",
                    Params::<'a>::EndCombatEvent(_) => "end_combat_event",
                    Params::<'a>::EnterCombatEvent(_) => "enter_combat_event",
                    Params::<'a>::DeathCombatEvent(_) => "death_combat_event",
                    Params::<'a>::PlayerInfo(_) => "player_info",
                    Params::<'a>::ParamsPosition(_) => "position",
                    Params::<'a>::UnlockRecipes(_) => "unlock_recipes",
                    Params::<'a>::EntityDestroy(_) => "entity_destroy",
                    Params::<'a>::RemoveEntityEffect(_) => "remove_entity_effect",
                    Params::<'a>::ResourcePackSend(_) => "resource_pack_send",
                    Params::<'a>::Respawn(_) => "respawn",
                    Params::<'a>::EntityUpdateAttributes(_) => "entity_update_attributes",
                    Params::<'a>::Camera(_) => "camera",
                    Params::<'a>::HeldItemSlot(_) => "held_item_slot",
                    Params::<'a>::UpdateViewPosition(_) => "update_view_position",
                    Params::<'a>::UpdateViewDistance(_) => "update_view_distance",
                    Params::<'a>::ScoreboardDisplayObjective(_) => "scoreboard_display_objective",
                    Params::<'a>::ParamsEntityMetadata(_) => "entity_metadata",
                    Params::<'a>::AttachEntity(_) => "attach_entity",
                    Params::<'a>::EntityVelocity(_) => "entity_velocity",
                    Params::<'a>::EntityEquipment(_) => "entity_equipment",
                    Params::<'a>::ParamsExperience(_) => "experience",
                    Params::<'a>::UpdateHealth(_) => "update_health",
                    Params::<'a>::ScoreboardObjective(_) => "scoreboard_objective",
                    Params::<'a>::SetPassengers(_) => "set_passengers",
                    Params::<'a>::Teams(_) => "teams",
                    Params::<'a>::ScoreboardScore(_) => "scoreboard_score",
                    Params::<'a>::ParamsSimulationDistance(_) => "simulation_distance",
                    Params::<'a>::SpawnPosition(_) => "spawn_position",
                    Params::<'a>::UpdateTime(_) => "update_time",
                    Params::<'a>::EntitySoundEffect(_) => "entity_sound_effect",
                    Params::<'a>::StopSound(_) => "stop_sound",
                    Params::<'a>::SoundEffect(_) => "sound_effect",
                    Params::<'a>::PlayerlistHeader(_) => "playerlist_header",
                    Params::<'a>::Collect(_) => "collect",
                    Params::<'a>::EntityTeleport(_) => "entity_teleport",
                    Params::<'a>::EntityHeadRotation(_) => "entity_head_rotation",
                    Params::<'a>::EntityEffect(_) => "entity_effect",
                    Params::<'a>::SelectAdvancementTab(_) => "select_advancement_tab",
                    Params::<'a>::DeclareRecipes(_) => "declare_recipes",
                    Params::<'a>::ParamsTags(_) => "tags",
                    Params::<'a>::AcknowledgePlayerDigging(_) => "acknowledge_player_digging",
                    Params::<'a>::SculkVibrationSignal(_) => "sculk_vibration_signal",
                    Params::<'a>::ClearTitles(_) => "clear_titles",
                    Params::<'a>::InitializeWorldBorder(_) => "initialize_world_border",
                    Params::<'a>::ActionBar(_) => "action_bar",
                    Params::<'a>::WorldBorderCenter(_) => "world_border_center",
                    Params::<'a>::WorldBorderLerpSize(_) => "world_border_lerp_size",
                    Params::<'a>::WorldBorderSize(_) => "world_border_size",
                    Params::<'a>::WorldBorderWarningDelay(_) => "world_border_warning_delay",
                    Params::<'a>::WorldBorderWarningReach(_) => "world_border_warning_reach",
                    Params::<'a>::ParamsPing(_) => "ping",
                    Params::<'a>::SetTitleSubtitle(_) => "set_title_subtitle",
                    Params::<'a>::SetTitleText(_) => "set_title_text",
                    Params::<'a>::SetTitleTime(_) => "set_title_time",
                    _ => "",
                }
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = match &self.params {
                    Params::<'a>::SpawnEntity(val) => {
                        let w = PacketSpawnEntity::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SpawnEntityExperienceOrb(val) => {
                        let w = PacketSpawnEntityExperienceOrb::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SpawnEntityLiving(val) => {
                        let w = PacketSpawnEntityLiving::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SpawnEntityPainting(val) => {
                        let w = PacketSpawnEntityPainting::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::NamedEntitySpawn(val) => {
                        let w = PacketNamedEntitySpawn::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsAnimation(val) => {
                        let w = PacketAnimation::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Statistics(val) => {
                        let w = PacketStatistics::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Advancements(val) => {
                        let w = PacketAdvancements::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::BlockBreakAnimation(val) => {
                        let w = PacketBlockBreakAnimation::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::TileEntityData(val) => {
                        let w = PacketTileEntityData::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::BlockAction(val) => {
                        let w = PacketBlockAction::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::BlockChange(val) => {
                        let w = PacketBlockChange::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::BossBar(val) => {
                        let w = PacketBossBar::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsDifficulty(val) => {
                        let w = PacketDifficulty::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::TabComplete(val) => {
                        let w = PacketTabComplete::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::DeclareCommands(val) => {
                        let w = PacketDeclareCommands::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::FacePlayer(val) => {
                        let w = PacketFacePlayer::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::NbtQueryResponse(val) => {
                        let w = PacketNbtQueryResponse::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Chat(val) => {
                        let w = PacketChat::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::MultiBlockChange(val) => {
                        let w = PacketMultiBlockChange::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::CloseWindow(val) => {
                        let w = PacketCloseWindow::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::OpenWindow(val) => {
                        let w = PacketOpenWindow::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WindowItems(val) => {
                        let w = PacketWindowItems::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::CraftProgressBar(val) => {
                        let w = PacketCraftProgressBar::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetSlot(val) => {
                        let w = PacketSetSlot::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetCooldown(val) => {
                        let w = PacketSetCooldown::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::CustomPayload(val) => {
                        let w = PacketCustomPayload::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::NamedSoundEffect(val) => {
                        let w = PacketNamedSoundEffect::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::KickDisconnect(val) => {
                        let w = PacketKickDisconnect::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsEntityStatus(val) => {
                        let w = PacketEntityStatus::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Explosion(val) => {
                        let w = PacketExplosion::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UnloadChunk(val) => {
                        let w = PacketUnloadChunk::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::GameStateChange(val) => {
                        let w = PacketGameStateChange::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::OpenHorseWindow(val) => {
                        let w = PacketOpenHorseWindow::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::KeepAlive(val) => {
                        let w = PacketKeepAlive::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::MapChunk(val) => {
                        let w = PacketMapChunk::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WorldEvent(val) => {
                        let w = PacketWorldEvent::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WorldParticles(val) => {
                        let w = PacketWorldParticles::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateLight(val) => {
                        let w = PacketUpdateLight::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Login(val) => {
                        let w = PacketLogin::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Map(val) => {
                        let w = PacketMap::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::TradeList(val) => {
                        let w = PacketTradeList::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::RelEntityMove(val) => {
                        let w = PacketRelEntityMove::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityMoveLook(val) => {
                        let w = PacketEntityMoveLook::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityLook(val) => {
                        let w = PacketEntityLook::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::VehicleMove(val) => {
                        let w = PacketVehicleMove::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::OpenBook(val) => {
                        let w = PacketOpenBook::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::OpenSignEntity(val) => {
                        let w = PacketOpenSignEntity::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::CraftRecipeResponse(val) => {
                        let w = PacketCraftRecipeResponse::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Abilities(val) => {
                        let w = PacketAbilities::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EndCombatEvent(val) => {
                        let w = PacketEndCombatEvent::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EnterCombatEvent(val) => {
                        let w = PacketEnterCombatEvent::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::DeathCombatEvent(val) => {
                        let w = PacketDeathCombatEvent::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::PlayerInfo(val) => {
                        let w = PacketPlayerInfo::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsPosition(val) => {
                        let w = PacketPosition::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UnlockRecipes(val) => {
                        let w = PacketUnlockRecipes::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityDestroy(val) => {
                        let w = PacketEntityDestroy::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::RemoveEntityEffect(val) => {
                        let w = PacketRemoveEntityEffect::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ResourcePackSend(val) => {
                        let w = PacketResourcePackSend::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Respawn(val) => {
                        let w = PacketRespawn::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityUpdateAttributes(val) => {
                        let w = PacketEntityUpdateAttributes::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Camera(val) => {
                        let w = PacketCamera::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::HeldItemSlot(val) => {
                        let w = PacketHeldItemSlot::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateViewPosition(val) => {
                        let w = PacketUpdateViewPosition::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateViewDistance(val) => {
                        let w = PacketUpdateViewDistance::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ScoreboardDisplayObjective(val) => {
                        let w = PacketScoreboardDisplayObjective::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsEntityMetadata(val) => {
                        let w = PacketEntityMetadata::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::AttachEntity(val) => {
                        let w = PacketAttachEntity::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityVelocity(val) => {
                        let w = PacketEntityVelocity::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityEquipment(val) => {
                        let w = PacketEntityEquipment::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsExperience(val) => {
                        let w = PacketExperience::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateHealth(val) => {
                        let w = PacketUpdateHealth::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ScoreboardObjective(val) => {
                        let w = PacketScoreboardObjective::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetPassengers(val) => {
                        let w = PacketSetPassengers::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Teams(val) => {
                        let w = PacketTeams::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ScoreboardScore(val) => {
                        let w = PacketScoreboardScore::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsSimulationDistance(val) => {
                        let w = PacketSimulationDistance::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SpawnPosition(val) => {
                        let w = PacketSpawnPosition::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateTime(val) => {
                        let w = PacketUpdateTime::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntitySoundEffect(val) => {
                        let w = PacketEntitySoundEffect::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::StopSound(val) => {
                        let w = PacketStopSound::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SoundEffect(val) => {
                        let w = PacketSoundEffect::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::PlayerlistHeader(val) => {
                        let w = PacketPlayerlistHeader::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Collect(val) => {
                        let w = PacketCollect::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityTeleport(val) => {
                        let w = PacketEntityTeleport::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityHeadRotation(val) => {
                        let w = PacketEntityHeadRotation::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityEffect(val) => {
                        let w = PacketEntityEffect::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SelectAdvancementTab(val) => {
                        let w = PacketSelectAdvancementTab::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::DeclareRecipes(val) => {
                        let w = PacketDeclareRecipes::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsTags(val) => {
                        let w = PacketTags::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::AcknowledgePlayerDigging(val) => {
                        let w = PacketAcknowledgePlayerDigging::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SculkVibrationSignal(val) => {
                        let w = PacketSculkVibrationSignal::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ClearTitles(val) => {
                        let w = PacketClearTitles::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::InitializeWorldBorder(val) => {
                        let w = PacketInitializeWorldBorder::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ActionBar(val) => {
                        let w = PacketActionBar::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WorldBorderCenter(val) => {
                        let w = PacketWorldBorderCenter::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WorldBorderLerpSize(val) => {
                        let w = PacketWorldBorderLerpSize::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WorldBorderSize(val) => {
                        let w = PacketWorldBorderSize::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WorldBorderWarningDelay(val) => {
                        let w = PacketWorldBorderWarningDelay::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WorldBorderWarningReach(val) => {
                        let w = PacketWorldBorderWarningReach::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsPing(val) => {
                        let w = PacketPing::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetTitleSubtitle(val) => {
                        let w = PacketSetTitleSubtitle::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetTitleText(val) => {
                        let w = PacketSetTitleText::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetTitleTime(val) => {
                        let w = PacketSetTitleTime::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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

                            _ => {
                                return Err(nom::Err::Error(nom::error::Error::new(
                                    input,
                                    nom::error::ErrorKind::Verify,
                                )))
                            }
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "spawn_entity" => nom::combinator::map(
                            PacketSpawnEntity::deserialize,
                            Params::<'a>::SpawnEntity,
                        )(input),
                        "spawn_entity_experience_orb" => nom::combinator::map(
                            PacketSpawnEntityExperienceOrb::deserialize,
                            Params::<'a>::SpawnEntityExperienceOrb,
                        )(input),
                        "spawn_entity_living" => nom::combinator::map(
                            PacketSpawnEntityLiving::deserialize,
                            Params::<'a>::SpawnEntityLiving,
                        )(input),
                        "spawn_entity_painting" => nom::combinator::map(
                            PacketSpawnEntityPainting::deserialize,
                            Params::<'a>::SpawnEntityPainting,
                        )(input),
                        "named_entity_spawn" => nom::combinator::map(
                            PacketNamedEntitySpawn::deserialize,
                            Params::<'a>::NamedEntitySpawn,
                        )(input),
                        "animation" => nom::combinator::map(
                            PacketAnimation::deserialize,
                            Params::<'a>::ParamsAnimation,
                        )(input),
                        "statistics" => nom::combinator::map(
                            PacketStatistics::deserialize,
                            Params::<'a>::Statistics,
                        )(input),
                        "advancements" => nom::combinator::map(
                            PacketAdvancements::<'a>::deserialize,
                            Params::<'a>::Advancements,
                        )(input),
                        "block_break_animation" => nom::combinator::map(
                            PacketBlockBreakAnimation::deserialize,
                            Params::<'a>::BlockBreakAnimation,
                        )(input),
                        "tile_entity_data" => nom::combinator::map(
                            PacketTileEntityData::deserialize,
                            Params::<'a>::TileEntityData,
                        )(input),
                        "block_action" => nom::combinator::map(
                            PacketBlockAction::deserialize,
                            Params::<'a>::BlockAction,
                        )(input),
                        "block_change" => nom::combinator::map(
                            PacketBlockChange::deserialize,
                            Params::<'a>::BlockChange,
                        )(input),
                        "boss_bar" => nom::combinator::map(
                            PacketBossBar::<'a>::deserialize,
                            Params::<'a>::BossBar,
                        )(input),
                        "difficulty" => nom::combinator::map(
                            PacketDifficulty::deserialize,
                            Params::<'a>::ParamsDifficulty,
                        )(input),
                        "tab_complete" => nom::combinator::map(
                            PacketTabComplete::<'a>::deserialize,
                            Params::<'a>::TabComplete,
                        )(input),
                        "declare_commands" => nom::combinator::map(
                            PacketDeclareCommands::<'a>::deserialize,
                            Params::<'a>::DeclareCommands,
                        )(input),
                        "face_player" => nom::combinator::map(
                            PacketFacePlayer::<'a>::deserialize,
                            Params::<'a>::FacePlayer,
                        )(input),
                        "nbt_query_response" => nom::combinator::map(
                            PacketNbtQueryResponse::deserialize,
                            Params::<'a>::NbtQueryResponse,
                        )(input),
                        "chat" => nom::combinator::map(
                            PacketChat::<'a>::deserialize,
                            Params::<'a>::Chat,
                        )(input),
                        "multi_block_change" => nom::combinator::map(
                            PacketMultiBlockChange::deserialize,
                            Params::<'a>::MultiBlockChange,
                        )(input),
                        "close_window" => nom::combinator::map(
                            PacketCloseWindow::deserialize,
                            Params::<'a>::CloseWindow,
                        )(input),
                        "open_window" => nom::combinator::map(
                            PacketOpenWindow::<'a>::deserialize,
                            Params::<'a>::OpenWindow,
                        )(input),
                        "window_items" => nom::combinator::map(
                            PacketWindowItems::deserialize,
                            Params::<'a>::WindowItems,
                        )(input),
                        "craft_progress_bar" => nom::combinator::map(
                            PacketCraftProgressBar::deserialize,
                            Params::<'a>::CraftProgressBar,
                        )(input),
                        "set_slot" => nom::combinator::map(
                            PacketSetSlot::deserialize,
                            Params::<'a>::SetSlot,
                        )(input),
                        "set_cooldown" => nom::combinator::map(
                            PacketSetCooldown::deserialize,
                            Params::<'a>::SetCooldown,
                        )(input),
                        "custom_payload" => nom::combinator::map(
                            PacketCustomPayload::<'a>::deserialize,
                            Params::<'a>::CustomPayload,
                        )(input),
                        "named_sound_effect" => nom::combinator::map(
                            PacketNamedSoundEffect::<'a>::deserialize,
                            Params::<'a>::NamedSoundEffect,
                        )(input),
                        "kick_disconnect" => nom::combinator::map(
                            PacketKickDisconnect::<'a>::deserialize,
                            Params::<'a>::KickDisconnect,
                        )(input),
                        "entity_status" => nom::combinator::map(
                            PacketEntityStatus::deserialize,
                            Params::<'a>::ParamsEntityStatus,
                        )(input),
                        "explosion" => nom::combinator::map(
                            PacketExplosion::deserialize,
                            Params::<'a>::Explosion,
                        )(input),
                        "unload_chunk" => nom::combinator::map(
                            PacketUnloadChunk::deserialize,
                            Params::<'a>::UnloadChunk,
                        )(input),
                        "game_state_change" => nom::combinator::map(
                            PacketGameStateChange::deserialize,
                            Params::<'a>::GameStateChange,
                        )(input),
                        "open_horse_window" => nom::combinator::map(
                            PacketOpenHorseWindow::deserialize,
                            Params::<'a>::OpenHorseWindow,
                        )(input),
                        "keep_alive" => nom::combinator::map(
                            PacketKeepAlive::deserialize,
                            Params::<'a>::KeepAlive,
                        )(input),
                        "map_chunk" => nom::combinator::map(
                            PacketMapChunk::<'a>::deserialize,
                            Params::<'a>::MapChunk,
                        )(input),
                        "world_event" => nom::combinator::map(
                            PacketWorldEvent::deserialize,
                            Params::<'a>::WorldEvent,
                        )(input),
                        "world_particles" => nom::combinator::map(
                            PacketWorldParticles::<'a>::deserialize,
                            Params::<'a>::WorldParticles,
                        )(input),
                        "update_light" => nom::combinator::map(
                            PacketUpdateLight::deserialize,
                            Params::<'a>::UpdateLight,
                        )(input),
                        "login" => nom::combinator::map(
                            PacketLogin::<'a>::deserialize,
                            Params::<'a>::Login,
                        )(input),
                        "map" => nom::combinator::map(
                            PacketMap::<'a>::deserialize,
                            Params::<'a>::Map,
                        )(input),
                        "trade_list" => nom::combinator::map(
                            PacketTradeList::deserialize,
                            Params::<'a>::TradeList,
                        )(input),
                        "rel_entity_move" => nom::combinator::map(
                            PacketRelEntityMove::deserialize,
                            Params::<'a>::RelEntityMove,
                        )(input),
                        "entity_move_look" => nom::combinator::map(
                            PacketEntityMoveLook::deserialize,
                            Params::<'a>::EntityMoveLook,
                        )(input),
                        "entity_look" => nom::combinator::map(
                            PacketEntityLook::deserialize,
                            Params::<'a>::EntityLook,
                        )(input),
                        "vehicle_move" => nom::combinator::map(
                            PacketVehicleMove::deserialize,
                            Params::<'a>::VehicleMove,
                        )(input),
                        "open_book" => nom::combinator::map(
                            PacketOpenBook::deserialize,
                            Params::<'a>::OpenBook,
                        )(input),
                        "open_sign_entity" => nom::combinator::map(
                            PacketOpenSignEntity::deserialize,
                            Params::<'a>::OpenSignEntity,
                        )(input),
                        "craft_recipe_response" => nom::combinator::map(
                            PacketCraftRecipeResponse::<'a>::deserialize,
                            Params::<'a>::CraftRecipeResponse,
                        )(input),
                        "abilities" => nom::combinator::map(
                            PacketAbilities::deserialize,
                            Params::<'a>::Abilities,
                        )(input),
                        "end_combat_event" => nom::combinator::map(
                            PacketEndCombatEvent::deserialize,
                            Params::<'a>::EndCombatEvent,
                        )(input),
                        "enter_combat_event" => nom::combinator::map(
                            PacketEnterCombatEvent::deserialize,
                            Params::<'a>::EnterCombatEvent,
                        )(input),
                        "death_combat_event" => nom::combinator::map(
                            PacketDeathCombatEvent::<'a>::deserialize,
                            Params::<'a>::DeathCombatEvent,
                        )(input),
                        "player_info" => nom::combinator::map(
                            PacketPlayerInfo::<'a>::deserialize,
                            Params::<'a>::PlayerInfo,
                        )(input),
                        "position" => nom::combinator::map(
                            PacketPosition::deserialize,
                            Params::<'a>::ParamsPosition,
                        )(input),
                        "unlock_recipes" => nom::combinator::map(
                            PacketUnlockRecipes::<'a>::deserialize,
                            Params::<'a>::UnlockRecipes,
                        )(input),
                        "entity_destroy" => nom::combinator::map(
                            PacketEntityDestroy::deserialize,
                            Params::<'a>::EntityDestroy,
                        )(input),
                        "remove_entity_effect" => nom::combinator::map(
                            PacketRemoveEntityEffect::deserialize,
                            Params::<'a>::RemoveEntityEffect,
                        )(input),
                        "resource_pack_send" => nom::combinator::map(
                            PacketResourcePackSend::<'a>::deserialize,
                            Params::<'a>::ResourcePackSend,
                        )(input),
                        "respawn" => nom::combinator::map(
                            PacketRespawn::<'a>::deserialize,
                            Params::<'a>::Respawn,
                        )(input),
                        "entity_update_attributes" => nom::combinator::map(
                            PacketEntityUpdateAttributes::<'a>::deserialize,
                            Params::<'a>::EntityUpdateAttributes,
                        )(input),
                        "camera" => nom::combinator::map(
                            PacketCamera::deserialize,
                            Params::<'a>::Camera,
                        )(input),
                        "held_item_slot" => nom::combinator::map(
                            PacketHeldItemSlot::deserialize,
                            Params::<'a>::HeldItemSlot,
                        )(input),
                        "update_view_position" => nom::combinator::map(
                            PacketUpdateViewPosition::deserialize,
                            Params::<'a>::UpdateViewPosition,
                        )(input),
                        "update_view_distance" => nom::combinator::map(
                            PacketUpdateViewDistance::deserialize,
                            Params::<'a>::UpdateViewDistance,
                        )(input),
                        "scoreboard_display_objective" => nom::combinator::map(
                            PacketScoreboardDisplayObjective::<'a>::deserialize,
                            Params::<'a>::ScoreboardDisplayObjective,
                        )(input),
                        "entity_metadata" => nom::combinator::map(
                            PacketEntityMetadata::<'a>::deserialize,
                            Params::<'a>::ParamsEntityMetadata,
                        )(input),
                        "attach_entity" => nom::combinator::map(
                            PacketAttachEntity::deserialize,
                            Params::<'a>::AttachEntity,
                        )(input),
                        "entity_velocity" => nom::combinator::map(
                            PacketEntityVelocity::deserialize,
                            Params::<'a>::EntityVelocity,
                        )(input),
                        "entity_equipment" => nom::combinator::map(
                            PacketEntityEquipment::deserialize,
                            Params::<'a>::EntityEquipment,
                        )(input),
                        "experience" => nom::combinator::map(
                            PacketExperience::deserialize,
                            Params::<'a>::ParamsExperience,
                        )(input),
                        "update_health" => nom::combinator::map(
                            PacketUpdateHealth::deserialize,
                            Params::<'a>::UpdateHealth,
                        )(input),
                        "scoreboard_objective" => nom::combinator::map(
                            PacketScoreboardObjective::<'a>::deserialize,
                            Params::<'a>::ScoreboardObjective,
                        )(input),
                        "set_passengers" => nom::combinator::map(
                            PacketSetPassengers::deserialize,
                            Params::<'a>::SetPassengers,
                        )(input),
                        "teams" => nom::combinator::map(
                            PacketTeams::<'a>::deserialize,
                            Params::<'a>::Teams,
                        )(input),
                        "scoreboard_score" => nom::combinator::map(
                            PacketScoreboardScore::<'a>::deserialize,
                            Params::<'a>::ScoreboardScore,
                        )(input),
                        "simulation_distance" => nom::combinator::map(
                            PacketSimulationDistance::deserialize,
                            Params::<'a>::ParamsSimulationDistance,
                        )(input),
                        "spawn_position" => nom::combinator::map(
                            PacketSpawnPosition::deserialize,
                            Params::<'a>::SpawnPosition,
                        )(input),
                        "update_time" => nom::combinator::map(
                            PacketUpdateTime::deserialize,
                            Params::<'a>::UpdateTime,
                        )(input),
                        "entity_sound_effect" => nom::combinator::map(
                            PacketEntitySoundEffect::deserialize,
                            Params::<'a>::EntitySoundEffect,
                        )(input),
                        "stop_sound" => nom::combinator::map(
                            PacketStopSound::<'a>::deserialize,
                            Params::<'a>::StopSound,
                        )(input),
                        "sound_effect" => nom::combinator::map(
                            PacketSoundEffect::deserialize,
                            Params::<'a>::SoundEffect,
                        )(input),
                        "playerlist_header" => nom::combinator::map(
                            PacketPlayerlistHeader::<'a>::deserialize,
                            Params::<'a>::PlayerlistHeader,
                        )(input),
                        "collect" => nom::combinator::map(
                            PacketCollect::deserialize,
                            Params::<'a>::Collect,
                        )(input),
                        "entity_teleport" => nom::combinator::map(
                            PacketEntityTeleport::deserialize,
                            Params::<'a>::EntityTeleport,
                        )(input),
                        "entity_head_rotation" => nom::combinator::map(
                            PacketEntityHeadRotation::deserialize,
                            Params::<'a>::EntityHeadRotation,
                        )(input),
                        "entity_effect" => nom::combinator::map(
                            PacketEntityEffect::deserialize,
                            Params::<'a>::EntityEffect,
                        )(input),
                        "select_advancement_tab" => nom::combinator::map(
                            PacketSelectAdvancementTab::<'a>::deserialize,
                            Params::<'a>::SelectAdvancementTab,
                        )(input),
                        "declare_recipes" => nom::combinator::map(
                            PacketDeclareRecipes::<'a>::deserialize,
                            Params::<'a>::DeclareRecipes,
                        )(input),
                        "tags" => nom::combinator::map(
                            PacketTags::<'a>::deserialize,
                            Params::<'a>::ParamsTags,
                        )(input),
                        "acknowledge_player_digging" => nom::combinator::map(
                            PacketAcknowledgePlayerDigging::deserialize,
                            Params::<'a>::AcknowledgePlayerDigging,
                        )(input),
                        "sculk_vibration_signal" => nom::combinator::map(
                            PacketSculkVibrationSignal::<'a>::deserialize,
                            Params::<'a>::SculkVibrationSignal,
                        )(input),
                        "clear_titles" => nom::combinator::map(
                            PacketClearTitles::deserialize,
                            Params::<'a>::ClearTitles,
                        )(input),
                        "initialize_world_border" => nom::combinator::map(
                            PacketInitializeWorldBorder::deserialize,
                            Params::<'a>::InitializeWorldBorder,
                        )(input),
                        "action_bar" => nom::combinator::map(
                            PacketActionBar::<'a>::deserialize,
                            Params::<'a>::ActionBar,
                        )(input),
                        "world_border_center" => nom::combinator::map(
                            PacketWorldBorderCenter::deserialize,
                            Params::<'a>::WorldBorderCenter,
                        )(input),
                        "world_border_lerp_size" => nom::combinator::map(
                            PacketWorldBorderLerpSize::deserialize,
                            Params::<'a>::WorldBorderLerpSize,
                        )(input),
                        "world_border_size" => nom::combinator::map(
                            PacketWorldBorderSize::deserialize,
                            Params::<'a>::WorldBorderSize,
                        )(input),
                        "world_border_warning_delay" => nom::combinator::map(
                            PacketWorldBorderWarningDelay::deserialize,
                            Params::<'a>::WorldBorderWarningDelay,
                        )(input),
                        "world_border_warning_reach" => nom::combinator::map(
                            PacketWorldBorderWarningReach::deserialize,
                            Params::<'a>::WorldBorderWarningReach,
                        )(input),
                        "ping" => nom::combinator::map(
                            PacketPing::deserialize,
                            Params::<'a>::ParamsPing,
                        )(input),
                        "set_title_subtitle" => nom::combinator::map(
                            PacketSetTitleSubtitle::<'a>::deserialize,
                            Params::<'a>::SetTitleSubtitle,
                        )(input),
                        "set_title_text" => nom::combinator::map(
                            PacketSetTitleText::<'a>::deserialize,
                            Params::<'a>::SetTitleText,
                        )(input),
                        "set_title_time" => nom::combinator::map(
                            PacketSetTitleTime::deserialize,
                            Params::<'a>::SetTitleTime,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, Params::<'a>::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketTeleportConfirm {
            teleport_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketTeleportConfirm {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.teleport_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(teleport_id,)| PacketTeleportConfirm { teleport_id },
                ))(input)
            }
        }

        pub struct PacketQueryBlockNbt {
            transaction_id: VarInt,
            location: Position,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketQueryBlockNbt {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = Position::serialize(&self.location, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, Position::deserialize)),
                    |(transaction_id, location)| PacketQueryBlockNbt {
                        transaction_id,
                        location,
                    },
                ))(input)
            }
        }

        pub struct PacketSetDifficulty {
            new_difficulty: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetDifficulty {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.new_difficulty, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((u8::deserialize,)),
                    |(new_difficulty,)| PacketSetDifficulty { new_difficulty },
                ))(input)
            }
        }

        pub struct PacketEditBook<'a> {
            hand: VarInt,
            pages: VarStringArray<'a>,
            title: Option<VarString<'a>>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketEditBook<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;
                let w = PrefixedArray::<VarString<'a>, VarInt>::serialize(&self.pages, w)?;
                let w = Option::<VarString<'a>>::serialize(&self.title, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedArray::<VarString<'a>, VarInt>::deserialize,
                        Option::<VarString<'a>>::deserialize,
                    )),
                    |(hand, pages, title)| PacketEditBook { hand, pages, title },
                ))(input)
            }
        }

        pub struct PacketQueryEntityNbt {
            transaction_id: VarInt,
            entity_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketQueryEntityNbt {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = VarInt::serialize(&self.entity_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, VarInt::deserialize)),
                    |(transaction_id, entity_id)| PacketQueryEntityNbt {
                        transaction_id,
                        entity_id,
                    },
                ))(input)
            }
        }

        pub struct PacketPickItem {
            slot: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPickItem {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.slot, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((VarInt::deserialize,)), |(slot,)| {
                    PacketPickItem { slot }
                }))(input)
            }
        }

        pub struct PacketNameItem<'a> {
            name: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketNameItem<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(name,)| PacketNameItem { name },
                ))(input)
            }
        }

        pub struct PacketSelectTrade {
            slot: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSelectTrade {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.slot, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((VarInt::deserialize,)), |(slot,)| {
                    PacketSelectTrade { slot }
                }))(input)
            }
        }

        pub struct PacketSetBeaconEffect {
            primary_effect: VarInt,
            secondary_effect: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetBeaconEffect {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.primary_effect, w)?;
                let w = VarInt::serialize(&self.secondary_effect, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize, VarInt::deserialize)),
                    |(primary_effect, secondary_effect)| PacketSetBeaconEffect {
                        primary_effect,
                        secondary_effect,
                    },
                ))(input)
            }
        }

        pub struct PacketUpdateCommandBlock<'a> {
            location: Position,
            command: VarString<'a>,
            mode: VarInt,
            flags: u8,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUpdateCommandBlock<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.command, w)?;
                let w = VarInt::serialize(&self.mode, w)?;
                let w = u8::serialize(&self.flags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Position::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        VarInt::deserialize,
                        u8::deserialize,
                    )),
                    |(location, command, mode, flags)| PacketUpdateCommandBlock {
                        location,
                        command,
                        mode,
                        flags,
                    },
                ))(input)
            }
        }

        pub struct PacketUpdateCommandBlockMinecart<'a> {
            entity_id: VarInt,
            command: VarString<'a>,
            track_output: bool,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketUpdateCommandBlockMinecart<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.command, w)?;
                let w = bool::serialize(&self.track_output, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        bool::deserialize,
                    )),
                    |(entity_id, command, track_output)| PacketUpdateCommandBlockMinecart {
                        entity_id,
                        command,
                        track_output,
                    },
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
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
                    |(
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
                    )| PacketUpdateStructureBlock {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.transaction_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(transaction_id, text)| PacketTabComplete {
                        transaction_id,
                        text,
                    },
                ))(input)
            }
        }

        pub struct PacketChat<'a> {
            message: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketChat<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.message, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(message,)| PacketChat { message },
                ))(input)
            }
        }

        pub struct PacketClientCommand {
            action_id: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketClientCommand {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((VarInt::deserialize,)),
                    |(action_id,)| PacketClientCommand { action_id },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        i8::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        u8::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(
                        locale,
                        view_distance,
                        chat_flags,
                        chat_colors,
                        skin_parts,
                        main_hand,
                        enable_text_filtering,
                        enable_server_listing,
                    )| PacketSettings {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = i8::serialize(&self.enchantment, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i8::deserialize, i8::deserialize)),
                    |(window_id, enchantment)| PacketEnchantItem {
                        window_id,
                        enchantment,
                    },
                ))(input)
            }
        }

        pub struct ChangedSlot {
            location: i16,
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for ChangedSlot {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i16::serialize(&self.location, w)?;
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i16::deserialize, Slot::deserialize)),
                    |(location, item)| ChangedSlot { location, item },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;
                let w = VarInt::serialize(&self.state_id, w)?;
                let w = i16::serialize(&self.slot, w)?;
                let w = i8::serialize(&self.mouse_button, w)?;
                let w = VarInt::serialize(&self.mode, w)?;
                let w = PrefixedArray::<ChangedSlot, VarInt>::serialize(&self.changed_slots, w)?;
                let w = Slot::serialize(&self.cursor_item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        u8::deserialize,
                        VarInt::deserialize,
                        i16::deserialize,
                        i8::deserialize,
                        VarInt::deserialize,
                        PrefixedArray::<ChangedSlot, VarInt>::deserialize,
                        Slot::deserialize,
                    )),
                    |(
                        window_id,
                        state_id,
                        slot,
                        mouse_button,
                        mode,
                        changed_slots,
                        cursor_item,
                    )| PacketWindowClick {
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = u8::serialize(&self.window_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((u8::deserialize,)), |(window_id,)| {
                    PacketCloseWindow { window_id }
                }))(input)
            }
        }

        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCustomPayload<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.channel, w)?;
                let w = RestBuffer::<'a>::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        PrefixedString::<'a, VarInt>::deserialize,
                        RestBuffer::<'a>::deserialize,
                    )),
                    |(channel, data)| PacketCustomPayload { channel, data },
                ))(input)
            }
        }

        pub enum X {
            X2(f32),
            Default(Void),
        }

        impl X {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    X::X2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum UseEntityY {
            UseEntityY2(f32),
            Default(Void),
        }

        impl UseEntityY {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    UseEntityY::UseEntityY2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum Z {
            Z2(f32),
            Default(Void),
        }

        impl Z {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Z::Z2(_) => "2",
                    _ => "",
                }
            }
        }
        pub enum UseEntityHand {
            UseEntityHand0(VarInt),
            UseEntityHand2(VarInt),
            Default(Void),
        }

        impl UseEntityHand {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    UseEntityHand::UseEntityHand0(_) => "0",
                    UseEntityHand::UseEntityHand2(_) => "2",
                    _ => "",
                }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.target, w)?;
                let w = VarInt::serialize(&self.mouse, w)?;

                let w = match &self.x {
                    X::X2(val) => {
                        let w = f32::serialize(&val, w)?;
                        w
                    }
                    X::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.y {
                    UseEntityY::UseEntityY2(val) => {
                        let w = f32::serialize(&val, w)?;
                        w
                    }
                    UseEntityY::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.z {
                    Z::Z2(val) => {
                        let w = f32::serialize(&val, w)?;
                        w
                    }
                    Z::Default(val) => Void::serialize(val, w)?,
                };

                let w = match &self.hand {
                    UseEntityHand::UseEntityHand0(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    UseEntityHand::UseEntityHand2(val) => {
                        let w = VarInt::serialize(&val, w)?;
                        w
                    }
                    UseEntityHand::Default(val) => Void::serialize(val, w)?,
                };

                let w = bool::serialize(&self.sneaking, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_target) = (VarInt::deserialize)(input)?;
                    let (input, self_mouse) = (VarInt::deserialize)(input)?;
                    let (input, self_x) = (|input| match &format!("{}", self_mouse)[..] {
                        "2" => nom::combinator::map(f32::deserialize, X::X2)(input),
                        _ => nom::combinator::map(Void::deserialize, X::Default)(input),
                    })(input)?;
                    let (input, self_y) = (|input| match &format!("{}", self_mouse)[..] {
                        "2" => {
                            nom::combinator::map(f32::deserialize, UseEntityY::UseEntityY2)(input)
                        }
                        _ => nom::combinator::map(Void::deserialize, UseEntityY::Default)(input),
                    })(input)?;
                    let (input, self_z) = (|input| match &format!("{}", self_mouse)[..] {
                        "2" => nom::combinator::map(f32::deserialize, Z::Z2)(input),
                        _ => nom::combinator::map(Void::deserialize, Z::Default)(input),
                    })(input)?;
                    let (input, self_hand) = (|input| match &format!("{}", self_mouse)[..] {
                        "0" => nom::combinator::map(
                            VarInt::deserialize,
                            UseEntityHand::UseEntityHand0,
                        )(input),
                        "2" => nom::combinator::map(
                            VarInt::deserialize,
                            UseEntityHand::UseEntityHand2,
                        )(input),
                        _ => nom::combinator::map(Void::deserialize, UseEntityHand::Default)(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = VarInt::serialize(&self.levels, w)?;
                let w = bool::serialize(&self.keep_jigsaws, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Position::deserialize,
                        VarInt::deserialize,
                        bool::deserialize,
                    )),
                    |(location, levels, keep_jigsaws)| PacketGenerateStructure {
                        location,
                        levels,
                        keep_jigsaws,
                    },
                ))(input)
            }
        }

        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketKeepAlive {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i64::serialize(&self.keep_alive_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i64::deserialize,)),
                    |(keep_alive_id,)| PacketKeepAlive { keep_alive_id },
                ))(input)
            }
        }

        pub struct PacketLockDifficulty {
            locked: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketLockDifficulty {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.locked, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((bool::deserialize,)), |(locked,)| {
                    PacketLockDifficulty { locked }
                }))(input)
            }
        }

        pub struct PacketPosition {
            x: f64,
            y: f64,
            z: f64,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketPosition {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        bool::deserialize,
                    )),
                    |(x, y, z, on_ground)| PacketPosition { x, y, z, on_ground },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        bool::deserialize,
                    )),
                    |(x, y, z, yaw, pitch, on_ground)| PacketPositionLook {
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

        pub struct PacketLook {
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketLook {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((f32::deserialize, f32::deserialize, bool::deserialize)),
                    |(yaw, pitch, on_ground)| PacketLook {
                        yaw,
                        pitch,
                        on_ground,
                    },
                ))(input)
            }
        }

        pub struct PacketFlying {
            on_ground: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketFlying {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.on_ground, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((bool::deserialize,)), |(on_ground,)| {
                    PacketFlying { on_ground }
                }))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = f32::serialize(&self.yaw, w)?;
                let w = f32::serialize(&self.pitch, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        f64::deserialize,
                        f64::deserialize,
                        f64::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                    )),
                    |(x, y, z, yaw, pitch)| PacketVehicleMove {
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                    },
                ))(input)
            }
        }

        pub struct PacketSteerBoat {
            left_paddle: bool,
            right_paddle: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSteerBoat {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = bool::serialize(&self.left_paddle, w)?;
                let w = bool::serialize(&self.right_paddle, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((bool::deserialize, bool::deserialize)),
                    |(left_paddle, right_paddle)| PacketSteerBoat {
                        left_paddle,
                        right_paddle,
                    },
                ))(input)
            }
        }

        pub struct PacketCraftRecipeRequest<'a> {
            window_id: i8,
            recipe: VarString<'a>,
            make_all: bool,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketCraftRecipeRequest<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.window_id, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe, w)?;
                let w = bool::serialize(&self.make_all, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        i8::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        bool::deserialize,
                    )),
                    |(window_id, recipe, make_all)| PacketCraftRecipeRequest {
                        window_id,
                        recipe,
                        make_all,
                    },
                ))(input)
            }
        }

        pub struct PacketAbilities {
            flags: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketAbilities {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i8::serialize(&self.flags, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((i8::deserialize,)), |(flags,)| {
                    PacketAbilities { flags }
                }))(input)
            }
        }

        pub struct PacketBlockDig {
            status: VarInt,
            location: Position,
            face: i8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketBlockDig {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.status, w)?;
                let w = Position::serialize(&self.location, w)?;
                let w = i8::serialize(&self.face, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        Position::deserialize,
                        i8::deserialize,
                    )),
                    |(status, location, face)| PacketBlockDig {
                        status,
                        location,
                        face,
                    },
                ))(input)
            }
        }

        pub struct PacketEntityAction {
            entity_id: VarInt,
            action_id: VarInt,
            jump_boost: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketEntityAction {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;
                let w = VarInt::serialize(&self.action_id, w)?;
                let w = VarInt::serialize(&self.jump_boost, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        VarInt::deserialize,
                        VarInt::deserialize,
                    )),
                    |(entity_id, action_id, jump_boost)| PacketEntityAction {
                        entity_id,
                        action_id,
                        jump_boost,
                    },
                ))(input)
            }
        }

        pub struct PacketSteerVehicle {
            sideways: f32,
            forward: f32,
            jump: u8,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSteerVehicle {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = f32::serialize(&self.sideways, w)?;
                let w = f32::serialize(&self.forward, w)?;
                let w = u8::serialize(&self.jump, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((f32::deserialize, f32::deserialize, u8::deserialize)),
                    |(sideways, forward, jump)| PacketSteerVehicle {
                        sideways,
                        forward,
                        jump,
                    },
                ))(input)
            }
        }

        pub struct PacketDisplayedRecipe<'a> {
            recipe_id: VarString<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketDisplayedRecipe<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.recipe_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((PrefixedString::<'a, VarInt>::deserialize,)),
                    |(recipe_id,)| PacketDisplayedRecipe { recipe_id },
                ))(input)
            }
        }

        pub struct PacketRecipeBook {
            book_id: VarInt,
            book_open: bool,
            filter_active: bool,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketRecipeBook {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.book_id, w)?;
                let w = bool::serialize(&self.book_open, w)?;
                let w = bool::serialize(&self.filter_active, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        bool::deserialize,
                        bool::deserialize,
                    )),
                    |(book_id, book_open, filter_active)| PacketRecipeBook {
                        book_id,
                        book_open,
                        filter_active,
                    },
                ))(input)
            }
        }

        pub struct PacketResourcePackReceive {
            result: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketResourcePackReceive {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.result, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((VarInt::deserialize,)), |(result,)| {
                    PacketResourcePackReceive { result }
                }))(input)
            }
        }

        pub struct PacketHeldItemSlot {
            slot_id: i16,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketHeldItemSlot {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i16::serialize(&self.slot_id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((i16::deserialize,)), |(slot_id,)| {
                    PacketHeldItemSlot { slot_id }
                }))(input)
            }
        }

        pub struct PacketSetCreativeSlot {
            slot: i16,
            item: Slot,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSetCreativeSlot {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i16::serialize(&self.slot, w)?;
                let w = Slot::serialize(&self.item, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((i16::deserialize, Slot::deserialize)),
                    |(slot, item)| PacketSetCreativeSlot { slot, item },
                ))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.target, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.pool, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.final_state, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.joint_type, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Position::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(location, name, target, pool, final_state, joint_type)| {
                        PacketUpdateJigsawBlock {
                            location,
                            name,
                            target,
                            pool,
                            final_state,
                            joint_type,
                        }
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Position::serialize(&self.location, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text1, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text2, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text3, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.text4, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(
                    nom::sequence::tuple((
                        Position::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                        PrefixedString::<'a, VarInt>::deserialize,
                    )),
                    |(location, text1, text2, text3, text4)| PacketUpdateSign {
                        location,
                        text1,
                        text2,
                        text3,
                        text4,
                    },
                ))(input)
            }
        }

        pub struct PacketArmAnimation {
            hand: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketArmAnimation {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((VarInt::deserialize,)), |(hand,)| {
                    PacketArmAnimation { hand }
                }))(input)
            }
        }

        pub struct PacketSpectate {
            target: Uuid,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketSpectate {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.target, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((Uuid::deserialize,)), |(target,)| {
                    PacketSpectate { target }
                }))(input)
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                (nom::combinator::map(
                    nom::sequence::tuple((
                        VarInt::deserialize,
                        Position::deserialize,
                        VarInt::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        f32::deserialize,
                        bool::deserialize,
                    )),
                    |(hand, location, direction, cursor_x, cursor_y, cursor_z, inside_block)| {
                        PacketBlockPlace {
                            hand,
                            location,
                            direction,
                            cursor_x,
                            cursor_y,
                            cursor_z,
                            inside_block,
                        }
                    },
                ))(input)
            }
        }

        pub struct PacketUseItem {
            hand: VarInt,
        }

        impl<'t> protocol_lib::Packet<'t> for PacketUseItem {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.hand, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((VarInt::deserialize,)), |(hand,)| {
                    PacketUseItem { hand }
                }))(input)
            }
        }

        pub enum TabId<'a> {
            TabId0(VarString<'a>),
            TabId1(Void),
            Default(Void),
        }

        impl<'a> TabId<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    TabId::<'a>::TabId0(_) => "0",
                    TabId::<'a>::TabId1(_) => "1",
                    _ => "",
                }
            }
        }
        pub struct PacketAdvancementTab<'a> {
            action: VarInt,
            tab_id: TabId<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for PacketAdvancementTab<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action, w)?;

                let w = match &self.tab_id {
                    TabId::<'a>::TabId0(val) => {
                        let w = PrefixedString::<'a, VarInt>::serialize(&val, w)?;
                        w
                    }
                    TabId::<'a>::TabId1(val) => {
                        let w = Void::serialize(&val, w)?;
                        w
                    }
                    TabId::<'a>::Default(val) => Void::serialize(val, w)?,
                };

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_action) = (VarInt::deserialize)(input)?;
                    let (input, self_tab_id) = (|input| match &format!("{}", self_action)[..] {
                        "0" => nom::combinator::map(
                            PrefixedString::<'a, VarInt>::deserialize,
                            TabId::<'a>::TabId0,
                        )(input),
                        "1" => nom::combinator::map(Void::deserialize, TabId::<'a>::TabId1)(input),
                        _ => nom::combinator::map(Void::deserialize, TabId::<'a>::Default)(input),
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
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
                let w = i32::serialize(&self.id, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (nom::combinator::map(nom::sequence::tuple((i32::deserialize,)), |(id,)| {
                    PacketPong { id }
                }))(input)
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
            ParamsPosition(PacketPosition),
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
            Default(Void),
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> &'static str {
                match self {
                    Params::<'a>::TeleportConfirm(_) => "teleport_confirm",
                    Params::<'a>::QueryBlockNbt(_) => "query_block_nbt",
                    Params::<'a>::SetDifficulty(_) => "set_difficulty",
                    Params::<'a>::EditBook(_) => "edit_book",
                    Params::<'a>::QueryEntityNbt(_) => "query_entity_nbt",
                    Params::<'a>::PickItem(_) => "pick_item",
                    Params::<'a>::NameItem(_) => "name_item",
                    Params::<'a>::SelectTrade(_) => "select_trade",
                    Params::<'a>::SetBeaconEffect(_) => "set_beacon_effect",
                    Params::<'a>::UpdateCommandBlock(_) => "update_command_block",
                    Params::<'a>::UpdateCommandBlockMinecart(_) => "update_command_block_minecart",
                    Params::<'a>::UpdateStructureBlock(_) => "update_structure_block",
                    Params::<'a>::TabComplete(_) => "tab_complete",
                    Params::<'a>::Chat(_) => "chat",
                    Params::<'a>::ClientCommand(_) => "client_command",
                    Params::<'a>::Settings(_) => "settings",
                    Params::<'a>::EnchantItem(_) => "enchant_item",
                    Params::<'a>::WindowClick(_) => "window_click",
                    Params::<'a>::CloseWindow(_) => "close_window",
                    Params::<'a>::CustomPayload(_) => "custom_payload",
                    Params::<'a>::UseEntity(_) => "use_entity",
                    Params::<'a>::GenerateStructure(_) => "generate_structure",
                    Params::<'a>::KeepAlive(_) => "keep_alive",
                    Params::<'a>::LockDifficulty(_) => "lock_difficulty",
                    Params::<'a>::ParamsPosition(_) => "position",
                    Params::<'a>::PositionLook(_) => "position_look",
                    Params::<'a>::Look(_) => "look",
                    Params::<'a>::Flying(_) => "flying",
                    Params::<'a>::VehicleMove(_) => "vehicle_move",
                    Params::<'a>::SteerBoat(_) => "steer_boat",
                    Params::<'a>::CraftRecipeRequest(_) => "craft_recipe_request",
                    Params::<'a>::Abilities(_) => "abilities",
                    Params::<'a>::BlockDig(_) => "block_dig",
                    Params::<'a>::EntityAction(_) => "entity_action",
                    Params::<'a>::SteerVehicle(_) => "steer_vehicle",
                    Params::<'a>::DisplayedRecipe(_) => "displayed_recipe",
                    Params::<'a>::RecipeBook(_) => "recipe_book",
                    Params::<'a>::ResourcePackReceive(_) => "resource_pack_receive",
                    Params::<'a>::HeldItemSlot(_) => "held_item_slot",
                    Params::<'a>::SetCreativeSlot(_) => "set_creative_slot",
                    Params::<'a>::UpdateJigsawBlock(_) => "update_jigsaw_block",
                    Params::<'a>::UpdateSign(_) => "update_sign",
                    Params::<'a>::ArmAnimation(_) => "arm_animation",
                    Params::<'a>::Spectate(_) => "spectate",
                    Params::<'a>::BlockPlace(_) => "block_place",
                    Params::<'a>::UseItem(_) => "use_item",
                    Params::<'a>::AdvancementTab(_) => "advancement_tab",
                    Params::<'a>::Pong(_) => "pong",
                    _ => "",
                }
            }
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }

        impl<'t: 'a, 'a> protocol_lib::Packet<'t> for Packet<'a> {
            fn serialize<W: std::io::Write>(
                &self,
                w: cookie_factory::WriteContext<W>,
            ) -> cookie_factory::GenResult<W> {
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
                };
                let tag2 = str::parse(tag).unwrap();
                let w = VarInt::serialize(&tag2, w)?;

                let w = match &self.params {
                    Params::<'a>::TeleportConfirm(val) => {
                        let w = PacketTeleportConfirm::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::QueryBlockNbt(val) => {
                        let w = PacketQueryBlockNbt::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetDifficulty(val) => {
                        let w = PacketSetDifficulty::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EditBook(val) => {
                        let w = PacketEditBook::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::QueryEntityNbt(val) => {
                        let w = PacketQueryEntityNbt::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::PickItem(val) => {
                        let w = PacketPickItem::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::NameItem(val) => {
                        let w = PacketNameItem::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SelectTrade(val) => {
                        let w = PacketSelectTrade::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetBeaconEffect(val) => {
                        let w = PacketSetBeaconEffect::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateCommandBlock(val) => {
                        let w = PacketUpdateCommandBlock::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateCommandBlockMinecart(val) => {
                        let w = PacketUpdateCommandBlockMinecart::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateStructureBlock(val) => {
                        let w = PacketUpdateStructureBlock::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::TabComplete(val) => {
                        let w = PacketTabComplete::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Chat(val) => {
                        let w = PacketChat::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ClientCommand(val) => {
                        let w = PacketClientCommand::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Settings(val) => {
                        let w = PacketSettings::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EnchantItem(val) => {
                        let w = PacketEnchantItem::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::WindowClick(val) => {
                        let w = PacketWindowClick::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::CloseWindow(val) => {
                        let w = PacketCloseWindow::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::CustomPayload(val) => {
                        let w = PacketCustomPayload::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UseEntity(val) => {
                        let w = PacketUseEntity::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::GenerateStructure(val) => {
                        let w = PacketGenerateStructure::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::KeepAlive(val) => {
                        let w = PacketKeepAlive::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::LockDifficulty(val) => {
                        let w = PacketLockDifficulty::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ParamsPosition(val) => {
                        let w = PacketPosition::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::PositionLook(val) => {
                        let w = PacketPositionLook::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Look(val) => {
                        let w = PacketLook::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Flying(val) => {
                        let w = PacketFlying::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::VehicleMove(val) => {
                        let w = PacketVehicleMove::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SteerBoat(val) => {
                        let w = PacketSteerBoat::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::CraftRecipeRequest(val) => {
                        let w = PacketCraftRecipeRequest::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Abilities(val) => {
                        let w = PacketAbilities::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::BlockDig(val) => {
                        let w = PacketBlockDig::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::EntityAction(val) => {
                        let w = PacketEntityAction::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SteerVehicle(val) => {
                        let w = PacketSteerVehicle::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::DisplayedRecipe(val) => {
                        let w = PacketDisplayedRecipe::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::RecipeBook(val) => {
                        let w = PacketRecipeBook::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ResourcePackReceive(val) => {
                        let w = PacketResourcePackReceive::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::HeldItemSlot(val) => {
                        let w = PacketHeldItemSlot::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::SetCreativeSlot(val) => {
                        let w = PacketSetCreativeSlot::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateJigsawBlock(val) => {
                        let w = PacketUpdateJigsawBlock::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UpdateSign(val) => {
                        let w = PacketUpdateSign::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::ArmAnimation(val) => {
                        let w = PacketArmAnimation::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Spectate(val) => {
                        let w = PacketSpectate::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::BlockPlace(val) => {
                        let w = PacketBlockPlace::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::UseItem(val) => {
                        let w = PacketUseItem::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::AdvancementTab(val) => {
                        let w = PacketAdvancementTab::<'a>::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Pong(val) => {
                        let w = PacketPong::serialize(&val, w)?;
                        w
                    }
                    Params::<'a>::Default(val) => Void::serialize(val, w)?,
                };

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

                            _ => {
                                return Err(nom::Err::Error(nom::error::Error::new(
                                    input,
                                    nom::error::ErrorKind::Verify,
                                )))
                            }
                        };
                        Ok((input, val))
                    })(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "teleport_confirm" => nom::combinator::map(
                            PacketTeleportConfirm::deserialize,
                            Params::<'a>::TeleportConfirm,
                        )(input),
                        "query_block_nbt" => nom::combinator::map(
                            PacketQueryBlockNbt::deserialize,
                            Params::<'a>::QueryBlockNbt,
                        )(input),
                        "set_difficulty" => nom::combinator::map(
                            PacketSetDifficulty::deserialize,
                            Params::<'a>::SetDifficulty,
                        )(input),
                        "edit_book" => nom::combinator::map(
                            PacketEditBook::<'a>::deserialize,
                            Params::<'a>::EditBook,
                        )(input),
                        "query_entity_nbt" => nom::combinator::map(
                            PacketQueryEntityNbt::deserialize,
                            Params::<'a>::QueryEntityNbt,
                        )(input),
                        "pick_item" => nom::combinator::map(
                            PacketPickItem::deserialize,
                            Params::<'a>::PickItem,
                        )(input),
                        "name_item" => nom::combinator::map(
                            PacketNameItem::<'a>::deserialize,
                            Params::<'a>::NameItem,
                        )(input),
                        "select_trade" => nom::combinator::map(
                            PacketSelectTrade::deserialize,
                            Params::<'a>::SelectTrade,
                        )(input),
                        "set_beacon_effect" => nom::combinator::map(
                            PacketSetBeaconEffect::deserialize,
                            Params::<'a>::SetBeaconEffect,
                        )(input),
                        "update_command_block" => nom::combinator::map(
                            PacketUpdateCommandBlock::<'a>::deserialize,
                            Params::<'a>::UpdateCommandBlock,
                        )(input),
                        "update_command_block_minecart" => nom::combinator::map(
                            PacketUpdateCommandBlockMinecart::<'a>::deserialize,
                            Params::<'a>::UpdateCommandBlockMinecart,
                        )(input),
                        "update_structure_block" => nom::combinator::map(
                            PacketUpdateStructureBlock::<'a>::deserialize,
                            Params::<'a>::UpdateStructureBlock,
                        )(input),
                        "tab_complete" => nom::combinator::map(
                            PacketTabComplete::<'a>::deserialize,
                            Params::<'a>::TabComplete,
                        )(input),
                        "chat" => nom::combinator::map(
                            PacketChat::<'a>::deserialize,
                            Params::<'a>::Chat,
                        )(input),
                        "client_command" => nom::combinator::map(
                            PacketClientCommand::deserialize,
                            Params::<'a>::ClientCommand,
                        )(input),
                        "settings" => nom::combinator::map(
                            PacketSettings::<'a>::deserialize,
                            Params::<'a>::Settings,
                        )(input),
                        "enchant_item" => nom::combinator::map(
                            PacketEnchantItem::deserialize,
                            Params::<'a>::EnchantItem,
                        )(input),
                        "window_click" => nom::combinator::map(
                            PacketWindowClick::deserialize,
                            Params::<'a>::WindowClick,
                        )(input),
                        "close_window" => nom::combinator::map(
                            PacketCloseWindow::deserialize,
                            Params::<'a>::CloseWindow,
                        )(input),
                        "custom_payload" => nom::combinator::map(
                            PacketCustomPayload::<'a>::deserialize,
                            Params::<'a>::CustomPayload,
                        )(input),
                        "use_entity" => nom::combinator::map(
                            PacketUseEntity::deserialize,
                            Params::<'a>::UseEntity,
                        )(input),
                        "generate_structure" => nom::combinator::map(
                            PacketGenerateStructure::deserialize,
                            Params::<'a>::GenerateStructure,
                        )(input),
                        "keep_alive" => nom::combinator::map(
                            PacketKeepAlive::deserialize,
                            Params::<'a>::KeepAlive,
                        )(input),
                        "lock_difficulty" => nom::combinator::map(
                            PacketLockDifficulty::deserialize,
                            Params::<'a>::LockDifficulty,
                        )(input),
                        "position" => nom::combinator::map(
                            PacketPosition::deserialize,
                            Params::<'a>::ParamsPosition,
                        )(input),
                        "position_look" => nom::combinator::map(
                            PacketPositionLook::deserialize,
                            Params::<'a>::PositionLook,
                        )(input),
                        "look" => {
                            nom::combinator::map(PacketLook::deserialize, Params::<'a>::Look)(input)
                        }
                        "flying" => nom::combinator::map(
                            PacketFlying::deserialize,
                            Params::<'a>::Flying,
                        )(input),
                        "vehicle_move" => nom::combinator::map(
                            PacketVehicleMove::deserialize,
                            Params::<'a>::VehicleMove,
                        )(input),
                        "steer_boat" => nom::combinator::map(
                            PacketSteerBoat::deserialize,
                            Params::<'a>::SteerBoat,
                        )(input),
                        "craft_recipe_request" => nom::combinator::map(
                            PacketCraftRecipeRequest::<'a>::deserialize,
                            Params::<'a>::CraftRecipeRequest,
                        )(input),
                        "abilities" => nom::combinator::map(
                            PacketAbilities::deserialize,
                            Params::<'a>::Abilities,
                        )(input),
                        "block_dig" => nom::combinator::map(
                            PacketBlockDig::deserialize,
                            Params::<'a>::BlockDig,
                        )(input),
                        "entity_action" => nom::combinator::map(
                            PacketEntityAction::deserialize,
                            Params::<'a>::EntityAction,
                        )(input),
                        "steer_vehicle" => nom::combinator::map(
                            PacketSteerVehicle::deserialize,
                            Params::<'a>::SteerVehicle,
                        )(input),
                        "displayed_recipe" => nom::combinator::map(
                            PacketDisplayedRecipe::<'a>::deserialize,
                            Params::<'a>::DisplayedRecipe,
                        )(input),
                        "recipe_book" => nom::combinator::map(
                            PacketRecipeBook::deserialize,
                            Params::<'a>::RecipeBook,
                        )(input),
                        "resource_pack_receive" => nom::combinator::map(
                            PacketResourcePackReceive::deserialize,
                            Params::<'a>::ResourcePackReceive,
                        )(input),
                        "held_item_slot" => nom::combinator::map(
                            PacketHeldItemSlot::deserialize,
                            Params::<'a>::HeldItemSlot,
                        )(input),
                        "set_creative_slot" => nom::combinator::map(
                            PacketSetCreativeSlot::deserialize,
                            Params::<'a>::SetCreativeSlot,
                        )(input),
                        "update_jigsaw_block" => nom::combinator::map(
                            PacketUpdateJigsawBlock::<'a>::deserialize,
                            Params::<'a>::UpdateJigsawBlock,
                        )(input),
                        "update_sign" => nom::combinator::map(
                            PacketUpdateSign::<'a>::deserialize,
                            Params::<'a>::UpdateSign,
                        )(input),
                        "arm_animation" => nom::combinator::map(
                            PacketArmAnimation::deserialize,
                            Params::<'a>::ArmAnimation,
                        )(input),
                        "spectate" => nom::combinator::map(
                            PacketSpectate::deserialize,
                            Params::<'a>::Spectate,
                        )(input),
                        "block_place" => nom::combinator::map(
                            PacketBlockPlace::deserialize,
                            Params::<'a>::BlockPlace,
                        )(input),
                        "use_item" => nom::combinator::map(
                            PacketUseItem::deserialize,
                            Params::<'a>::UseItem,
                        )(input),
                        "advancement_tab" => nom::combinator::map(
                            PacketAdvancementTab::<'a>::deserialize,
                            Params::<'a>::AdvancementTab,
                        )(input),
                        "pong" => {
                            nom::combinator::map(PacketPong::deserialize, Params::<'a>::Pong)(input)
                        }
                        _ => nom::combinator::map(Void::deserialize, Params::<'a>::Default)(input),
                    })(input)?;
                    Ok((
                        input,
                        Packet {
                            name: self_name,
                            params: self_params,
                        },
                    ))
                })(input)
            }
        }
    }
}
