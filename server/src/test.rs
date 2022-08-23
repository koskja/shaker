
use nom::{combinator::map, sequence::tuple};
#[allow(unused_imports)]
use protocol_lib::{types::*, Packet};
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

impl<'t> Packet<'t> for Position {
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
#[derive(protocol_lib::Packet)]
pub struct RTrue {
    item_id: VarInt,
    item_count: i8,
    nbt_data: OptionalNbt,
}
pub enum Ident0 {
    RFalse,
    RTrue(RTrue),
    Default,
}

impl Ident0 {
    pub fn discriminant(&self) -> bool {
        match self {
            Ident0::RFalse => false,
            Ident0::RTrue(_) => true,
            Ident0::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t> Packet<'t> for Slot {
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
#[derive(protocol_lib::Packet)]
pub struct Data2 {
    block_state: VarInt,
}
#[derive(protocol_lib::Packet)]
pub struct Data3 {
    block_state: VarInt,
}
#[derive(protocol_lib::Packet)]
pub struct Data14 {
    red: f32,
    green: f32,
    blue: f32,
    scale: f32,
}
#[derive(protocol_lib::Packet)]
pub struct Data15 {
    from_red: f32,
    from_green: f32,
    from_blue: f32,
    scale: f32,
    to_red: f32,
    to_green: f32,
    to_blue: f32,
}
#[derive(protocol_lib::Packet)]
pub struct Data24 {
    block_state: VarInt,
}
#[derive(protocol_lib::Packet)]
pub struct Data35 {
    item: Slot,
}
pub enum Destination {
    Block(Position),
    Entity(VarInt),
    Default,
}

impl Destination {
    pub fn discriminant(&self) -> &str {
        match self {
            Destination::Block(_) => "minecraft:block",
            Destination::Entity(_) => "minecraft:entity",
            Destination::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t: 'a, 'a> Packet<'t> for Data36<'a> {
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
    pub fn discriminant(&self) -> i32 {
        match self {
            Data::Data2(_) => 2,
            Data::Data3(_) => 3,
            Data::Data14(_) => 14,
            Data::Data15(_) => 15,
            Data::Data24(_) => 24,
            Data::Data35(_) => 35,
            Data::Data36(_) => 36,
            Data::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t: 'a, 'a> Packet<'t> for Particle<'a> {
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
#[derive(protocol_lib::Packet)]
pub struct Ident1 {
    key: u8,
    r_type: VarInt,
}
#[derive(protocol_lib::Packet)]
pub struct Value8 {
    pitch: f32,
    yaw: f32,
    roll: f32,
}
#[derive(protocol_lib::Packet)]
pub struct Value16 {
    villager_type: VarInt,
    villager_profession: VarInt,
    level: VarInt,
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
    pub fn discriminant(&self) -> i32 {
        match self {
            EntityMetadata::Value0(_) => 0,
            EntityMetadata::Value1(_) => 1,
            EntityMetadata::Value2(_) => 2,
            EntityMetadata::Value3(_) => 3,
            EntityMetadata::Value4(_) => 4,
            EntityMetadata::Value5(_) => 5,
            EntityMetadata::Value6(_) => 6,
            EntityMetadata::Value7(_) => 7,
            EntityMetadata::Value8(_) => 8,
            EntityMetadata::Value9(_) => 9,
            EntityMetadata::Value10(_) => 10,
            EntityMetadata::Value11(_) => 11,
            EntityMetadata::Value12(_) => 12,
            EntityMetadata::Value13(_) => 13,
            EntityMetadata::Value14(_) => 14,
            EntityMetadata::Value15(_) => 15,
            EntityMetadata::Value16(_) => 16,
            EntityMetadata::Value17(_) => 17,
            EntityMetadata::Value18(_) => 18,
            EntityMetadata::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t: 'a, 'a> Packet<'t> for EntityMetadataWrapper<'a> {
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
#[derive(protocol_lib::Packet)]
pub struct MinecraftSmeltingFormat<'a> {
    group: VarString<'a>,
    ingredient: VarArray<Slot>,
    result: Slot,
    experience: f32,
    cook_time: VarInt,
}
#[derive(protocol_lib::Packet)]
pub struct Tag<'a> {
    tag_name: VarString<'a>,
    entries: VarArray<VarInt>,
}
pub struct Ident2 {
    x: u8,
    z: u8,
}

impl<'t> Packet<'t> for Ident2 {
    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = write_bits(&[(self.x as u64, 4), (self.z as u64, 4)], w)?;

        Ok(w)
    }

    fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
        (nom::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(map(tuple((parse_bits_unsigned(4), parse_bits_unsigned(4))), |(x, z)| Ident2 { x, z })))(input)
    }
}
#[derive(protocol_lib::Packet)]
pub struct ChunkBlockEntity {
    ident2: Ident2,
    y: i16,
    r_type: VarInt,
    nbt_data: OptionalNbt,
}
pub struct Flags {
    unused: u8,
    has_custom_suggestions: u8,
    has_redirect_node: u8,
    has_command: u8,
    command_node_type: u8,
}

impl<'t> Packet<'t> for Flags {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            RedirectNode::RedirectNode1(_) => 1,
            RedirectNode::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
        let w = match &self {
            RedirectNode::RedirectNode1(val) => VarInt::serialize(&val, w)?,
            RedirectNode::Default => w,
        };

        Ok(w)
    }
}
#[derive(protocol_lib::Packet)]
pub struct ExtraNodeData1<'a> {
    name: VarString<'a>,
}
pub struct FloatFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

impl<'t> Packet<'t> for FloatFlags {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            Min::Min1(_) => 1,
            Min::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            Max::Max1(_) => 1,
            Max::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t> Packet<'t> for Float {
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

impl<'t> Packet<'t> for DoubleFlags {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            DoubleMin::DoubleMin1(_) => 1,
            DoubleMin::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            DoubleMax::DoubleMax1(_) => 1,
            DoubleMax::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t> Packet<'t> for Double {
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

impl<'t> Packet<'t> for IntegerFlags {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            IntegerMin::IntegerMin1(_) => 1,
            IntegerMin::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            IntegerMax::IntegerMax1(_) => 1,
            IntegerMax::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t> Packet<'t> for Integer {
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

impl<'t> Packet<'t> for LongFlags {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            LongMin::LongMin1(_) => 1,
            LongMin::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            LongMax::LongMax1(_) => 1,
            LongMax::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t> Packet<'t> for Long {
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

impl<'t> Packet<'t> for MinecraftEntity {
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

impl<'t> Packet<'t> for ScoreHolder {
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
#[derive(protocol_lib::Packet)]
pub struct Range {
    allow_decimals: bool,
}
#[derive(protocol_lib::Packet)]
pub struct ResourceOrTag<'a> {
    registry: VarString<'a>,
}
#[derive(protocol_lib::Packet)]
pub struct Resource<'a> {
    registry: VarString<'a>,
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
    pub fn discriminant(&self) -> &str {
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
            Properties::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            SuggestionType::SuggestionType1(_) => 1,
            SuggestionType::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
    pub fn discriminant(&self) -> u8 {
        match self {
            ExtraNodeData::ExtraNodeData0 => 0,
            ExtraNodeData::ExtraNodeData1(_) => 1,
            ExtraNodeData::ExtraNodeData2(_) => 2,
            ExtraNodeData::Default => todo!(),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
impl<'t: 'a, 'a> Packet<'t> for CommandNode<'a> {
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
                        let (input, self_extra_node_data_ExtraNodeData2_name) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                        let (input, self_extra_node_data_ExtraNodeData2_parser) = (PrefixedString::<'a, VarInt>::deserialize)(input)?;
                        let (input, self_extra_node_data_ExtraNodeData2_properties) = (|input| match &format!("{}", self_extra_node_data_ExtraNodeData2_parser)[..] {
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
                        let (input, self_extra_node_data_ExtraNodeData2_suggestion_type) = (|input| match &format!("{}", self_flags.has_custom_suggestions)[..] {
                            "1" => map(PrefixedString::<'a, VarInt>::deserialize, SuggestionType::SuggestionType1)(input),
                            _ => Ok((input, SuggestionType::Default)),
                        })(input)?;
                        Ok((
                            input,
                            ExtraNodeData2 {
                                name: self_extra_node_data_ExtraNodeData2_name,
                                parser: self_extra_node_data_ExtraNodeData2_parser,
                                properties: self_extra_node_data_ExtraNodeData2_properties,
                                suggestion_type: self_extra_node_data_ExtraNodeData2_suggestion_type,
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
        use protocol_lib::Packet;
        pub enum Params {
            Default,
        }

        impl Params {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Params::Default => w,
                };

                Ok(w)
            }
        }

        pub struct ToClient {
            name: VarInt,
            params: Params,
        }
        impl<'t> Packet<'t> for ToClient {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToClient { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        use protocol_lib::Packet;
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetProtocol<'a> {
            protocol_version: VarInt,
            server_host: VarString<'a>,
            server_port: u16,
            next_state: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketLegacyServerListPing {
            payload: u8,
        }
        pub enum Params<'a> {
            SetProtocol(PacketSetProtocol<'a>),
            LegacyServerListPing(PacketLegacyServerListPing),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::SetProtocol(_) => 0x00,
                    Params::LegacyServerListPing(_) => 0xfe,
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Params::SetProtocol(val) => PacketSetProtocol::serialize(&val, w)?,
                    Params::LegacyServerListPing(val) => PacketLegacyServerListPing::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }

        pub struct ToServer<'a> {
            name: VarInt,
            params: Params<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for ToServer<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "0x00" => map(PacketSetProtocol::deserialize, Params::SetProtocol)(input),
                        "0xfe" => map(PacketLegacyServerListPing::deserialize, Params::LegacyServerListPing)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToServer { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
pub mod status {
    pub mod clientbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        use protocol_lib::Packet;
        #[derive(protocol_lib::Packet)]
        pub struct PacketServerInfo<'a> {
            response: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketPing {
            time: i64,
        }
        pub enum Params<'a> {
            ServerInfo(PacketServerInfo<'a>),
            Ping(PacketPing),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::ServerInfo(_) => 0x00,
                    Params::Ping(_) => 0x01,
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Params::ServerInfo(val) => PacketServerInfo::serialize(&val, w)?,
                    Params::Ping(val) => PacketPing::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }

        pub struct ToClient<'a> {
            name: VarInt,
            params: Params<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for ToClient<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "0x00" => map(PacketServerInfo::deserialize, Params::ServerInfo)(input),
                        "0x01" => map(PacketPing::deserialize, Params::Ping)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToClient { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        use protocol_lib::Packet;
        #[derive(protocol_lib::Packet)]
        pub struct PacketPingStart {}
        #[derive(protocol_lib::Packet)]
        pub struct PacketPing {
            time: i64,
        }
        pub enum Params {
            PingStart(PacketPingStart),
            Ping(PacketPing),
            Default,
        }

        impl Params {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::PingStart(_) => 0x00,
                    Params::Ping(_) => 0x01,
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Params::PingStart(val) => PacketPingStart::serialize(&val, w)?,
                    Params::Ping(val) => PacketPing::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }

        pub struct ToServer {
            name: VarInt,
            params: Params,
        }
        impl<'t> Packet<'t> for ToServer {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "0x00" => map(PacketPingStart::deserialize, Params::PingStart)(input),
                        "0x01" => map(PacketPing::deserialize, Params::Ping)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToServer { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
pub mod login {
    pub mod clientbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        use protocol_lib::Packet;
        #[derive(protocol_lib::Packet)]
        pub struct PacketDisconnect<'a> {
            reason: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEncryptionBegin<'a> {
            server_id: VarString<'a>,
            public_key: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSuccess<'a> {
            uuid: Uuid,
            username: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCompress {
            threshold: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketLoginPluginRequest<'a> {
            message_id: VarInt,
            channel: VarString<'a>,
            data: RestBuffer<'a>,
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::Disconnect(_) => 0x00,
                    Params::EncryptionBegin(_) => 0x01,
                    Params::Success(_) => 0x02,
                    Params::Compress(_) => 0x03,
                    Params::LoginPluginRequest(_) => 0x04,
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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

        pub struct ToClient<'a> {
            name: VarInt,
            params: Params<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for ToClient<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "0x00" => map(PacketDisconnect::deserialize, Params::Disconnect)(input),
                        "0x01" => map(PacketEncryptionBegin::deserialize, Params::EncryptionBegin)(input),
                        "0x02" => map(PacketSuccess::deserialize, Params::Success)(input),
                        "0x03" => map(PacketCompress::deserialize, Params::Compress)(input),
                        "0x04" => map(PacketLoginPluginRequest::deserialize, Params::LoginPluginRequest)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToClient { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        use protocol_lib::Packet;
        #[derive(protocol_lib::Packet)]
        pub struct PacketLoginStart<'a> {
            username: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEncryptionBegin<'a> {
            shared_secret: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketLoginPluginResponse<'a> {
            message_id: VarInt,
            data: Option<RestBuffer<'a>>,
        }
        pub enum Params<'a> {
            LoginStart(PacketLoginStart<'a>),
            EncryptionBegin(PacketEncryptionBegin<'a>),
            LoginPluginResponse(PacketLoginPluginResponse<'a>),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::LoginStart(_) => 0x00,
                    Params::EncryptionBegin(_) => 0x01,
                    Params::LoginPluginResponse(_) => 0x02,
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Params::LoginStart(val) => PacketLoginStart::serialize(&val, w)?,
                    Params::EncryptionBegin(val) => PacketEncryptionBegin::serialize(&val, w)?,
                    Params::LoginPluginResponse(val) => PacketLoginPluginResponse::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }

        pub struct ToServer<'a> {
            name: VarInt,
            params: Params<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for ToServer<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "0x00" => map(PacketLoginStart::deserialize, Params::LoginStart)(input),
                        "0x01" => map(PacketEncryptionBegin::deserialize, Params::EncryptionBegin)(input),
                        "0x02" => map(PacketLoginPluginResponse::deserialize, Params::LoginPluginResponse)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToServer { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
pub mod play {
    pub mod clientbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        use protocol_lib::Packet;
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketSpawnEntityExperienceOrb {
            entity_id: VarInt,
            x: f64,
            y: f64,
            z: f64,
            count: i16,
        }
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketSpawnEntityPainting {
            entity_id: VarInt,
            entity_uuid: Uuid,
            title: VarInt,
            location: Position,
            direction: u8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketNamedEntitySpawn {
            entity_id: VarInt,
            player_uuid: Uuid,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketAnimation {
            entity_id: VarInt,
            animation: u8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct StatisticsEntry {
            category_id: VarInt,
            statistic_id: VarInt,
            value: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketStatistics {
            entries: VarArray<StatisticsEntry>,
        }
        pub struct Ident3Flags {
            unused: u32,
            hidden: u8,
            show_toast: u8,
            has_background_texture: u8,
        }

        impl<'t> Packet<'t> for Ident3Flags {
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
                    |(unused, hidden, show_toast, has_background_texture)| Ident3Flags {
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
            pub fn discriminant(&self) -> u8 {
                match self {
                    BackgroundTexture::BackgroundTexture1(_) => 1,
                    BackgroundTexture::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    BackgroundTexture::BackgroundTexture1(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    BackgroundTexture::Default => w,
                };

                Ok(w)
            }
        }

        pub struct Ident3<'a> {
            title: VarString<'a>,
            description: VarString<'a>,
            icon: Slot,
            frame_type: VarInt,
            flags: Ident3Flags,
            background_texture: BackgroundTexture<'a>,
            x_cord: f32,
            y_cord: f32,
        }
        impl<'t: 'a, 'a> Packet<'t> for Ident3<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.title, w)?;
                let w = PrefixedString::<'a, VarInt>::serialize(&self.description, w)?;
                let w = Slot::serialize(&self.icon, w)?;
                let w = VarInt::serialize(&self.frame_type, w)?;
                let w = Ident3Flags::serialize(&self.flags, w)?;
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
                    let (input, self_flags) = (Ident3Flags::deserialize)(input)?;
                    let (input, self_background_texture) = (|input| match &format!("{}", self_flags.has_background_texture)[..] {
                        "1" => map(PrefixedString::<'a, VarInt>::deserialize, BackgroundTexture::BackgroundTexture1)(input),
                        _ => Ok((input, BackgroundTexture::Default)),
                    })(input)?;
                    let (input, self_x_cord) = (f32::deserialize)(input)?;
                    let (input, self_y_cord) = (f32::deserialize)(input)?;
                    Ok((
                        input,
                        Ident3 {
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
        #[derive(protocol_lib::Packet)]
        pub struct CriteriaItem<'a> {
            key: VarString<'a>,
            value: Void,
        }
        #[derive(protocol_lib::Packet)]
        pub struct AdvancementMappingItemValue<'a> {
            parent_id: Option<VarString<'a>>,
            display_data: Option<Ident3<'a>>,
            criteria: VarArray<CriteriaItem<'a>>,
            requirements: VarArray<VarStringArray<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct AdvancementMappingItem<'a> {
            key: VarString<'a>,
            value: AdvancementMappingItemValue<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct ProgressMappingItemValueItem<'a> {
            criterion_identifier: VarString<'a>,
            criterion_progress: Option<i64>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct ProgressMappingItem<'a> {
            key: VarString<'a>,
            value: VarArray<ProgressMappingItemValueItem<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketAdvancements<'a> {
            reset: bool,
            advancement_mapping: VarArray<AdvancementMappingItem<'a>>,
            identifiers: VarStringArray<'a>,
            progress_mapping: VarArray<ProgressMappingItem<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketBlockBreakAnimation {
            entity_id: VarInt,
            location: Position,
            destroy_stage: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketTileEntityData {
            location: Position,
            action: VarInt,
            nbt_data: OptionalNbt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketBlockAction {
            location: Position,
            byte1: u8,
            byte2: u8,
            block_id: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketBlockChange {
            location: Position,
            r_type: VarInt,
        }
        pub enum Title<'a> {
            Title0(VarString<'a>),
            Title3(VarString<'a>),
            Default,
        }

        impl<'a> Title<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Title::Title0(_) => 0,
                    Title::Title3(_) => 3,
                    Title::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Title::Title0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Title::Title3(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Title::Default => w,
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    Health::Health0(_) => 0,
                    Health::Health2(_) => 2,
                    Health::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    Color::Color0(_) => 0,
                    Color::Color4(_) => 4,
                    Color::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    Dividers::Dividers0(_) => 0,
                    Dividers::Dividers4(_) => 4,
                    Dividers::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    BossBarFlags::BossBarFlags0(_) => 0,
                    BossBarFlags::BossBarFlags5(_) => 5,
                    BossBarFlags::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            title: Title<'a>,
            health: Health,
            color: Color,
            dividers: Dividers,
            flags: BossBarFlags,
        }
        impl<'t: 'a, 'a> Packet<'t> for PacketBossBar<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = Uuid::serialize(&self.entity_uuid, w)?;
                let w = VarInt::serialize(&self.action, w)?;
                let w = Title::serialize(&self.title, w)?;
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
                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, Title::Title0)(input),
                        "3" => map(PrefixedString::<'a, VarInt>::deserialize, Title::Title3)(input),
                        _ => Ok((input, Title::Default)),
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketDifficulty {
            difficulty: u8,
            difficulty_locked: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct Matche<'a> {
            r_match: VarString<'a>,
            tooltip: Option<VarString<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketTabComplete<'a> {
            transaction_id: VarInt,
            start: VarInt,
            length: VarInt,
            matches: VarArray<Matche<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketDeclareCommands<'a> {
            nodes: VarArray<CommandNode<'a>>,
            root_index: VarInt,
        }
        pub enum EntityId {
            True(VarInt),
            Default,
        }

        impl EntityId {
            pub fn discriminant(&self) -> bool {
                match self {
                    EntityId::True(_) => true,
                    EntityId::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    EntityId::True(val) => VarInt::serialize(&val, w)?,
                    EntityId::Default => w,
                };

                Ok(w)
            }
        }
        pub enum EntityFeetEyes<'a> {
            True(VarString<'a>),
            Default,
        }

        impl<'a> EntityFeetEyes<'a> {
            pub fn discriminant(&self) -> bool {
                match self {
                    EntityFeetEyes::True(_) => true,
                    EntityFeetEyes::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            entity_id: EntityId,
            entity_feet_eyes: EntityFeetEyes<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for PacketFacePlayer<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.feet_eyes, w)?;
                let w = f64::serialize(&self.x, w)?;
                let w = f64::serialize(&self.y, w)?;
                let w = f64::serialize(&self.z, w)?;
                let w = bool::serialize(&self.is_entity, w)?;
                let w = EntityId::serialize(&self.entity_id, w)?;
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
                        "true" => map(VarInt::deserialize, EntityId::True)(input),
                        _ => Ok((input, EntityId::Default)),
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketNbtQueryResponse {
            transaction_id: VarInt,
            nbt: OptionalNbt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketChat<'a> {
            message: VarString<'a>,
            position: i8,
            sender: Uuid,
        }
        pub struct ChunkCoordinates {
            x: i32,
            z: i32,
            y: i32,
        }

        impl<'t> Packet<'t> for ChunkCoordinates {
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketMultiBlockChange {
            chunk_coordinates: ChunkCoordinates,
            not_trust_edges: bool,
            records: VarArray<VarLong>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCloseWindow {
            window_id: u8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketOpenWindow<'a> {
            window_id: VarInt,
            inventory_type: VarInt,
            window_title: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketWindowItems {
            window_id: u8,
            state_id: VarInt,
            items: VarArray<Slot>,
            carried_item: Slot,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCraftProgressBar {
            window_id: u8,
            property: i16,
            value: i16,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetSlot {
            window_id: i8,
            state_id: VarInt,
            slot: i16,
            item: Slot,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetCooldown {
            item_id: VarInt,
            cooldown_ticks: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketNamedSoundEffect<'a> {
            sound_name: VarString<'a>,
            sound_category: VarInt,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketKickDisconnect<'a> {
            reason: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityStatus {
            entity_id: i32,
            entity_status: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct AffectedBlockOffset {
            x: i8,
            y: i8,
            z: i8,
        }
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketUnloadChunk {
            chunk_x: i32,
            chunk_z: i32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketGameStateChange {
            reason: u8,
            game_mode: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketOpenHorseWindow {
            window_id: u8,
            nb_slots: VarInt,
            entity_id: i32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketWorldEvent {
            effect_id: i32,
            location: Position,
            data: i32,
            global: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct WorldParticlesData2 {
            block_state: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct WorldParticlesData3 {
            block_state: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct WorldParticlesData14 {
            red: f32,
            green: f32,
            blue: f32,
            scale: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct WorldParticlesData15 {
            from_red: f32,
            from_green: f32,
            from_blue: f32,
            scale: f32,
            to_red: f32,
            to_green: f32,
            to_blue: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct WorldParticlesData24 {
            block_state: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct WorldParticlesData35 {
            item: Slot,
        }
        pub enum WorldParticlesData36Destination {
            MinecraftBlock(Position),
            Entity(VarInt),
            Default,
        }

        impl WorldParticlesData36Destination {
            pub fn discriminant(&self) -> &str {
                match self {
                    WorldParticlesData36Destination::MinecraftBlock(_) => "minecraft:block",
                    WorldParticlesData36Destination::Entity(_) => "minecraft:entity",
                    WorldParticlesData36Destination::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
        impl<'t: 'a, 'a> Packet<'t> for WorldParticlesData36<'a> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    WorldParticlesData::WorldParticlesData2(_) => 2,
                    WorldParticlesData::WorldParticlesData3(_) => 3,
                    WorldParticlesData::WorldParticlesData14(_) => 14,
                    WorldParticlesData::WorldParticlesData15(_) => 15,
                    WorldParticlesData::WorldParticlesData24(_) => 24,
                    WorldParticlesData::WorldParticlesData35(_) => 35,
                    WorldParticlesData::WorldParticlesData36(_) => 36,
                    WorldParticlesData::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
        impl<'t: 'a, 'a> Packet<'t> for PacketWorldParticles<'a> {
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
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct Ident4<'a> {
            r_type: VarInt,
            x: i8,
            z: i8,
            direction: u8,
            display_name: Option<VarString<'a>>,
        }
        pub enum Rows {
            Rows0,
            Default(u8),
        }

        impl Rows {
            pub fn discriminant(&self) -> u8 {
                match self {
                    Rows::Rows0 => 0,
                    Rows::Default(_) => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Rows::Rows0 => w,
                    Rows::Default(val) => u8::serialize(&val, w)?,
                };

                Ok(w)
            }
        }
        pub enum X {
            X0,
            Default(u8),
        }

        impl X {
            pub fn discriminant(&self) -> u8 {
                match self {
                    X::X0 => 0,
                    X::Default(_) => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    X::X0 => w,
                    X::Default(val) => u8::serialize(&val, w)?,
                };

                Ok(w)
            }
        }
        pub enum Y {
            Y0,
            Default(u8),
        }

        impl Y {
            pub fn discriminant(&self) -> u8 {
                match self {
                    Y::Y0 => 0,
                    Y::Default(_) => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Y::Y0 => w,
                    Y::Default(val) => u8::serialize(&val, w)?,
                };

                Ok(w)
            }
        }
        pub enum MapData<'a> {
            MapData0,
            Default(VarBuffer<'a>),
        }

        impl<'a> MapData<'a> {
            pub fn discriminant(&self) -> u8 {
                match self {
                    MapData::MapData0 => 0,
                    MapData::Default(_) => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    MapData::MapData0 => w,
                    MapData::Default(val) => PrefixedBuffer::<'a, VarInt>::serialize(&val, w)?,
                };

                Ok(w)
            }
        }

        pub struct PacketMap<'a> {
            item_damage: VarInt,
            scale: i8,
            locked: bool,
            icons: Option<PrefixedArray<Ident4<'a>, VarInt>>,
            columns: u8,
            rows: Rows,
            x: X,
            y: Y,
            data: MapData<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for PacketMap<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.item_damage, w)?;
                let w = i8::serialize(&self.scale, w)?;
                let w = bool::serialize(&self.locked, w)?;
                let w = Option::<VarArray<Ident4>>::serialize(&self.icons, w)?;
                let w = u8::serialize(&self.columns, w)?;
                let w = Rows::serialize(&self.rows, w)?;
                let w = X::serialize(&self.x, w)?;
                let w = Y::serialize(&self.y, w)?;
                let w = MapData::serialize(&self.data, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_item_damage) = (VarInt::deserialize)(input)?;
                    let (input, self_scale) = (i8::deserialize)(input)?;
                    let (input, self_locked) = (bool::deserialize)(input)?;
                    let (input, self_icons) = (Option::<VarArray<Ident4>>::deserialize)(input)?;
                    let (input, self_columns) = (u8::deserialize)(input)?;
                    let (input, self_rows) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => Ok((input, Rows::Rows0)),
                        _ => map(u8::deserialize, Rows::Default)(input),
                    })(input)?;
                    let (input, self_x) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => Ok((input, X::X0)),
                        _ => map(u8::deserialize, X::Default)(input),
                    })(input)?;
                    let (input, self_y) = (|input| match &format!("{}", self_columns)[..] {
                        "0" => Ok((input, Y::Y0)),
                        _ => map(u8::deserialize, Y::Default)(input),
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
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketTradeList {
            window_id: VarInt,
            trades: PrefixedArray<Trade, u8>,
            villager_level: VarInt,
            experience: VarInt,
            is_regular_villager: bool,
            can_restock: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketRelEntityMove {
            entity_id: VarInt,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityMoveLook {
            entity_id: VarInt,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityLook {
            entity_id: VarInt,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketVehicleMove {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketOpenBook {
            hand: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketOpenSignEntity {
            location: Position,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCraftRecipeResponse<'a> {
            window_id: i8,
            recipe: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketAbilities {
            flags: i8,
            flying_speed: f32,
            walking_speed: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEndCombatEvent {
            duration: VarInt,
            entity_id: i32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEnterCombatEvent {}
        #[derive(protocol_lib::Packet)]
        pub struct PacketDeathCombatEvent<'a> {
            player_id: VarInt,
            entity_id: i32,
            message: VarString<'a>,
        }
        pub enum Name<'a> {
            Name0(VarString<'a>),
            Default,
        }

        impl<'a> Name<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Name::Name0(_) => 0,
                    Name::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Name::Name0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    Name::Default => w,
                };

                Ok(w)
            }
        }
        #[derive(protocol_lib::Packet)]
        pub struct PlayerInfoDataItemProperties0<'a> {
            name: VarString<'a>,
            value: VarString<'a>,
            signature: Option<VarString<'a>>,
        }
        pub enum PlayerInfoDataItemProperties<'a> {
            PlayerInfoDataItemProperties0(PrefixedArray<PlayerInfoDataItemProperties0<'a>, VarInt>),
            Default,
        }

        impl<'a> PlayerInfoDataItemProperties<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    PlayerInfoDataItemProperties::PlayerInfoDataItemProperties0(_) => 0,
                    PlayerInfoDataItemProperties::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    Gamemode::Gamemode0(_) => 0,
                    Gamemode::Gamemode1(_) => 1,
                    Gamemode::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    Ping::Ping0(_) => 0,
                    Ping::Ping2(_) => 2,
                    Ping::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName0(_) => 0,
                    PlayerInfoDataItemDisplayName::PlayerInfoDataItemDisplayName3(_) => 3,
                    PlayerInfoDataItemDisplayName::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            name: Name<'a>,
            properties: PlayerInfoDataItemProperties<'a>,
            gamemode: Gamemode,
            ping: Ping,
            display_name: PlayerInfoDataItemDisplayName<'a>,
        }
        pub struct PacketPlayerInfo<'a> {
            action: VarInt,
            data: VarArray<PlayerInfoDataItem<'a>>,
        }
        impl<'t: 'a, 'a> Packet<'t> for PacketPlayerInfo<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.action, w)?;

                let w = PrefixedArray::<PlayerInfoDataItem, VarInt>::len(&self.data).serialize(w)?;

                let mut w = w;
                let items = self.data.0.iter();
                for i in items {
                    w = {
                        let w = Uuid::serialize(&i.uuid, w)?;
                        let w = Name::serialize(&i.name, w)?;
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
                                        "0" => map(PrefixedString::<'a, VarInt>::deserialize, Name::Name0)(input),
                                        _ => Ok((input, Name::Default)),
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
        #[derive(protocol_lib::Packet)]
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
        pub enum Recipes2<'a> {
            Recipes20(VarStringArray<'a>),
            Default,
        }

        impl<'a> Recipes2<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Recipes2::Recipes20(_) => 0,
                    Recipes2::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
        impl<'t: 'a, 'a> Packet<'t> for PacketUnlockRecipes<'a> {
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityDestroy {
            entity_ids: VarArray<VarInt>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketRemoveEntityEffect {
            entity_id: VarInt,
            effect_id: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketResourcePackSend<'a> {
            url: VarString<'a>,
            hash: VarString<'a>,
            forced: bool,
            prompt_message: Option<VarString<'a>>,
        }
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityHeadRotation {
            entity_id: VarInt,
            head_yaw: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCamera {
            camera_id: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketHeldItemSlot {
            slot: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateViewPosition {
            chunk_x: VarInt,
            chunk_z: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateViewDistance {
            view_distance: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketScoreboardDisplayObjective<'a> {
            position: i8,
            name: VarString<'a>,
        }
        pub struct PacketEntityMetadata<'a> {
            entity_id: VarInt,
            metadata: Vec<EntityMetadata<'a>>,
        }
        impl<'t: 'a, 'a> Packet<'t> for PacketEntityMetadata<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.entity_id, w)?;

                let mut w = w;
                for (index, item) in self.metadata.iter().enumerate() {
                    w = u8::serialize(&if index == self.metadata.len() - 1 { 255 } else { index as u8 }, w)?;
                    w = item.discriminant().serialize(w)?;
                    w = EntityMetadata::serialize(&item, w)?
                }

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_entity_id) = (VarInt::deserialize)(input)?;
                    let (input, self_metadata) = (|mut input| {
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
                    })(input)?;
                    Ok((
                        input,
                        PacketEntityMetadata {
                            entity_id: self_entity_id,
                            metadata: self_metadata,
                        },
                    ))
                })(input)
            }
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketAttachEntity {
            entity_id: i32,
            vehicle_id: i32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityVelocity {
            entity_id: VarInt,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        }
        pub struct PacketEntityEquipment {
            entity_id: VarInt,
            equipments: std::collections::HashMap<i8, Slot>,
        }
        impl<'t> Packet<'t> for PacketEntityEquipment {
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
                (|input| {
                    let (input, self_entity_id) = (VarInt::deserialize)(input)?;
                    let (input, self_equipments) = (|mut input| {
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
                    })(input)?;
                    Ok((
                        input,
                        PacketEntityEquipment {
                            entity_id: self_entity_id,
                            equipments: self_equipments,
                        },
                    ))
                })(input)
            }
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketExperience {
            experience_bar: f32,
            level: VarInt,
            total_experience: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateHealth {
            health: f32,
            food: VarInt,
            food_saturation: f32,
        }
        pub enum DisplayText<'a> {
            DisplayText0(VarString<'a>),
            DisplayText2(VarString<'a>),
            Default,
        }

        impl<'a> DisplayText<'a> {
            pub fn discriminant(&self) -> i8 {
                match self {
                    DisplayText::DisplayText0(_) => 0,
                    DisplayText::DisplayText2(_) => 2,
                    DisplayText::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    DisplayText::DisplayText0(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    DisplayText::DisplayText2(val) => PrefixedString::<'a, VarInt>::serialize(&val, w)?,
                    DisplayText::Default => w,
                };

                Ok(w)
            }
        }
        pub enum RType {
            RType0(VarInt),
            RType2(VarInt),
            Default,
        }

        impl RType {
            pub fn discriminant(&self) -> i8 {
                match self {
                    RType::RType0(_) => 0,
                    RType::RType2(_) => 2,
                    RType::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    RType::RType0(val) => VarInt::serialize(&val, w)?,
                    RType::RType2(val) => VarInt::serialize(&val, w)?,
                    RType::Default => w,
                };

                Ok(w)
            }
        }

        pub struct PacketScoreboardObjective<'a> {
            name: VarString<'a>,
            action: i8,
            display_text: DisplayText<'a>,
            r_type: RType,
        }
        impl<'t: 'a, 'a> Packet<'t> for PacketScoreboardObjective<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = PrefixedString::<'a, VarInt>::serialize(&self.name, w)?;
                let w = i8::serialize(&self.action, w)?;
                let w = DisplayText::serialize(&self.display_text, w)?;
                let w = RType::serialize(&self.r_type, w)?;

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
                        "0" => map(VarInt::deserialize, RType::RType0)(input),
                        "2" => map(VarInt::deserialize, RType::RType2)(input),
                        _ => Ok((input, RType::Default)),
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetPassengers {
            entity_id: VarInt,
            passengers: VarArray<VarInt>,
        }
        pub enum TeamsName<'a> {
            TeamsName0(VarString<'a>),
            TeamsName2(VarString<'a>),
            Default,
        }

        impl<'a> TeamsName<'a> {
            pub fn discriminant(&self) -> i8 {
                match self {
                    TeamsName::TeamsName0(_) => 0,
                    TeamsName::TeamsName2(_) => 2,
                    TeamsName::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    FriendlyFire::FriendlyFire0(_) => 0,
                    FriendlyFire::FriendlyFire2(_) => 2,
                    FriendlyFire::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    NameTagVisibility::NameTagVisibility0(_) => 0,
                    NameTagVisibility::NameTagVisibility2(_) => 2,
                    NameTagVisibility::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    CollisionRule::CollisionRule0(_) => 0,
                    CollisionRule::CollisionRule2(_) => 2,
                    CollisionRule::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    Formatting::Formatting0(_) => 0,
                    Formatting::Formatting2(_) => 2,
                    Formatting::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    Prefix::Prefix0(_) => 0,
                    Prefix::Prefix2(_) => 2,
                    Prefix::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    Suffix::Suffix0(_) => 0,
                    Suffix::Suffix2(_) => 2,
                    Suffix::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    Players::Players0(_) => 0,
                    Players::Players3(_) => 3,
                    Players::Players4(_) => 4,
                    Players::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
        impl<'t: 'a, 'a> Packet<'t> for PacketTeams<'a> {
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
            pub fn discriminant(&self) -> i32 {
                match self {
                    ScoreboardScoreValue::ScoreboardScoreValue1 => 1,
                    ScoreboardScoreValue::Default(_) => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    ScoreboardScoreValue::ScoreboardScoreValue1 => w,
                    ScoreboardScoreValue::Default(val) => VarInt::serialize(&val, w)?,
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
        impl<'t: 'a, 'a> Packet<'t> for PacketScoreboardScore<'a> {
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketSpawnPosition {
            location: Position,
            angle: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateTime {
            age: i64,
            time: i64,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntitySoundEffect {
            sound_id: VarInt,
            sound_category: VarInt,
            entity_id: VarInt,
            volume: f32,
            pitch: f32,
        }
        pub enum Source {
            Source3(VarInt),
            Source1(VarInt),
            Default,
        }

        impl Source {
            pub fn discriminant(&self) -> i8 {
                match self {
                    Source::Source3(_) => 3,
                    Source::Source1(_) => 1,
                    Source::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
            pub fn discriminant(&self) -> i8 {
                match self {
                    Sound::Sound3(_) => 3,
                    Sound::Sound2(_) => 2,
                    Sound::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
        impl<'t: 'a, 'a> Packet<'t> for PacketStopSound<'a> {
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketSoundEffect {
            sound_id: VarInt,
            sound_category: VarInt,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketPlayerlistHeader<'a> {
            header: VarString<'a>,
            footer: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCollect {
            collected_entity_id: VarInt,
            collector_entity_id: VarInt,
            pickup_item_count: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityTeleport {
            entity_id: VarInt,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct Modifier {
            uuid: Uuid,
            amount: f64,
            operation: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct EntityUpdateAttrsProperty<'a> {
            key: VarString<'a>,
            value: f64,
            modifiers: VarArray<Modifier>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityUpdateAttributes<'a> {
            entity_id: VarInt,
            properties: VarArray<EntityUpdateAttrsProperty<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityEffect {
            entity_id: VarInt,
            effect_id: i8,
            amplifier: i8,
            duration: VarInt,
            hide_particles: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSelectAdvancementTab<'a> {
            id: Option<VarString<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct CraftingShapeless<'a> {
            group: VarString<'a>,
            ingredients: VarArray<VarArray<Slot>>,
            result: Slot,
        }
        pub struct CraftingShaped<'a> {
            width: VarInt,
            height: VarInt,
            group: VarString<'a>,
            ingredients: Vec<Vec<VarArray<Slot>>>,
            result: Slot,
        }
        impl<'t: 'a, 'a> Packet<'t> for CraftingShaped<'a> {
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
        #[derive(protocol_lib::Packet)]
        pub struct Stonecutting<'a> {
            group: VarString<'a>,
            ingredient: VarArray<Slot>,
            result: Slot,
        }
        #[derive(protocol_lib::Packet)]
        pub struct Smithing {
            base: VarArray<Slot>,
            addition: VarArray<Slot>,
            result: Slot,
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
            pub fn discriminant(&self) -> &str {
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
                    RecipeData::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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

        pub struct Recipe<'a> {
            r_type: VarString<'a>,
            recipe_id: VarString<'a>,
            data: RecipeData<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for Recipe<'a> {
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
                        Recipe {
                            r_type: self_r_type,
                            recipe_id: self_recipe_id,
                            data: self_data,
                        },
                    ))
                })(input)
            }
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketDeclareRecipes<'a> {
            recipes: VarArray<Recipe<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct TagsTag<'a> {
            tag_type: VarString<'a>,
            tags: VarArray<Tag<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketTags<'a> {
            tags: VarArray<TagsTag<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketAcknowledgePlayerDigging {
            location: Position,
            block: VarInt,
            status: VarInt,
            successful: bool,
        }
        pub enum SculkVibrationSignalDestination {
            Block(Position),
            EntityId(VarInt),
            Default,
        }

        impl SculkVibrationSignalDestination {
            pub fn discriminant(&self) -> &str {
                match self {
                    SculkVibrationSignalDestination::Block(_) => "block",
                    SculkVibrationSignalDestination::EntityId(_) => "entityId",
                    SculkVibrationSignalDestination::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
        impl<'t: 'a, 'a> Packet<'t> for PacketSculkVibrationSignal<'a> {
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketClearTitles {
            reset: bool,
        }
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketActionBar<'a> {
            text: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketWorldBorderCenter {
            x: f64,
            z: f64,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketWorldBorderLerpSize {
            old_diameter: f64,
            new_diameter: f64,
            speed: VarLong,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketWorldBorderSize {
            diameter: f64,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketWorldBorderWarningDelay {
            warning_time: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketWorldBorderWarningReach {
            warning_blocks: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketPing {
            id: i32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetTitleSubtitle<'a> {
            text: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetTitleText<'a> {
            text: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetTitleTime {
            fade_in: i32,
            stay: i32,
            fade_out: i32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSimulationDistance {
            distance: VarInt,
        }
        pub enum Params<'a> {
            SpawnEntity(PacketSpawnEntity),
            SpawnEntityExperienceOrb(PacketSpawnEntityExperienceOrb),
            SpawnEntityLiving(PacketSpawnEntityLiving),
            SpawnEntityPainting(PacketSpawnEntityPainting),
            NamedEntitySpawn(PacketNamedEntitySpawn),
            SculkVibrationSignal(PacketSculkVibrationSignal<'a>),
            Animation(PacketAnimation),
            Statistics(PacketStatistics),
            AcknowledgePlayerDigging(PacketAcknowledgePlayerDigging),
            BlockBreakAnimation(PacketBlockBreakAnimation),
            TileEntityData(PacketTileEntityData),
            BlockAction(PacketBlockAction),
            BlockChange(PacketBlockChange),
            BossBar(PacketBossBar<'a>),
            Difficulty(PacketDifficulty),
            Chat(PacketChat<'a>),
            ClearTitles(PacketClearTitles),
            TabComplete(PacketTabComplete<'a>),
            DeclareCommands(PacketDeclareCommands<'a>),
            CloseWindow(PacketCloseWindow),
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
            InitializeWorldBorder(PacketInitializeWorldBorder),
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
            OpenWindow(PacketOpenWindow<'a>),
            OpenSignEntity(PacketOpenSignEntity),
            Ping(PacketPing),
            CraftRecipeResponse(PacketCraftRecipeResponse<'a>),
            Abilities(PacketAbilities),
            EndCombatEvent(PacketEndCombatEvent),
            EnterCombatEvent(PacketEnterCombatEvent),
            DeathCombatEvent(PacketDeathCombatEvent<'a>),
            PlayerInfo(PacketPlayerInfo<'a>),
            FacePlayer(PacketFacePlayer<'a>),
            Position(PacketPosition),
            UnlockRecipes(PacketUnlockRecipes<'a>),
            EntityDestroy(PacketEntityDestroy),
            RemoveEntityEffect(PacketRemoveEntityEffect),
            ResourcePackSend(PacketResourcePackSend<'a>),
            Respawn(PacketRespawn<'a>),
            EntityHeadRotation(PacketEntityHeadRotation),
            MultiBlockChange(PacketMultiBlockChange),
            SelectAdvancementTab(PacketSelectAdvancementTab<'a>),
            ActionBar(PacketActionBar<'a>),
            WorldBorderCenter(PacketWorldBorderCenter),
            WorldBorderLerpSize(PacketWorldBorderLerpSize),
            WorldBorderSize(PacketWorldBorderSize),
            WorldBorderWarningDelay(PacketWorldBorderWarningDelay),
            WorldBorderWarningReach(PacketWorldBorderWarningReach),
            Camera(PacketCamera),
            HeldItemSlot(PacketHeldItemSlot),
            UpdateViewPosition(PacketUpdateViewPosition),
            UpdateViewDistance(PacketUpdateViewDistance),
            SpawnPosition(PacketSpawnPosition),
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
            SetTitleSubtitle(PacketSetTitleSubtitle<'a>),
            UpdateTime(PacketUpdateTime),
            SetTitleText(PacketSetTitleText<'a>),
            SetTitleTime(PacketSetTitleTime),
            EntitySoundEffect(PacketEntitySoundEffect),
            SoundEffect(PacketSoundEffect),
            StopSound(PacketStopSound<'a>),
            PlayerlistHeader(PacketPlayerlistHeader<'a>),
            NbtQueryResponse(PacketNbtQueryResponse),
            Collect(PacketCollect),
            EntityTeleport(PacketEntityTeleport),
            Advancements(PacketAdvancements<'a>),
            EntityUpdateAttributes(PacketEntityUpdateAttributes<'a>),
            EntityEffect(PacketEntityEffect),
            DeclareRecipes(PacketDeclareRecipes<'a>),
            Tags(PacketTags<'a>),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::SpawnEntity(_) => 0x00,
                    Params::SpawnEntityExperienceOrb(_) => 0x01,
                    Params::SpawnEntityLiving(_) => 0x02,
                    Params::SpawnEntityPainting(_) => 0x03,
                    Params::NamedEntitySpawn(_) => 0x04,
                    Params::SculkVibrationSignal(_) => 0x05,
                    Params::Animation(_) => 0x06,
                    Params::Statistics(_) => 0x07,
                    Params::AcknowledgePlayerDigging(_) => 0x08,
                    Params::BlockBreakAnimation(_) => 0x09,
                    Params::TileEntityData(_) => 0x0a,
                    Params::BlockAction(_) => 0x0b,
                    Params::BlockChange(_) => 0x0c,
                    Params::BossBar(_) => 0x0d,
                    Params::Difficulty(_) => 0x0e,
                    Params::Chat(_) => 0x0f,
                    Params::ClearTitles(_) => 0x10,
                    Params::TabComplete(_) => 0x11,
                    Params::DeclareCommands(_) => 0x12,
                    Params::CloseWindow(_) => 0x13,
                    Params::WindowItems(_) => 0x14,
                    Params::CraftProgressBar(_) => 0x15,
                    Params::SetSlot(_) => 0x16,
                    Params::SetCooldown(_) => 0x17,
                    Params::CustomPayload(_) => 0x18,
                    Params::NamedSoundEffect(_) => 0x19,
                    Params::KickDisconnect(_) => 0x1a,
                    Params::EntityStatus(_) => 0x1b,
                    Params::Explosion(_) => 0x1c,
                    Params::UnloadChunk(_) => 0x1d,
                    Params::GameStateChange(_) => 0x1e,
                    Params::OpenHorseWindow(_) => 0x1f,
                    Params::InitializeWorldBorder(_) => 0x20,
                    Params::KeepAlive(_) => 0x21,
                    Params::MapChunk(_) => 0x22,
                    Params::WorldEvent(_) => 0x23,
                    Params::WorldParticles(_) => 0x24,
                    Params::UpdateLight(_) => 0x25,
                    Params::Login(_) => 0x26,
                    Params::Map(_) => 0x27,
                    Params::TradeList(_) => 0x28,
                    Params::RelEntityMove(_) => 0x29,
                    Params::EntityMoveLook(_) => 0x2a,
                    Params::EntityLook(_) => 0x2b,
                    Params::VehicleMove(_) => 0x2c,
                    Params::OpenBook(_) => 0x2d,
                    Params::OpenWindow(_) => 0x2e,
                    Params::OpenSignEntity(_) => 0x2f,
                    Params::Ping(_) => 0x30,
                    Params::CraftRecipeResponse(_) => 0x31,
                    Params::Abilities(_) => 0x32,
                    Params::EndCombatEvent(_) => 0x33,
                    Params::EnterCombatEvent(_) => 0x34,
                    Params::DeathCombatEvent(_) => 0x35,
                    Params::PlayerInfo(_) => 0x36,
                    Params::FacePlayer(_) => 0x37,
                    Params::Position(_) => 0x38,
                    Params::UnlockRecipes(_) => 0x39,
                    Params::EntityDestroy(_) => 0x3a,
                    Params::RemoveEntityEffect(_) => 0x3b,
                    Params::ResourcePackSend(_) => 0x3c,
                    Params::Respawn(_) => 0x3d,
                    Params::EntityHeadRotation(_) => 0x3e,
                    Params::MultiBlockChange(_) => 0x3f,
                    Params::SelectAdvancementTab(_) => 0x40,
                    Params::ActionBar(_) => 0x41,
                    Params::WorldBorderCenter(_) => 0x42,
                    Params::WorldBorderLerpSize(_) => 0x43,
                    Params::WorldBorderSize(_) => 0x44,
                    Params::WorldBorderWarningDelay(_) => 0x45,
                    Params::WorldBorderWarningReach(_) => 0x46,
                    Params::Camera(_) => 0x47,
                    Params::HeldItemSlot(_) => 0x48,
                    Params::UpdateViewPosition(_) => 0x49,
                    Params::UpdateViewDistance(_) => 0x4a,
                    Params::SpawnPosition(_) => 0x4b,
                    Params::ScoreboardDisplayObjective(_) => 0x4c,
                    Params::EntityMetadata(_) => 0x4d,
                    Params::AttachEntity(_) => 0x4e,
                    Params::EntityVelocity(_) => 0x4f,
                    Params::EntityEquipment(_) => 0x50,
                    Params::Experience(_) => 0x51,
                    Params::UpdateHealth(_) => 0x52,
                    Params::ScoreboardObjective(_) => 0x53,
                    Params::SetPassengers(_) => 0x54,
                    Params::Teams(_) => 0x55,
                    Params::ScoreboardScore(_) => 0x56,
                    Params::SimulationDistance(_) => 0x57,
                    Params::SetTitleSubtitle(_) => 0x58,
                    Params::UpdateTime(_) => 0x59,
                    Params::SetTitleText(_) => 0x5a,
                    Params::SetTitleTime(_) => 0x5b,
                    Params::EntitySoundEffect(_) => 0x5c,
                    Params::SoundEffect(_) => 0x5d,
                    Params::StopSound(_) => 0x5e,
                    Params::PlayerlistHeader(_) => 0x5f,
                    Params::NbtQueryResponse(_) => 0x60,
                    Params::Collect(_) => 0x61,
                    Params::EntityTeleport(_) => 0x62,
                    Params::Advancements(_) => 0x63,
                    Params::EntityUpdateAttributes(_) => 0x64,
                    Params::EntityEffect(_) => 0x65,
                    Params::DeclareRecipes(_) => 0x66,
                    Params::Tags(_) => 0x67,
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Params::SpawnEntity(val) => PacketSpawnEntity::serialize(&val, w)?,
                    Params::SpawnEntityExperienceOrb(val) => PacketSpawnEntityExperienceOrb::serialize(&val, w)?,
                    Params::SpawnEntityLiving(val) => PacketSpawnEntityLiving::serialize(&val, w)?,
                    Params::SpawnEntityPainting(val) => PacketSpawnEntityPainting::serialize(&val, w)?,
                    Params::NamedEntitySpawn(val) => PacketNamedEntitySpawn::serialize(&val, w)?,
                    Params::SculkVibrationSignal(val) => PacketSculkVibrationSignal::serialize(&val, w)?,
                    Params::Animation(val) => PacketAnimation::serialize(&val, w)?,
                    Params::Statistics(val) => PacketStatistics::serialize(&val, w)?,
                    Params::AcknowledgePlayerDigging(val) => PacketAcknowledgePlayerDigging::serialize(&val, w)?,
                    Params::BlockBreakAnimation(val) => PacketBlockBreakAnimation::serialize(&val, w)?,
                    Params::TileEntityData(val) => PacketTileEntityData::serialize(&val, w)?,
                    Params::BlockAction(val) => PacketBlockAction::serialize(&val, w)?,
                    Params::BlockChange(val) => PacketBlockChange::serialize(&val, w)?,
                    Params::BossBar(val) => PacketBossBar::serialize(&val, w)?,
                    Params::Difficulty(val) => PacketDifficulty::serialize(&val, w)?,
                    Params::Chat(val) => PacketChat::serialize(&val, w)?,
                    Params::ClearTitles(val) => PacketClearTitles::serialize(&val, w)?,
                    Params::TabComplete(val) => PacketTabComplete::serialize(&val, w)?,
                    Params::DeclareCommands(val) => PacketDeclareCommands::serialize(&val, w)?,
                    Params::CloseWindow(val) => PacketCloseWindow::serialize(&val, w)?,
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
                    Params::InitializeWorldBorder(val) => PacketInitializeWorldBorder::serialize(&val, w)?,
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
                    Params::OpenWindow(val) => PacketOpenWindow::serialize(&val, w)?,
                    Params::OpenSignEntity(val) => PacketOpenSignEntity::serialize(&val, w)?,
                    Params::Ping(val) => PacketPing::serialize(&val, w)?,
                    Params::CraftRecipeResponse(val) => PacketCraftRecipeResponse::serialize(&val, w)?,
                    Params::Abilities(val) => PacketAbilities::serialize(&val, w)?,
                    Params::EndCombatEvent(val) => PacketEndCombatEvent::serialize(&val, w)?,
                    Params::EnterCombatEvent(val) => PacketEnterCombatEvent::serialize(&val, w)?,
                    Params::DeathCombatEvent(val) => PacketDeathCombatEvent::serialize(&val, w)?,
                    Params::PlayerInfo(val) => PacketPlayerInfo::serialize(&val, w)?,
                    Params::FacePlayer(val) => PacketFacePlayer::serialize(&val, w)?,
                    Params::Position(val) => PacketPosition::serialize(&val, w)?,
                    Params::UnlockRecipes(val) => PacketUnlockRecipes::serialize(&val, w)?,
                    Params::EntityDestroy(val) => PacketEntityDestroy::serialize(&val, w)?,
                    Params::RemoveEntityEffect(val) => PacketRemoveEntityEffect::serialize(&val, w)?,
                    Params::ResourcePackSend(val) => PacketResourcePackSend::serialize(&val, w)?,
                    Params::Respawn(val) => PacketRespawn::serialize(&val, w)?,
                    Params::EntityHeadRotation(val) => PacketEntityHeadRotation::serialize(&val, w)?,
                    Params::MultiBlockChange(val) => PacketMultiBlockChange::serialize(&val, w)?,
                    Params::SelectAdvancementTab(val) => PacketSelectAdvancementTab::serialize(&val, w)?,
                    Params::ActionBar(val) => PacketActionBar::serialize(&val, w)?,
                    Params::WorldBorderCenter(val) => PacketWorldBorderCenter::serialize(&val, w)?,
                    Params::WorldBorderLerpSize(val) => PacketWorldBorderLerpSize::serialize(&val, w)?,
                    Params::WorldBorderSize(val) => PacketWorldBorderSize::serialize(&val, w)?,
                    Params::WorldBorderWarningDelay(val) => PacketWorldBorderWarningDelay::serialize(&val, w)?,
                    Params::WorldBorderWarningReach(val) => PacketWorldBorderWarningReach::serialize(&val, w)?,
                    Params::Camera(val) => PacketCamera::serialize(&val, w)?,
                    Params::HeldItemSlot(val) => PacketHeldItemSlot::serialize(&val, w)?,
                    Params::UpdateViewPosition(val) => PacketUpdateViewPosition::serialize(&val, w)?,
                    Params::UpdateViewDistance(val) => PacketUpdateViewDistance::serialize(&val, w)?,
                    Params::SpawnPosition(val) => PacketSpawnPosition::serialize(&val, w)?,
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
                    Params::SetTitleSubtitle(val) => PacketSetTitleSubtitle::serialize(&val, w)?,
                    Params::UpdateTime(val) => PacketUpdateTime::serialize(&val, w)?,
                    Params::SetTitleText(val) => PacketSetTitleText::serialize(&val, w)?,
                    Params::SetTitleTime(val) => PacketSetTitleTime::serialize(&val, w)?,
                    Params::EntitySoundEffect(val) => PacketEntitySoundEffect::serialize(&val, w)?,
                    Params::SoundEffect(val) => PacketSoundEffect::serialize(&val, w)?,
                    Params::StopSound(val) => PacketStopSound::serialize(&val, w)?,
                    Params::PlayerlistHeader(val) => PacketPlayerlistHeader::serialize(&val, w)?,
                    Params::NbtQueryResponse(val) => PacketNbtQueryResponse::serialize(&val, w)?,
                    Params::Collect(val) => PacketCollect::serialize(&val, w)?,
                    Params::EntityTeleport(val) => PacketEntityTeleport::serialize(&val, w)?,
                    Params::Advancements(val) => PacketAdvancements::serialize(&val, w)?,
                    Params::EntityUpdateAttributes(val) => PacketEntityUpdateAttributes::serialize(&val, w)?,
                    Params::EntityEffect(val) => PacketEntityEffect::serialize(&val, w)?,
                    Params::DeclareRecipes(val) => PacketDeclareRecipes::serialize(&val, w)?,
                    Params::Tags(val) => PacketTags::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }

        pub struct ToClient<'a> {
            name: VarInt,
            params: Params<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for ToClient<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "0x00" => map(PacketSpawnEntity::deserialize, Params::SpawnEntity)(input),
                        "0x01" => map(PacketSpawnEntityExperienceOrb::deserialize, Params::SpawnEntityExperienceOrb)(input),
                        "0x02" => map(PacketSpawnEntityLiving::deserialize, Params::SpawnEntityLiving)(input),
                        "0x03" => map(PacketSpawnEntityPainting::deserialize, Params::SpawnEntityPainting)(input),
                        "0x04" => map(PacketNamedEntitySpawn::deserialize, Params::NamedEntitySpawn)(input),
                        "0x05" => map(PacketSculkVibrationSignal::deserialize, Params::SculkVibrationSignal)(input),
                        "0x06" => map(PacketAnimation::deserialize, Params::Animation)(input),
                        "0x07" => map(PacketStatistics::deserialize, Params::Statistics)(input),
                        "0x08" => map(PacketAcknowledgePlayerDigging::deserialize, Params::AcknowledgePlayerDigging)(input),
                        "0x09" => map(PacketBlockBreakAnimation::deserialize, Params::BlockBreakAnimation)(input),
                        "0x0a" => map(PacketTileEntityData::deserialize, Params::TileEntityData)(input),
                        "0x0b" => map(PacketBlockAction::deserialize, Params::BlockAction)(input),
                        "0x0c" => map(PacketBlockChange::deserialize, Params::BlockChange)(input),
                        "0x0d" => map(PacketBossBar::deserialize, Params::BossBar)(input),
                        "0x0e" => map(PacketDifficulty::deserialize, Params::Difficulty)(input),
                        "0x0f" => map(PacketChat::deserialize, Params::Chat)(input),
                        "0x10" => map(PacketClearTitles::deserialize, Params::ClearTitles)(input),
                        "0x11" => map(PacketTabComplete::deserialize, Params::TabComplete)(input),
                        "0x12" => map(PacketDeclareCommands::deserialize, Params::DeclareCommands)(input),
                        "0x13" => map(PacketCloseWindow::deserialize, Params::CloseWindow)(input),
                        "0x14" => map(PacketWindowItems::deserialize, Params::WindowItems)(input),
                        "0x15" => map(PacketCraftProgressBar::deserialize, Params::CraftProgressBar)(input),
                        "0x16" => map(PacketSetSlot::deserialize, Params::SetSlot)(input),
                        "0x17" => map(PacketSetCooldown::deserialize, Params::SetCooldown)(input),
                        "0x18" => map(PacketCustomPayload::deserialize, Params::CustomPayload)(input),
                        "0x19" => map(PacketNamedSoundEffect::deserialize, Params::NamedSoundEffect)(input),
                        "0x1a" => map(PacketKickDisconnect::deserialize, Params::KickDisconnect)(input),
                        "0x1b" => map(PacketEntityStatus::deserialize, Params::EntityStatus)(input),
                        "0x1c" => map(PacketExplosion::deserialize, Params::Explosion)(input),
                        "0x1d" => map(PacketUnloadChunk::deserialize, Params::UnloadChunk)(input),
                        "0x1e" => map(PacketGameStateChange::deserialize, Params::GameStateChange)(input),
                        "0x1f" => map(PacketOpenHorseWindow::deserialize, Params::OpenHorseWindow)(input),
                        "0x20" => map(PacketInitializeWorldBorder::deserialize, Params::InitializeWorldBorder)(input),
                        "0x21" => map(PacketKeepAlive::deserialize, Params::KeepAlive)(input),
                        "0x22" => map(PacketMapChunk::deserialize, Params::MapChunk)(input),
                        "0x23" => map(PacketWorldEvent::deserialize, Params::WorldEvent)(input),
                        "0x24" => map(PacketWorldParticles::deserialize, Params::WorldParticles)(input),
                        "0x25" => map(PacketUpdateLight::deserialize, Params::UpdateLight)(input),
                        "0x26" => map(PacketLogin::deserialize, Params::Login)(input),
                        "0x27" => map(PacketMap::deserialize, Params::Map)(input),
                        "0x28" => map(PacketTradeList::deserialize, Params::TradeList)(input),
                        "0x29" => map(PacketRelEntityMove::deserialize, Params::RelEntityMove)(input),
                        "0x2a" => map(PacketEntityMoveLook::deserialize, Params::EntityMoveLook)(input),
                        "0x2b" => map(PacketEntityLook::deserialize, Params::EntityLook)(input),
                        "0x2c" => map(PacketVehicleMove::deserialize, Params::VehicleMove)(input),
                        "0x2d" => map(PacketOpenBook::deserialize, Params::OpenBook)(input),
                        "0x2e" => map(PacketOpenWindow::deserialize, Params::OpenWindow)(input),
                        "0x2f" => map(PacketOpenSignEntity::deserialize, Params::OpenSignEntity)(input),
                        "0x30" => map(PacketPing::deserialize, Params::Ping)(input),
                        "0x31" => map(PacketCraftRecipeResponse::deserialize, Params::CraftRecipeResponse)(input),
                        "0x32" => map(PacketAbilities::deserialize, Params::Abilities)(input),
                        "0x33" => map(PacketEndCombatEvent::deserialize, Params::EndCombatEvent)(input),
                        "0x34" => map(PacketEnterCombatEvent::deserialize, Params::EnterCombatEvent)(input),
                        "0x35" => map(PacketDeathCombatEvent::deserialize, Params::DeathCombatEvent)(input),
                        "0x36" => map(PacketPlayerInfo::deserialize, Params::PlayerInfo)(input),
                        "0x37" => map(PacketFacePlayer::deserialize, Params::FacePlayer)(input),
                        "0x38" => map(PacketPosition::deserialize, Params::Position)(input),
                        "0x39" => map(PacketUnlockRecipes::deserialize, Params::UnlockRecipes)(input),
                        "0x3a" => map(PacketEntityDestroy::deserialize, Params::EntityDestroy)(input),
                        "0x3b" => map(PacketRemoveEntityEffect::deserialize, Params::RemoveEntityEffect)(input),
                        "0x3c" => map(PacketResourcePackSend::deserialize, Params::ResourcePackSend)(input),
                        "0x3d" => map(PacketRespawn::deserialize, Params::Respawn)(input),
                        "0x3e" => map(PacketEntityHeadRotation::deserialize, Params::EntityHeadRotation)(input),
                        "0x3f" => map(PacketMultiBlockChange::deserialize, Params::MultiBlockChange)(input),
                        "0x40" => map(PacketSelectAdvancementTab::deserialize, Params::SelectAdvancementTab)(input),
                        "0x41" => map(PacketActionBar::deserialize, Params::ActionBar)(input),
                        "0x42" => map(PacketWorldBorderCenter::deserialize, Params::WorldBorderCenter)(input),
                        "0x43" => map(PacketWorldBorderLerpSize::deserialize, Params::WorldBorderLerpSize)(input),
                        "0x44" => map(PacketWorldBorderSize::deserialize, Params::WorldBorderSize)(input),
                        "0x45" => map(PacketWorldBorderWarningDelay::deserialize, Params::WorldBorderWarningDelay)(input),
                        "0x46" => map(PacketWorldBorderWarningReach::deserialize, Params::WorldBorderWarningReach)(input),
                        "0x47" => map(PacketCamera::deserialize, Params::Camera)(input),
                        "0x48" => map(PacketHeldItemSlot::deserialize, Params::HeldItemSlot)(input),
                        "0x49" => map(PacketUpdateViewPosition::deserialize, Params::UpdateViewPosition)(input),
                        "0x4a" => map(PacketUpdateViewDistance::deserialize, Params::UpdateViewDistance)(input),
                        "0x4b" => map(PacketSpawnPosition::deserialize, Params::SpawnPosition)(input),
                        "0x4c" => map(PacketScoreboardDisplayObjective::deserialize, Params::ScoreboardDisplayObjective)(input),
                        "0x4d" => map(PacketEntityMetadata::deserialize, Params::EntityMetadata)(input),
                        "0x4e" => map(PacketAttachEntity::deserialize, Params::AttachEntity)(input),
                        "0x4f" => map(PacketEntityVelocity::deserialize, Params::EntityVelocity)(input),
                        "0x50" => map(PacketEntityEquipment::deserialize, Params::EntityEquipment)(input),
                        "0x51" => map(PacketExperience::deserialize, Params::Experience)(input),
                        "0x52" => map(PacketUpdateHealth::deserialize, Params::UpdateHealth)(input),
                        "0x53" => map(PacketScoreboardObjective::deserialize, Params::ScoreboardObjective)(input),
                        "0x54" => map(PacketSetPassengers::deserialize, Params::SetPassengers)(input),
                        "0x55" => map(PacketTeams::deserialize, Params::Teams)(input),
                        "0x56" => map(PacketScoreboardScore::deserialize, Params::ScoreboardScore)(input),
                        "0x57" => map(PacketSimulationDistance::deserialize, Params::SimulationDistance)(input),
                        "0x58" => map(PacketSetTitleSubtitle::deserialize, Params::SetTitleSubtitle)(input),
                        "0x59" => map(PacketUpdateTime::deserialize, Params::UpdateTime)(input),
                        "0x5a" => map(PacketSetTitleText::deserialize, Params::SetTitleText)(input),
                        "0x5b" => map(PacketSetTitleTime::deserialize, Params::SetTitleTime)(input),
                        "0x5c" => map(PacketEntitySoundEffect::deserialize, Params::EntitySoundEffect)(input),
                        "0x5d" => map(PacketSoundEffect::deserialize, Params::SoundEffect)(input),
                        "0x5e" => map(PacketStopSound::deserialize, Params::StopSound)(input),
                        "0x5f" => map(PacketPlayerlistHeader::deserialize, Params::PlayerlistHeader)(input),
                        "0x60" => map(PacketNbtQueryResponse::deserialize, Params::NbtQueryResponse)(input),
                        "0x61" => map(PacketCollect::deserialize, Params::Collect)(input),
                        "0x62" => map(PacketEntityTeleport::deserialize, Params::EntityTeleport)(input),
                        "0x63" => map(PacketAdvancements::deserialize, Params::Advancements)(input),
                        "0x64" => map(PacketEntityUpdateAttributes::deserialize, Params::EntityUpdateAttributes)(input),
                        "0x65" => map(PacketEntityEffect::deserialize, Params::EntityEffect)(input),
                        "0x66" => map(PacketDeclareRecipes::deserialize, Params::DeclareRecipes)(input),
                        "0x67" => map(PacketTags::deserialize, Params::Tags)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToClient { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
    pub mod serverbound {
        use crate::test::*;
        use nom::{combinator::map, sequence::tuple};
        use protocol_lib::Packet;
        #[derive(protocol_lib::Packet)]
        pub struct PacketTeleportConfirm {
            teleport_id: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketQueryBlockNbt {
            transaction_id: VarInt,
            location: Position,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetDifficulty {
            new_difficulty: u8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEditBook<'a> {
            hand: VarInt,
            pages: VarStringArray<'a>,
            title: Option<VarString<'a>>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketQueryEntityNbt {
            transaction_id: VarInt,
            entity_id: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketPickItem {
            slot: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketNameItem<'a> {
            name: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSelectTrade {
            slot: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetBeaconEffect {
            primary_effect: VarInt,
            secondary_effect: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateCommandBlock<'a> {
            location: Position,
            command: VarString<'a>,
            mode: VarInt,
            flags: u8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateCommandBlockMinecart<'a> {
            entity_id: VarInt,
            command: VarString<'a>,
            track_output: bool,
        }
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketTabComplete<'a> {
            transaction_id: VarInt,
            text: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketChat<'a> {
            message: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketClientCommand {
            action_id: VarInt,
        }
        #[derive(protocol_lib::Packet)]
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketEnchantItem {
            window_id: i8,
            enchantment: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct ChangedSlot {
            location: i16,
            item: Slot,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketWindowClick {
            window_id: u8,
            state_id: VarInt,
            slot: i16,
            mouse_button: i8,
            mode: VarInt,
            changed_slots: VarArray<ChangedSlot>,
            cursor_item: Slot,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCloseWindow {
            window_id: u8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }
        pub enum X {
            X2(f32),
            Default,
        }

        impl X {
            pub fn discriminant(&self) -> i32 {
                match self {
                    X::X2(_) => 2,
                    X::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    X::X2(val) => f32::serialize(&val, w)?,
                    X::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Y {
            Y2(f32),
            Default,
        }

        impl Y {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Y::Y2(_) => 2,
                    Y::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Y::Y2(val) => f32::serialize(&val, w)?,
                    Y::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Z {
            Z2(f32),
            Default,
        }

        impl Z {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Z::Z2(_) => 2,
                    Z::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Z::Z2(val) => f32::serialize(&val, w)?,
                    Z::Default => w,
                };

                Ok(w)
            }
        }
        pub enum Hand {
            Hand0(VarInt),
            Hand2(VarInt),
            Default,
        }

        impl Hand {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Hand::Hand0(_) => 0,
                    Hand::Hand2(_) => 2,
                    Hand::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Hand::Hand0(val) => VarInt::serialize(&val, w)?,
                    Hand::Hand2(val) => VarInt::serialize(&val, w)?,
                    Hand::Default => w,
                };

                Ok(w)
            }
        }

        pub struct PacketUseEntity {
            target: VarInt,
            mouse: VarInt,
            x: X,
            y: Y,
            z: Z,
            hand: Hand,
            sneaking: bool,
        }
        impl<'t> Packet<'t> for PacketUseEntity {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.target, w)?;
                let w = VarInt::serialize(&self.mouse, w)?;
                let w = X::serialize(&self.x, w)?;
                let w = Y::serialize(&self.y, w)?;
                let w = Z::serialize(&self.z, w)?;
                let w = Hand::serialize(&self.hand, w)?;
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
                        "2" => map(f32::deserialize, Y::Y2)(input),
                        _ => Ok((input, Y::Default)),
                    })(input)?;
                    let (input, self_z) = (|input| match &format!("{}", self_mouse)[..] {
                        "2" => map(f32::deserialize, Z::Z2)(input),
                        _ => Ok((input, Z::Default)),
                    })(input)?;
                    let (input, self_hand) = (|input| match &format!("{}", self_mouse)[..] {
                        "0" => map(VarInt::deserialize, Hand::Hand0)(input),
                        "2" => map(VarInt::deserialize, Hand::Hand2)(input),
                        _ => Ok((input, Hand::Default)),
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketGenerateStructure {
            location: Position,
            levels: VarInt,
            keep_jigsaws: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketLockDifficulty {
            locked: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketPosition {
            x: f64,
            y: f64,
            z: f64,
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketPositionLook {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketLook {
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketFlying {
            on_ground: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketVehicleMove {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSteerBoat {
            left_paddle: bool,
            right_paddle: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketCraftRecipeRequest<'a> {
            window_id: i8,
            recipe: VarString<'a>,
            make_all: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketAbilities {
            flags: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketBlockDig {
            status: VarInt,
            location: Position,
            face: i8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketEntityAction {
            entity_id: VarInt,
            action_id: VarInt,
            jump_boost: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSteerVehicle {
            sideways: f32,
            forward: f32,
            jump: u8,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketDisplayedRecipe<'a> {
            recipe_id: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketRecipeBook {
            book_id: VarInt,
            book_open: bool,
            filter_active: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketResourcePackReceive {
            result: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketHeldItemSlot {
            slot_id: i16,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSetCreativeSlot {
            slot: i16,
            item: Slot,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateJigsawBlock<'a> {
            location: Position,
            name: VarString<'a>,
            target: VarString<'a>,
            pool: VarString<'a>,
            final_state: VarString<'a>,
            joint_type: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUpdateSign<'a> {
            location: Position,
            text1: VarString<'a>,
            text2: VarString<'a>,
            text3: VarString<'a>,
            text4: VarString<'a>,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketArmAnimation {
            hand: VarInt,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketSpectate {
            target: Uuid,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketBlockPlace {
            hand: VarInt,
            location: Position,
            direction: VarInt,
            cursor_x: f32,
            cursor_y: f32,
            cursor_z: f32,
            inside_block: bool,
        }
        #[derive(protocol_lib::Packet)]
        pub struct PacketUseItem {
            hand: VarInt,
        }
        pub enum TabId<'a> {
            TabId0(VarString<'a>),
            TabId1,
            Default,
        }

        impl<'a> TabId<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    TabId::TabId0(_) => 0,
                    TabId::TabId1 => 1,
                    TabId::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
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
        impl<'t: 'a, 'a> Packet<'t> for PacketAdvancementTab<'a> {
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
        #[derive(protocol_lib::Packet)]
        pub struct PacketPong {
            id: i32,
        }
        pub enum Params<'a> {
            TeleportConfirm(PacketTeleportConfirm),
            QueryBlockNbt(PacketQueryBlockNbt),
            SetDifficulty(PacketSetDifficulty),
            Chat(PacketChat<'a>),
            ClientCommand(PacketClientCommand),
            Settings(PacketSettings<'a>),
            TabComplete(PacketTabComplete<'a>),
            EnchantItem(PacketEnchantItem),
            WindowClick(PacketWindowClick),
            CloseWindow(PacketCloseWindow),
            CustomPayload(PacketCustomPayload<'a>),
            EditBook(PacketEditBook<'a>),
            QueryEntityNbt(PacketQueryEntityNbt),
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
            PickItem(PacketPickItem),
            CraftRecipeRequest(PacketCraftRecipeRequest<'a>),
            Abilities(PacketAbilities),
            BlockDig(PacketBlockDig),
            EntityAction(PacketEntityAction),
            SteerVehicle(PacketSteerVehicle),
            Pong(PacketPong),
            RecipeBook(PacketRecipeBook),
            DisplayedRecipe(PacketDisplayedRecipe<'a>),
            NameItem(PacketNameItem<'a>),
            ResourcePackReceive(PacketResourcePackReceive),
            AdvancementTab(PacketAdvancementTab<'a>),
            SelectTrade(PacketSelectTrade),
            SetBeaconEffect(PacketSetBeaconEffect),
            HeldItemSlot(PacketHeldItemSlot),
            UpdateCommandBlock(PacketUpdateCommandBlock<'a>),
            UpdateCommandBlockMinecart(PacketUpdateCommandBlockMinecart<'a>),
            SetCreativeSlot(PacketSetCreativeSlot),
            UpdateJigsawBlock(PacketUpdateJigsawBlock<'a>),
            UpdateStructureBlock(PacketUpdateStructureBlock<'a>),
            UpdateSign(PacketUpdateSign<'a>),
            ArmAnimation(PacketArmAnimation),
            Spectate(PacketSpectate),
            BlockPlace(PacketBlockPlace),
            UseItem(PacketUseItem),
            Default,
        }

        impl<'a> Params<'a> {
            pub fn discriminant(&self) -> i32 {
                match self {
                    Params::TeleportConfirm(_) => 0x00,
                    Params::QueryBlockNbt(_) => 0x01,
                    Params::SetDifficulty(_) => 0x02,
                    Params::Chat(_) => 0x03,
                    Params::ClientCommand(_) => 0x04,
                    Params::Settings(_) => 0x05,
                    Params::TabComplete(_) => 0x06,
                    Params::EnchantItem(_) => 0x07,
                    Params::WindowClick(_) => 0x08,
                    Params::CloseWindow(_) => 0x09,
                    Params::CustomPayload(_) => 0x0a,
                    Params::EditBook(_) => 0x0b,
                    Params::QueryEntityNbt(_) => 0x0c,
                    Params::UseEntity(_) => 0x0d,
                    Params::GenerateStructure(_) => 0x0e,
                    Params::KeepAlive(_) => 0x0f,
                    Params::LockDifficulty(_) => 0x10,
                    Params::Position(_) => 0x11,
                    Params::PositionLook(_) => 0x12,
                    Params::Look(_) => 0x13,
                    Params::Flying(_) => 0x14,
                    Params::VehicleMove(_) => 0x15,
                    Params::SteerBoat(_) => 0x16,
                    Params::PickItem(_) => 0x17,
                    Params::CraftRecipeRequest(_) => 0x18,
                    Params::Abilities(_) => 0x19,
                    Params::BlockDig(_) => 0x1a,
                    Params::EntityAction(_) => 0x1b,
                    Params::SteerVehicle(_) => 0x1c,
                    Params::Pong(_) => 0x1d,
                    Params::RecipeBook(_) => 0x1e,
                    Params::DisplayedRecipe(_) => 0x1f,
                    Params::NameItem(_) => 0x20,
                    Params::ResourcePackReceive(_) => 0x21,
                    Params::AdvancementTab(_) => 0x22,
                    Params::SelectTrade(_) => 0x23,
                    Params::SetBeaconEffect(_) => 0x24,
                    Params::HeldItemSlot(_) => 0x25,
                    Params::UpdateCommandBlock(_) => 0x26,
                    Params::UpdateCommandBlockMinecart(_) => 0x27,
                    Params::SetCreativeSlot(_) => 0x28,
                    Params::UpdateJigsawBlock(_) => 0x29,
                    Params::UpdateStructureBlock(_) => 0x2a,
                    Params::UpdateSign(_) => 0x2b,
                    Params::ArmAnimation(_) => 0x2c,
                    Params::Spectate(_) => 0x2d,
                    Params::BlockPlace(_) => 0x2e,
                    Params::UseItem(_) => 0x2f,
                    Params::Default => todo!(),
                }
            }
            pub fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = match &self {
                    Params::TeleportConfirm(val) => PacketTeleportConfirm::serialize(&val, w)?,
                    Params::QueryBlockNbt(val) => PacketQueryBlockNbt::serialize(&val, w)?,
                    Params::SetDifficulty(val) => PacketSetDifficulty::serialize(&val, w)?,
                    Params::Chat(val) => PacketChat::serialize(&val, w)?,
                    Params::ClientCommand(val) => PacketClientCommand::serialize(&val, w)?,
                    Params::Settings(val) => PacketSettings::serialize(&val, w)?,
                    Params::TabComplete(val) => PacketTabComplete::serialize(&val, w)?,
                    Params::EnchantItem(val) => PacketEnchantItem::serialize(&val, w)?,
                    Params::WindowClick(val) => PacketWindowClick::serialize(&val, w)?,
                    Params::CloseWindow(val) => PacketCloseWindow::serialize(&val, w)?,
                    Params::CustomPayload(val) => PacketCustomPayload::serialize(&val, w)?,
                    Params::EditBook(val) => PacketEditBook::serialize(&val, w)?,
                    Params::QueryEntityNbt(val) => PacketQueryEntityNbt::serialize(&val, w)?,
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
                    Params::PickItem(val) => PacketPickItem::serialize(&val, w)?,
                    Params::CraftRecipeRequest(val) => PacketCraftRecipeRequest::serialize(&val, w)?,
                    Params::Abilities(val) => PacketAbilities::serialize(&val, w)?,
                    Params::BlockDig(val) => PacketBlockDig::serialize(&val, w)?,
                    Params::EntityAction(val) => PacketEntityAction::serialize(&val, w)?,
                    Params::SteerVehicle(val) => PacketSteerVehicle::serialize(&val, w)?,
                    Params::Pong(val) => PacketPong::serialize(&val, w)?,
                    Params::RecipeBook(val) => PacketRecipeBook::serialize(&val, w)?,
                    Params::DisplayedRecipe(val) => PacketDisplayedRecipe::serialize(&val, w)?,
                    Params::NameItem(val) => PacketNameItem::serialize(&val, w)?,
                    Params::ResourcePackReceive(val) => PacketResourcePackReceive::serialize(&val, w)?,
                    Params::AdvancementTab(val) => PacketAdvancementTab::serialize(&val, w)?,
                    Params::SelectTrade(val) => PacketSelectTrade::serialize(&val, w)?,
                    Params::SetBeaconEffect(val) => PacketSetBeaconEffect::serialize(&val, w)?,
                    Params::HeldItemSlot(val) => PacketHeldItemSlot::serialize(&val, w)?,
                    Params::UpdateCommandBlock(val) => PacketUpdateCommandBlock::serialize(&val, w)?,
                    Params::UpdateCommandBlockMinecart(val) => PacketUpdateCommandBlockMinecart::serialize(&val, w)?,
                    Params::SetCreativeSlot(val) => PacketSetCreativeSlot::serialize(&val, w)?,
                    Params::UpdateJigsawBlock(val) => PacketUpdateJigsawBlock::serialize(&val, w)?,
                    Params::UpdateStructureBlock(val) => PacketUpdateStructureBlock::serialize(&val, w)?,
                    Params::UpdateSign(val) => PacketUpdateSign::serialize(&val, w)?,
                    Params::ArmAnimation(val) => PacketArmAnimation::serialize(&val, w)?,
                    Params::Spectate(val) => PacketSpectate::serialize(&val, w)?,
                    Params::BlockPlace(val) => PacketBlockPlace::serialize(&val, w)?,
                    Params::UseItem(val) => PacketUseItem::serialize(&val, w)?,
                    Params::Default => w,
                };

                Ok(w)
            }
        }

        pub struct ToServer<'a> {
            name: VarInt,
            params: Params<'a>,
        }
        impl<'t: 'a, 'a> Packet<'t> for ToServer<'a> {
            fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                let w = VarInt::serialize(&self.name, w)?;
                let w = Params::serialize(&self.params, w)?;

                Ok(w)
            }

            fn deserialize(input: &'t [u8]) -> nom::IResult<&'t [u8], Self> {
                (|input| {
                    let (input, self_name) = (VarInt::deserialize)(input)?;
                    let (input, self_params) = (|input| match &format!("{}", self_name)[..] {
                        "0x00" => map(PacketTeleportConfirm::deserialize, Params::TeleportConfirm)(input),
                        "0x01" => map(PacketQueryBlockNbt::deserialize, Params::QueryBlockNbt)(input),
                        "0x02" => map(PacketSetDifficulty::deserialize, Params::SetDifficulty)(input),
                        "0x03" => map(PacketChat::deserialize, Params::Chat)(input),
                        "0x04" => map(PacketClientCommand::deserialize, Params::ClientCommand)(input),
                        "0x05" => map(PacketSettings::deserialize, Params::Settings)(input),
                        "0x06" => map(PacketTabComplete::deserialize, Params::TabComplete)(input),
                        "0x07" => map(PacketEnchantItem::deserialize, Params::EnchantItem)(input),
                        "0x08" => map(PacketWindowClick::deserialize, Params::WindowClick)(input),
                        "0x09" => map(PacketCloseWindow::deserialize, Params::CloseWindow)(input),
                        "0x0a" => map(PacketCustomPayload::deserialize, Params::CustomPayload)(input),
                        "0x0b" => map(PacketEditBook::deserialize, Params::EditBook)(input),
                        "0x0c" => map(PacketQueryEntityNbt::deserialize, Params::QueryEntityNbt)(input),
                        "0x0d" => map(PacketUseEntity::deserialize, Params::UseEntity)(input),
                        "0x0e" => map(PacketGenerateStructure::deserialize, Params::GenerateStructure)(input),
                        "0x0f" => map(PacketKeepAlive::deserialize, Params::KeepAlive)(input),
                        "0x10" => map(PacketLockDifficulty::deserialize, Params::LockDifficulty)(input),
                        "0x11" => map(PacketPosition::deserialize, Params::Position)(input),
                        "0x12" => map(PacketPositionLook::deserialize, Params::PositionLook)(input),
                        "0x13" => map(PacketLook::deserialize, Params::Look)(input),
                        "0x14" => map(PacketFlying::deserialize, Params::Flying)(input),
                        "0x15" => map(PacketVehicleMove::deserialize, Params::VehicleMove)(input),
                        "0x16" => map(PacketSteerBoat::deserialize, Params::SteerBoat)(input),
                        "0x17" => map(PacketPickItem::deserialize, Params::PickItem)(input),
                        "0x18" => map(PacketCraftRecipeRequest::deserialize, Params::CraftRecipeRequest)(input),
                        "0x19" => map(PacketAbilities::deserialize, Params::Abilities)(input),
                        "0x1a" => map(PacketBlockDig::deserialize, Params::BlockDig)(input),
                        "0x1b" => map(PacketEntityAction::deserialize, Params::EntityAction)(input),
                        "0x1c" => map(PacketSteerVehicle::deserialize, Params::SteerVehicle)(input),
                        "0x1d" => map(PacketPong::deserialize, Params::Pong)(input),
                        "0x1e" => map(PacketRecipeBook::deserialize, Params::RecipeBook)(input),
                        "0x1f" => map(PacketDisplayedRecipe::deserialize, Params::DisplayedRecipe)(input),
                        "0x20" => map(PacketNameItem::deserialize, Params::NameItem)(input),
                        "0x21" => map(PacketResourcePackReceive::deserialize, Params::ResourcePackReceive)(input),
                        "0x22" => map(PacketAdvancementTab::deserialize, Params::AdvancementTab)(input),
                        "0x23" => map(PacketSelectTrade::deserialize, Params::SelectTrade)(input),
                        "0x24" => map(PacketSetBeaconEffect::deserialize, Params::SetBeaconEffect)(input),
                        "0x25" => map(PacketHeldItemSlot::deserialize, Params::HeldItemSlot)(input),
                        "0x26" => map(PacketUpdateCommandBlock::deserialize, Params::UpdateCommandBlock)(input),
                        "0x27" => map(PacketUpdateCommandBlockMinecart::deserialize, Params::UpdateCommandBlockMinecart)(input),
                        "0x28" => map(PacketSetCreativeSlot::deserialize, Params::SetCreativeSlot)(input),
                        "0x29" => map(PacketUpdateJigsawBlock::deserialize, Params::UpdateJigsawBlock)(input),
                        "0x2a" => map(PacketUpdateStructureBlock::deserialize, Params::UpdateStructureBlock)(input),
                        "0x2b" => map(PacketUpdateSign::deserialize, Params::UpdateSign)(input),
                        "0x2c" => map(PacketArmAnimation::deserialize, Params::ArmAnimation)(input),
                        "0x2d" => map(PacketSpectate::deserialize, Params::Spectate)(input),
                        "0x2e" => map(PacketBlockPlace::deserialize, Params::BlockPlace)(input),
                        "0x2f" => map(PacketUseItem::deserialize, Params::UseItem)(input),
                        _ => Ok((input, Params::Default)),
                    })(input)?;
                    Ok((input, ToServer { name: self_name, params: self_params }))
                })(input)
            }
        }
    }
}
