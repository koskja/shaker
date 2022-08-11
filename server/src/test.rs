
#[allow(unused_imports)]
use protocol_lib::types::*;
type VInt = VarInt<i32>;
type VLong = VarInt<i64>;
type VarString<'a> = PrefixedString<'a, VInt>;
type VarStringArray<'a> = PrefixedArray<PrefixedString<'a, VInt>, VInt>;
type VarArray<T> = PrefixedArray<T, VInt>;
type VarBuffer<'a> = PrefixedBuffer<'a, VInt>;

type optvarint = VInt;
pub struct Position {
    x: i32,
    z: i32,
    y: i16,
}
pub struct RTrue {
    item_id: VInt,
    item_count: i8,
    nbt_data: OptionalNbt,
}
pub enum Ident0 {
    RFalse,
    RTrue(RTrue),
    Default,
}
pub struct Slot {
    present: bool,
    ident0: Ident0,
}
pub struct Data2 {
    block_state: VInt,
}
pub struct Data3 {
    block_state: VInt,
}
pub struct Data14 {
    red: f32,
    green: f32,
    blue: f32,
    scale: f32,
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
pub struct Data24 {
    block_state: VInt,
}
pub struct Data35 {
    item: Slot,
}
pub enum Destination {
    Block(Position),
    Entity(VInt),
    Default,
}
pub struct Data36<'a> {
    origin: Position,
    position_type: VarString<'a>,
    destination: Destination,
    ticks: VInt,
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
pub struct Particle<'a> {
    particle_id: VInt,
    data: Data<'a>,
}
pub struct EntityMetadataItem8 {
    pitch: f32,
    yaw: f32,
    roll: f32,
}
pub struct EntityMetadataItem16 {
    villager_type: VInt,
    villager_profession: VInt,
    level: VInt,
}
pub enum EntityMetadataItem<'a> {
    EntityMetadataItem0(i8),
    EntityMetadataItem1(VInt),
    EntityMetadataItem2(f32),
    EntityMetadataItem3(VarString<'a>),
    EntityMetadataItem4(VarString<'a>),
    EntityMetadataItem5(Option<VarString<'a>>),
    EntityMetadataItem6(Slot),
    EntityMetadataItem7(bool),
    EntityMetadataItem8(EntityMetadataItem8),
    EntityMetadataItem9(Position),
    EntityMetadataItem10(Option<Position>),
    EntityMetadataItem11(VInt),
    EntityMetadataItem12(Option<Uuid>),
    EntityMetadataItem13(VInt),
    EntityMetadataItem14(Nbt),
    EntityMetadataItem15(Particle<'a>),
    EntityMetadataItem16(EntityMetadataItem16),
    EntityMetadataItem17(optvarint),
    EntityMetadataItem18(VInt),
    Default,
}
pub struct MinecraftSmeltingFormat<'a> {
    group: VarString<'a>,
    ingredient: VarArray<Slot>,
    result: Slot,
    experience: f32,
    cook_time: VInt,
}
pub struct Tag<'a> {
    tag_name: VarString<'a>,
    entries: VarArray<VInt>,
}
pub struct Ident4 {
    x: u8,
    z: u8,
}
pub struct ChunkBlockEntity {
    ident4: Ident4,
    y: i16,
    r_type: VInt,
    nbt_data: OptionalNbt,
}
pub struct Flags {
    unused: u8,
    has_custom_suggestions: u8,
    has_redirect_node: u8,
    has_command: u8,
    command_node_type: u8,
}
pub enum RedirectNode {
    RedirectNode1(VInt),
    Default,
}
pub struct ExtraNodeData1<'a> {
    name: VarString<'a>,
}
pub struct FloatFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}
pub enum Min {
    Min1(f32),
    Default,
}
pub enum Max {
    Max1(f32),
    Default,
}
pub struct Float {
    flags: FloatFlags,
    min: Min,
    max: Max,
}
pub struct DoubleFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}
pub enum DoubleMin {
    DoubleMin1(f64),
    Default,
}
pub enum DoubleMax {
    DoubleMax1(f64),
    Default,
}
pub struct Double {
    flags: DoubleFlags,
    min: DoubleMin,
    max: DoubleMax,
}
pub struct IntegerFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}
pub enum IntegerMin {
    IntegerMin1(i32),
    Default,
}
pub enum IntegerMax {
    IntegerMax1(i32),
    Default,
}
pub struct Integer {
    flags: IntegerFlags,
    min: IntegerMin,
    max: IntegerMax,
}
pub struct LongFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}
pub enum LongMin {
    LongMin1(i64),
    Default,
}
pub enum LongMax {
    LongMax1(i64),
    Default,
}
pub struct Long {
    flags: LongFlags,
    min: LongMin,
    max: LongMax,
}
pub struct MinecraftEntity {
    unused: u8,
    only_allow_players: u8,
    only_allow_entities: u8,
}
pub struct ScoreHolder {
    unused: u8,
    allow_multiple: u8,
}
pub struct Range {
    allow_decimals: bool,
}
pub struct ResourceOrTag<'a> {
    registry: VarString<'a>,
}
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
pub enum SuggestionType<'a> {
    SuggestionType1(VarString<'a>),
    Default,
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
pub struct CommandNode<'a> {
    flags: Flags,
    children: VarArray<VInt>,
    redirect_node: RedirectNode,
    extra_node_data: ExtraNodeData<'a>,
}
pub mod handshaking {
    pub mod clientbound {
        use crate::test::*;
        pub enum Params {
            Default,
        }
        pub struct Packet {
            name: &'static str,
            params: Params,
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketSetProtocol<'a> {
            protocol_version: VInt,
            server_host: VarString<'a>,
            server_port: u16,
            next_state: VInt,
        }
        pub struct PacketLegacyServerListPing {
            payload: u8,
        }
        pub enum Params<'a> {
            SetProtocol(PacketSetProtocol<'a>),
            LegacyServerListPing(PacketLegacyServerListPing),
            Default,
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
}
pub mod status {
    pub mod clientbound {
        use crate::test::*;
        pub struct PacketServerInfo<'a> {
            response: VarString<'a>,
        }
        pub struct PacketPing {
            time: i64,
        }
        pub enum Params<'a> {
            ServerInfo(PacketServerInfo<'a>),
            Ping(PacketPing),
            Default,
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketPingStart {}
        pub struct PacketPing {
            time: i64,
        }
        pub enum Params {
            PingStart(PacketPingStart),
            Ping(PacketPing),
            Default,
        }
        pub struct Packet {
            name: &'static str,
            params: Params,
        }
    }
}
pub mod login {
    pub mod clientbound {
        use crate::test::*;
        pub struct PacketDisconnect<'a> {
            reason: VarString<'a>,
        }
        pub struct PacketEncryptionBegin<'a> {
            server_id: VarString<'a>,
            public_key: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }
        pub struct PacketSuccess<'a> {
            uuid: Uuid,
            username: VarString<'a>,
        }
        pub struct PacketCompress {
            threshold: VInt,
        }
        pub struct PacketLoginPluginRequest<'a> {
            message_id: VInt,
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
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketLoginStart<'a> {
            username: VarString<'a>,
        }
        pub struct PacketEncryptionBegin<'a> {
            shared_secret: VarBuffer<'a>,
            verify_token: VarBuffer<'a>,
        }
        pub struct PacketLoginPluginResponse<'a> {
            message_id: VInt,
            data: Option<RestBuffer<'a>>,
        }
        pub enum Params<'a> {
            LoginStart(PacketLoginStart<'a>),
            EncryptionBegin(PacketEncryptionBegin<'a>),
            LoginPluginResponse(PacketLoginPluginResponse<'a>),
            Default,
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
}
pub mod play {
    pub mod clientbound {
        use crate::test::*;
        pub struct PacketSpawnEntity {
            entity_id: VInt,
            object_uuid: Uuid,
            r_type: VInt,
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
        pub struct PacketSpawnEntityExperienceOrb {
            entity_id: VInt,
            x: f64,
            y: f64,
            z: f64,
            count: i16,
        }
        pub struct PacketSpawnEntityLiving {
            entity_id: VInt,
            entity_uuid: Uuid,
            r_type: VInt,
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
        pub struct PacketSpawnEntityPainting {
            entity_id: VInt,
            entity_uuid: Uuid,
            title: VInt,
            location: Position,
            direction: u8,
        }
        pub struct PacketNamedEntitySpawn {
            entity_id: VInt,
            player_uuid: Uuid,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
        }
        pub struct PacketAnimation {
            entity_id: VInt,
            animation: u8,
        }
        pub struct StatisticsEntry {
            category_id: VInt,
            statistic_id: VInt,
            value: VInt,
        }
        pub struct PacketStatistics {
            entries: VarArray<StatisticsEntry>,
        }
        pub struct Ident7Flags {
            unused: u32,
            hidden: u8,
            show_toast: u8,
            has_background_texture: u8,
        }
        pub enum BackgroundTexture<'a> {
            BackgroundTexture1(VarString<'a>),
            Default,
        }
        pub struct Ident7<'a> {
            title: VarString<'a>,
            description: VarString<'a>,
            icon: Slot,
            frame_type: VInt,
            flags: Ident7Flags,
            background_texture: BackgroundTexture<'a>,
            x_cord: f32,
            y_cord: f32,
        }
        pub struct CriteriaItem<'a> {
            key: VarString<'a>,
            value: Void,
        }
        pub struct AdvancementMappingItemValue<'a> {
            parent_id: Option<VarString<'a>>,
            display_data: Option<Ident7<'a>>,
            criteria: VarArray<CriteriaItem<'a>>,
            requirements: VarArray<VarStringArray<'a>>,
        }
        pub struct AdvancementMappingItem<'a> {
            key: VarString<'a>,
            value: AdvancementMappingItemValue<'a>,
        }
        pub struct ProgressMappingItemValueItem<'a> {
            criterion_identifier: VarString<'a>,
            criterion_progress: Option<i64>,
        }
        pub struct ProgressMappingItem<'a> {
            key: VarString<'a>,
            value: VarArray<ProgressMappingItemValueItem<'a>>,
        }
        pub struct PacketAdvancements<'a> {
            reset: bool,
            advancement_mapping: VarArray<AdvancementMappingItem<'a>>,
            identifiers: VarStringArray<'a>,
            progress_mapping: VarArray<ProgressMappingItem<'a>>,
        }
        pub struct PacketBlockBreakAnimation {
            entity_id: VInt,
            location: Position,
            destroy_stage: i8,
        }
        pub struct PacketTileEntityData {
            location: Position,
            action: VInt,
            nbt_data: OptionalNbt,
        }
        pub struct PacketBlockAction {
            location: Position,
            byte1: u8,
            byte2: u8,
            block_id: VInt,
        }
        pub struct PacketBlockChange {
            location: Position,
            r_type: VInt,
        }
        pub enum BossBarTitle<'a> {
            BossBarTitle0(VarString<'a>),
            BossBarTitle3(VarString<'a>),
            Default,
        }
        pub enum Health {
            Health0(f32),
            Health2(f32),
            Default,
        }
        pub enum Color {
            Color0(VInt),
            Color4(VInt),
            Default,
        }
        pub enum Dividers {
            Dividers0(VInt),
            Dividers4(VInt),
            Default,
        }
        pub enum BossBarFlags {
            BossBarFlags0(u8),
            BossBarFlags5(u8),
            Default,
        }
        pub struct PacketBossBar<'a> {
            entity_uuid: Uuid,
            action: VInt,
            title: BossBarTitle<'a>,
            health: Health,
            color: Color,
            dividers: Dividers,
            flags: BossBarFlags,
        }
        pub struct PacketDifficulty {
            difficulty: u8,
            difficulty_locked: bool,
        }
        pub struct Matche<'a> {
            r_match: VarString<'a>,
            tooltip: Option<VarString<'a>>,
        }
        pub struct PacketTabComplete<'a> {
            transaction_id: VInt,
            start: VInt,
            length: VInt,
            matches: VarArray<Matche<'a>>,
        }
        pub struct PacketDeclareCommands<'a> {
            nodes: VarArray<CommandNode<'a>>,
            root_index: VInt,
        }
        pub enum FacePlayerEntityId {
            FacePlayerEntityIdTrue(VInt),
            Default,
        }
        pub enum EntityFeetEyes<'a> {
            EntityFeetEyesTrue(VarString<'a>),
            Default,
        }
        pub struct PacketFacePlayer<'a> {
            feet_eyes: VInt,
            x: f64,
            y: f64,
            z: f64,
            is_entity: bool,
            entity_id: FacePlayerEntityId,
            entity_feet_eyes: EntityFeetEyes<'a>,
        }
        pub struct PacketNbtQueryResponse {
            transaction_id: VInt,
            nbt: OptionalNbt,
        }
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
        pub struct PacketMultiBlockChange {
            chunk_coordinates: ChunkCoordinates,
            not_trust_edges: bool,
            records: VarArray<VLong>,
        }
        pub struct PacketCloseWindow {
            window_id: u8,
        }
        pub struct PacketOpenWindow<'a> {
            window_id: VInt,
            inventory_type: VInt,
            window_title: VarString<'a>,
        }
        pub struct PacketWindowItems {
            window_id: u8,
            state_id: VInt,
            items: VarArray<Slot>,
            carried_item: Slot,
        }
        pub struct PacketCraftProgressBar {
            window_id: u8,
            property: i16,
            value: i16,
        }
        pub struct PacketSetSlot {
            window_id: i8,
            state_id: VInt,
            slot: i16,
            item: Slot,
        }
        pub struct PacketSetCooldown {
            item_id: VInt,
            cooldown_ticks: VInt,
        }
        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }
        pub struct PacketNamedSoundEffect<'a> {
            sound_name: VarString<'a>,
            sound_category: VInt,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }
        pub struct PacketKickDisconnect<'a> {
            reason: VarString<'a>,
        }
        pub struct PacketEntityStatus {
            entity_id: i32,
            entity_status: i8,
        }
        pub struct AffectedBlockOffset {
            x: i8,
            y: i8,
            z: i8,
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
        pub struct PacketUnloadChunk {
            chunk_x: i32,
            chunk_z: i32,
        }
        pub struct PacketGameStateChange {
            reason: u8,
            game_mode: f32,
        }
        pub struct PacketOpenHorseWindow {
            window_id: u8,
            nb_slots: VInt,
            entity_id: i32,
        }
        pub struct PacketKeepAlive {
            keep_alive_id: i64,
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
        pub struct PacketWorldEvent {
            effect_id: i32,
            location: Position,
            data: i32,
            global: bool,
        }
        pub struct WorldParticlesData2 {
            block_state: VInt,
        }
        pub struct WorldParticlesData3 {
            block_state: VInt,
        }
        pub struct WorldParticlesData14 {
            red: f32,
            green: f32,
            blue: f32,
            scale: f32,
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
        pub struct WorldParticlesData24 {
            block_state: VInt,
        }
        pub struct WorldParticlesData35 {
            item: Slot,
        }
        pub enum WorldParticlesData36Destination {
            MinecraftBlock(Position),
            WorldParticlesData36DestinationEntity(VInt),
            Default,
        }
        pub struct WorldParticlesData36<'a> {
            origin: Position,
            position_type: VarString<'a>,
            destination: WorldParticlesData36Destination,
            ticks: VInt,
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
        pub struct PacketUpdateLight {
            chunk_x: VInt,
            chunk_z: VInt,
            trust_edges: bool,
            sky_light_mask: VarArray<i64>,
            block_light_mask: VarArray<i64>,
            empty_sky_light_mask: VarArray<i64>,
            empty_block_light_mask: VarArray<i64>,
            sky_light: VarArray<VarArray<u8>>,
            block_light: VarArray<VarArray<u8>>,
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
            max_players: VInt,
            view_distance: VInt,
            simulation_distance: VInt,
            reduced_debug_info: bool,
            enable_respawn_screen: bool,
            is_debug: bool,
            is_flat: bool,
        }
        pub struct Ident10<'a> {
            r_type: VInt,
            x: i8,
            z: i8,
            direction: u8,
            display_name: Option<VarString<'a>>,
        }
        pub enum Rows {
            Rows0,
            Default(u8),
        }
        pub enum MapX {
            MapX0,
            Default(u8),
        }
        pub enum MapY {
            MapY0,
            Default(u8),
        }
        pub enum MapData<'a> {
            MapData0,
            Default(VarBuffer<'a>),
        }
        pub struct PacketMap<'a> {
            item_damage: VInt,
            scale: i8,
            locked: bool,
            icons: Option<VarArray<Ident10<'a>>>,
            columns: u8,
            rows: Rows,
            x: MapX,
            y: MapY,
            data: MapData<'a>,
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
        pub struct PacketTradeList {
            window_id: VInt,
            trades: PrefixedArray<Trade, u8>,
            villager_level: VInt,
            experience: VInt,
            is_regular_villager: bool,
            can_restock: bool,
        }
        pub struct PacketRelEntityMove {
            entity_id: VInt,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            on_ground: bool,
        }
        pub struct PacketEntityMoveLook {
            entity_id: VInt,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        pub struct PacketEntityLook {
            entity_id: VInt,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        pub struct PacketVehicleMove {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        }
        pub struct PacketOpenBook {
            hand: VInt,
        }
        pub struct PacketOpenSignEntity {
            location: Position,
        }
        pub struct PacketCraftRecipeResponse<'a> {
            window_id: i8,
            recipe: VarString<'a>,
        }
        pub struct PacketAbilities {
            flags: i8,
            flying_speed: f32,
            walking_speed: f32,
        }
        pub struct PacketEndCombatEvent {
            duration: VInt,
            entity_id: i32,
        }
        pub struct PacketEnterCombatEvent {}
        pub struct PacketDeathCombatEvent<'a> {
            player_id: VInt,
            entity_id: i32,
            message: VarString<'a>,
        }
        pub enum PlayerInfoDataItemName<'a> {
            PlayerInfoDataItemName0(VarString<'a>),
            Default,
        }
        pub struct PlayerInfoDataItemProperties0Item<'a> {
            name: VarString<'a>,
            value: VarString<'a>,
            signature: Option<VarString<'a>>,
        }
        pub enum PlayerInfoDataItemProperties<'a> {
            PlayerInfoDataItemProperties0(VarArray<PlayerInfoDataItemProperties0Item<'a>>),
            Default,
        }
        pub enum Gamemode {
            Gamemode0(VInt),
            Gamemode1(VInt),
            Default,
        }
        pub enum Ping {
            Ping0(VInt),
            Ping2(VInt),
            Default,
        }
        pub enum PlayerInfoDataItemDisplayName<'a> {
            PlayerInfoDataItemDisplayName0(Option<VarString<'a>>),
            PlayerInfoDataItemDisplayName3(Option<VarString<'a>>),
            Default,
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
            action: VInt,
            data: VarArray<PlayerInfoDataItem<'a>>,
        }
        pub struct PacketPosition {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            flags: i8,
            teleport_id: VInt,
            dismount_vehicle: bool,
        }
        pub enum Recipes2<'a> {
            Recipes20(VarStringArray<'a>),
            Default,
        }
        pub struct PacketUnlockRecipes<'a> {
            action: VInt,
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
        pub struct PacketEntityDestroy {
            entity_ids: VarArray<VInt>,
        }
        pub struct PacketRemoveEntityEffect {
            entity_id: VInt,
            effect_id: i8,
        }
        pub struct PacketResourcePackSend<'a> {
            url: VarString<'a>,
            hash: VarString<'a>,
            forced: bool,
            prompt_message: Option<VarString<'a>>,
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
        pub struct PacketEntityHeadRotation {
            entity_id: VInt,
            head_yaw: i8,
        }
        pub struct PacketCamera {
            camera_id: VInt,
        }
        pub struct PacketHeldItemSlot {
            slot: i8,
        }
        pub struct PacketUpdateViewPosition {
            chunk_x: VInt,
            chunk_z: VInt,
        }
        pub struct PacketUpdateViewDistance {
            view_distance: VInt,
        }
        pub struct PacketScoreboardDisplayObjective<'a> {
            position: i8,
            name: VarString<'a>,
        }
        pub struct PacketEntityMetadata<'a> {
            entity_id: VInt,
            metadata: Vec<EntityMetadataItem<'a>>,
        }
        pub struct PacketAttachEntity {
            entity_id: i32,
            vehicle_id: i32,
        }
        pub struct PacketEntityVelocity {
            entity_id: VInt,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        }
        pub struct PacketEntityEquipment {
            entity_id: VInt,
            equipments: std::collections::HashMap<i8, Slot>,
        }
        pub struct PacketExperience {
            experience_bar: f32,
            level: VInt,
            total_experience: VInt,
        }
        pub struct PacketUpdateHealth {
            health: f32,
            food: VInt,
            food_saturation: f32,
        }
        pub enum DisplayText<'a> {
            DisplayText0(VarString<'a>),
            DisplayText2(VarString<'a>),
            Default,
        }
        pub enum ScoreboardObjectiveType {
            ScoreboardObjectiveType0(VInt),
            ScoreboardObjectiveType2(VInt),
            Default,
        }
        pub struct PacketScoreboardObjective<'a> {
            name: VarString<'a>,
            action: i8,
            display_text: DisplayText<'a>,
            r_type: ScoreboardObjectiveType,
        }
        pub struct PacketSetPassengers {
            entity_id: VInt,
            passengers: VarArray<VInt>,
        }
        pub enum TeamsName<'a> {
            TeamsName0(VarString<'a>),
            TeamsName2(VarString<'a>),
            Default,
        }
        pub enum FriendlyFire {
            FriendlyFire0(i8),
            FriendlyFire2(i8),
            Default,
        }
        pub enum NameTagVisibility<'a> {
            NameTagVisibility0(VarString<'a>),
            NameTagVisibility2(VarString<'a>),
            Default,
        }
        pub enum CollisionRule<'a> {
            CollisionRule0(VarString<'a>),
            CollisionRule2(VarString<'a>),
            Default,
        }
        pub enum Formatting {
            Formatting0(VInt),
            Formatting2(VInt),
            Default,
        }
        pub enum Prefix<'a> {
            Prefix0(VarString<'a>),
            Prefix2(VarString<'a>),
            Default,
        }
        pub enum Suffix<'a> {
            Suffix0(VarString<'a>),
            Suffix2(VarString<'a>),
            Default,
        }
        pub enum Players<'a> {
            Players0(VarStringArray<'a>),
            Players3(VarStringArray<'a>),
            Players4(VarStringArray<'a>),
            Default,
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
        pub enum ScoreboardScoreValue {
            ScoreboardScoreValue1,
            Default(VInt),
        }
        pub struct PacketScoreboardScore<'a> {
            item_name: VarString<'a>,
            action: VInt,
            score_name: VarString<'a>,
            value: ScoreboardScoreValue,
        }
        pub struct PacketSpawnPosition {
            location: Position,
            angle: f32,
        }
        pub struct PacketUpdateTime {
            age: i64,
            time: i64,
        }
        pub struct PacketEntitySoundEffect {
            sound_id: VInt,
            sound_category: VInt,
            entity_id: VInt,
            volume: f32,
            pitch: f32,
        }
        pub enum Source {
            Source3(VInt),
            Source1(VInt),
            Default,
        }
        pub enum Sound<'a> {
            Sound3(VarString<'a>),
            Sound2(VarString<'a>),
            Default,
        }
        pub struct PacketStopSound<'a> {
            flags: i8,
            source: Source,
            sound: Sound<'a>,
        }
        pub struct PacketSoundEffect {
            sound_id: VInt,
            sound_category: VInt,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }
        pub struct PacketPlayerlistHeader<'a> {
            header: VarString<'a>,
            footer: VarString<'a>,
        }
        pub struct PacketCollect {
            collected_entity_id: VInt,
            collector_entity_id: VInt,
            pickup_item_count: VInt,
        }
        pub struct PacketEntityTeleport {
            entity_id: VInt,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        pub struct Modifier {
            uuid: Uuid,
            amount: f64,
            operation: i8,
        }
        pub struct EntityUpdateAttrsProperty<'a> {
            key: VarString<'a>,
            value: f64,
            modifiers: VarArray<Modifier>,
        }
        pub struct PacketEntityUpdateAttributes<'a> {
            entity_id: VInt,
            properties: VarArray<EntityUpdateAttrsProperty<'a>>,
        }
        pub struct PacketEntityEffect {
            entity_id: VInt,
            effect_id: i8,
            amplifier: i8,
            duration: VInt,
            hide_particles: i8,
        }
        pub struct PacketSelectAdvancementTab<'a> {
            id: Option<VarString<'a>>,
        }
        pub struct CraftingShapeless<'a> {
            group: VarString<'a>,
            ingredients: VarArray<VarArray<Slot>>,
            result: Slot,
        }
        pub struct CraftingShaped<'a> {
            width: VInt,
            height: VInt,
            group: VarString<'a>,
            ingredients: Vec<Vec<VarArray<Slot>>>,
            result: Slot,
        }
        pub struct Stonecutting<'a> {
            group: VarString<'a>,
            ingredient: VarArray<Slot>,
            result: Slot,
        }
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
        pub struct RecipesItem<'a> {
            r_type: VarString<'a>,
            recipe_id: VarString<'a>,
            data: RecipeData<'a>,
        }
        pub struct PacketDeclareRecipes<'a> {
            recipes: VarArray<RecipesItem<'a>>,
        }
        pub struct TagsTag<'a> {
            tag_type: VarString<'a>,
            tags: VarArray<Tag<'a>>,
        }
        pub struct PacketTags<'a> {
            tags: VarArray<TagsTag<'a>>,
        }
        pub struct PacketAcknowledgePlayerDigging {
            location: Position,
            block: VInt,
            status: VInt,
            successful: bool,
        }
        pub enum SculkVibrationSignalDestination {
            SculkVibrationSignalDestinationBlock(Position),
            SculkVibrationSignalDestinationEntityId(VInt),
            Default,
        }
        pub struct PacketSculkVibrationSignal<'a> {
            source_position: Position,
            destination_identifier: VarString<'a>,
            destination: SculkVibrationSignalDestination,
            arrival_ticks: VInt,
        }
        pub struct PacketClearTitles {
            reset: bool,
        }
        pub struct PacketInitializeWorldBorder {
            x: f64,
            z: f64,
            old_diameter: f64,
            new_diameter: f64,
            speed: VLong,
            portal_teleport_boundary: VInt,
            warning_blocks: VInt,
            warning_time: VInt,
        }
        pub struct PacketActionBar<'a> {
            text: VarString<'a>,
        }
        pub struct PacketWorldBorderCenter {
            x: f64,
            z: f64,
        }
        pub struct PacketWorldBorderLerpSize {
            old_diameter: f64,
            new_diameter: f64,
            speed: VLong,
        }
        pub struct PacketWorldBorderSize {
            diameter: f64,
        }
        pub struct PacketWorldBorderWarningDelay {
            warning_time: VInt,
        }
        pub struct PacketWorldBorderWarningReach {
            warning_blocks: VInt,
        }
        pub struct PacketPing {
            id: i32,
        }
        pub struct PacketSetTitleSubtitle<'a> {
            text: VarString<'a>,
        }
        pub struct PacketSetTitleText<'a> {
            text: VarString<'a>,
        }
        pub struct PacketSetTitleTime {
            fade_in: i32,
            stay: i32,
            fade_out: i32,
        }
        pub struct PacketSimulationDistance {
            distance: VInt,
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
            EntityMetadata(PacketEntityMetadata<'a>),
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
            Default,
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
    pub mod serverbound {
        use crate::test::*;
        pub struct PacketTeleportConfirm {
            teleport_id: VInt,
        }
        pub struct PacketQueryBlockNbt {
            transaction_id: VInt,
            location: Position,
        }
        pub struct PacketSetDifficulty {
            new_difficulty: u8,
        }
        pub struct PacketEditBook<'a> {
            hand: VInt,
            pages: VarStringArray<'a>,
            title: Option<VarString<'a>>,
        }
        pub struct PacketQueryEntityNbt {
            transaction_id: VInt,
            entity_id: VInt,
        }
        pub struct PacketPickItem {
            slot: VInt,
        }
        pub struct PacketNameItem<'a> {
            name: VarString<'a>,
        }
        pub struct PacketSelectTrade {
            slot: VInt,
        }
        pub struct PacketSetBeaconEffect {
            primary_effect: VInt,
            secondary_effect: VInt,
        }
        pub struct PacketUpdateCommandBlock<'a> {
            location: Position,
            command: VarString<'a>,
            mode: VInt,
            flags: u8,
        }
        pub struct PacketUpdateCommandBlockMinecart<'a> {
            entity_id: VInt,
            command: VarString<'a>,
            track_output: bool,
        }
        pub struct PacketUpdateStructureBlock<'a> {
            location: Position,
            action: VInt,
            mode: VInt,
            name: VarString<'a>,
            offset_x: i8,
            offset_y: i8,
            offset_z: i8,
            size_x: i8,
            size_y: i8,
            size_z: i8,
            mirror: VInt,
            rotation: VInt,
            metadata: VarString<'a>,
            integrity: f32,
            seed: VLong,
            flags: u8,
        }
        pub struct PacketTabComplete<'a> {
            transaction_id: VInt,
            text: VarString<'a>,
        }
        pub struct PacketChat<'a> {
            message: VarString<'a>,
        }
        pub struct PacketClientCommand {
            action_id: VInt,
        }
        pub struct PacketSettings<'a> {
            locale: VarString<'a>,
            view_distance: i8,
            chat_flags: VInt,
            chat_colors: bool,
            skin_parts: u8,
            main_hand: VInt,
            enable_text_filtering: bool,
            enable_server_listing: bool,
        }
        pub struct PacketEnchantItem {
            window_id: i8,
            enchantment: i8,
        }
        pub struct ChangedSlot {
            location: i16,
            item: Slot,
        }
        pub struct PacketWindowClick {
            window_id: u8,
            state_id: VInt,
            slot: i16,
            mouse_button: i8,
            mode: VInt,
            changed_slots: VarArray<ChangedSlot>,
            cursor_item: Slot,
        }
        pub struct PacketCloseWindow {
            window_id: u8,
        }
        pub struct PacketCustomPayload<'a> {
            channel: VarString<'a>,
            data: RestBuffer<'a>,
        }
        pub enum X {
            X2(f32),
            Default,
        }
        pub enum UseEntityY {
            UseEntityY2(f32),
            Default,
        }
        pub enum Z {
            Z2(f32),
            Default,
        }
        pub enum UseEntityHand {
            UseEntityHand0(VInt),
            UseEntityHand2(VInt),
            Default,
        }
        pub struct PacketUseEntity {
            target: VInt,
            mouse: VInt,
            x: X,
            y: UseEntityY,
            z: Z,
            hand: UseEntityHand,
            sneaking: bool,
        }
        pub struct PacketGenerateStructure {
            location: Position,
            levels: VInt,
            keep_jigsaws: bool,
        }
        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }
        pub struct PacketLockDifficulty {
            locked: bool,
        }
        pub struct PacketPosition {
            x: f64,
            y: f64,
            z: f64,
            on_ground: bool,
        }
        pub struct PacketPositionLook {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        }
        pub struct PacketLook {
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        }
        pub struct PacketFlying {
            on_ground: bool,
        }
        pub struct PacketVehicleMove {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        }
        pub struct PacketSteerBoat {
            left_paddle: bool,
            right_paddle: bool,
        }
        pub struct PacketCraftRecipeRequest<'a> {
            window_id: i8,
            recipe: VarString<'a>,
            make_all: bool,
        }
        pub struct PacketAbilities {
            flags: i8,
        }
        pub struct PacketBlockDig {
            status: VInt,
            location: Position,
            face: i8,
        }
        pub struct PacketEntityAction {
            entity_id: VInt,
            action_id: VInt,
            jump_boost: VInt,
        }
        pub struct PacketSteerVehicle {
            sideways: f32,
            forward: f32,
            jump: u8,
        }
        pub struct PacketDisplayedRecipe<'a> {
            recipe_id: VarString<'a>,
        }
        pub struct PacketRecipeBook {
            book_id: VInt,
            book_open: bool,
            filter_active: bool,
        }
        pub struct PacketResourcePackReceive {
            result: VInt,
        }
        pub struct PacketHeldItemSlot {
            slot_id: i16,
        }
        pub struct PacketSetCreativeSlot {
            slot: i16,
            item: Slot,
        }
        pub struct PacketUpdateJigsawBlock<'a> {
            location: Position,
            name: VarString<'a>,
            target: VarString<'a>,
            pool: VarString<'a>,
            final_state: VarString<'a>,
            joint_type: VarString<'a>,
        }
        pub struct PacketUpdateSign<'a> {
            location: Position,
            text1: VarString<'a>,
            text2: VarString<'a>,
            text3: VarString<'a>,
            text4: VarString<'a>,
        }
        pub struct PacketArmAnimation {
            hand: VInt,
        }
        pub struct PacketSpectate {
            target: Uuid,
        }
        pub struct PacketBlockPlace {
            hand: VInt,
            location: Position,
            direction: VInt,
            cursor_x: f32,
            cursor_y: f32,
            cursor_z: f32,
            inside_block: bool,
        }
        pub struct PacketUseItem {
            hand: VInt,
        }
        pub enum TabId<'a> {
            TabId0(VarString<'a>),
            TabId1,
            Default,
        }
        pub struct PacketAdvancementTab<'a> {
            action: VInt,
            tab_id: TabId<'a>,
        }
        pub struct PacketPong {
            id: i32,
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
            Default,
        }
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
}
