
#[allow(unused_imports)]
use protocol_lib::types::*;

pub struct Position {
    x: i32,
    z: i32,
    y: i16,
}

pub struct RTrue {
    item_id: VarInt<i32>,
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

pub struct F2 {
    block_state: VarInt<i32>,
}

pub struct F3 {
    block_state: VarInt<i32>,
}

pub struct F14 {
    red: f32,
    green: f32,
    blue: f32,
    scale: f32,
}

pub struct F15 {
    from_red: f32,
    from_green: f32,
    from_blue: f32,
    scale: f32,
    to_red: f32,
    to_green: f32,
    to_blue: f32,
}

pub struct F24 {
    block_state: VarInt<i32>,
}

pub struct F35 {
    item: Slot,
}

pub enum Destination {
    MinecraftBlock(Position),
    MinecraftEntity(VarInt<i32>),
    Default,
}

pub struct F36<'a> {
    origin: Position,
    position_type: PrefixedString<'a, VarInt<i32>>,
    destination: Destination,
    ticks: VarInt<i32>,
}

pub enum Data<'a> {
    F2(F2),
    F3(F3),
    F14(F14),
    F15(F15),
    F24(F24),
    F35(F35),
    F36(F36<'a>),
    Default,
}

pub struct Particle<'a> {
    particle_id: VarInt<i32>,
    data: Data<'a>,
}

pub struct F8 {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

pub struct F16 {
    villager_type: VarInt<i32>,
    villager_profession: VarInt<i32>,
    level: VarInt<i32>,
}

pub enum EntityMetadata<'a> {
    F0(i8),
    F1(VarInt<i32>),
    F2(f32),
    F3(PrefixedString<'a, VarInt<i32>>),
    F4(PrefixedString<'a, VarInt<i32>>),
    F5(Option<PrefixedString<'a, VarInt<i32>>>),
    F6(Slot),
    F7(bool),
    F8(F8),
    F9(Position),
    F10(Option<Position>),
    F11(VarInt<i32>),
    F12(Option<Uuid>),
    F13(VarInt<i32>),
    F14(Nbt),
    F15(Particle<'a>),
    F16(F16),
    F17(VarInt<i32>),
    F18(VarInt<i32>),
    Default,
}

pub struct MinecraftSmeltingFormat<'a> {
    group: PrefixedString<'a, VarInt<i32>>,
    ingredient: PrefixedArray<Slot, VarInt<i32>>,
    result: Slot,
    experience: f32,
    cook_time: VarInt<i32>,
}

pub struct Tags<'a> {
    tag_name: PrefixedString<'a, VarInt<i32>>,
    entries: PrefixedArray<VarInt<i32>, VarInt<i32>>,
}

pub struct Ident4<'a> {
    tag_name: PrefixedString<'a, VarInt<i32>>,
    entries: PrefixedArray<VarInt<i32>, VarInt<i32>>,
}

pub struct Ident5 {
    x: u8,
    z: u8,
}

pub struct ChunkBlockEntity {
    ident5: Ident5,
    y: i16,
    r_type: VarInt<i32>,
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
    F1(VarInt<i32>),
    Default,
}

pub struct ExtraNodeDataF1<'a> {
    name: PrefixedString<'a, VarInt<i32>>,
}

pub struct BrigadierFloatFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

pub enum Min {
    F1(f32),
    Default,
}

pub enum Max {
    F1(f32),
    Default,
}

pub struct BrigadierFloat {
    flags: BrigadierFloatFlags,
    min: Min,
    max: Max,
}

pub struct BrigadierDoubleFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

pub enum BrigadierDoubleMin {
    F1(f64),
    Default,
}

pub enum BrigadierDoubleMax {
    F1(f64),
    Default,
}

pub struct BrigadierDouble {
    flags: BrigadierDoubleFlags,
    min: BrigadierDoubleMin,
    max: BrigadierDoubleMax,
}

pub struct BrigadierIntegerFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

pub enum BrigadierIntegerMin {
    F1(i32),
    Default,
}

pub enum BrigadierIntegerMax {
    F1(i32),
    Default,
}

pub struct BrigadierInteger {
    flags: BrigadierIntegerFlags,
    min: BrigadierIntegerMin,
    max: BrigadierIntegerMax,
}

pub struct BrigadierLongFlags {
    unused: u8,
    max_present: u8,
    min_present: u8,
}

pub enum BrigadierLongMin {
    F1(i64),
    Default,
}

pub enum BrigadierLongMax {
    F1(i64),
    Default,
}

pub struct BrigadierLong {
    flags: BrigadierLongFlags,
    min: BrigadierLongMin,
    max: BrigadierLongMax,
}

pub struct PropertiesMinecraftEntity {
    unused: u8,
    onlyAllowPlayers: u8,
    onlyAllowEntities: u8,
}

pub struct MinecraftScoreHolder {
    unused: u8,
    allowMultiple: u8,
}

pub struct MinecraftRange {
    allow_decimals: bool,
}

pub struct MinecraftResourceOrTag<'a> {
    registry: PrefixedString<'a, VarInt<i32>>,
}

pub struct MinecraftResource<'a> {
    registry: PrefixedString<'a, VarInt<i32>>,
}

pub enum Properties<'a> {
    BrigadierBool,
    BrigadierFloat(BrigadierFloat),
    BrigadierDouble(BrigadierDouble),
    BrigadierInteger(BrigadierInteger),
    BrigadierLong(BrigadierLong),
    BrigadierString(&'static str),
    MinecraftEntity(PropertiesMinecraftEntity),
    MinecraftGameProfile,
    MinecraftBlockPos,
    MinecraftColumnPos,
    MinecraftVec3,
    MinecraftVec2,
    MinecraftBlockState,
    MinecraftBlockPredicate,
    MinecraftItemStack,
    MinecraftItemPredicate,
    MinecraftColor,
    MinecraftComponent,
    MinecraftMessage,
    MinecraftNbt,
    MinecraftNbtPath,
    MinecraftObjective,
    MinecraftObjectiveCriteria,
    MinecraftOperation,
    MinecraftParticle,
    MinecraftAngle,
    MinecraftRotation,
    MinecraftScoreboardSlot,
    MinecraftScoreHolder(MinecraftScoreHolder),
    MinecraftSwizzle,
    MinecraftTeam,
    MinecraftItemSlot,
    MinecraftResourceLocation,
    MinecraftMobEffect,
    MinecraftFunction,
    MinecraftEntityAnchor,
    MinecraftRange(MinecraftRange),
    MinecraftIntRange,
    MinecraftFloatRange,
    MinecraftItemEnchantment,
    MinecraftEntitySummon,
    MinecraftDimension,
    MinecraftNbtCompoundTag,
    MinecraftTime,
    MinecraftResourceOrTag(MinecraftResourceOrTag<'a>),
    MinecraftResource(MinecraftResource<'a>),
    MinecraftUuid,
    Default,
}

pub enum SuggestionType<'a> {
    F1(PrefixedString<'a, VarInt<i32>>),
    Default,
}

pub struct ExtraNodeDataF2<'a> {
    name: PrefixedString<'a, VarInt<i32>>,
    parser: PrefixedString<'a, VarInt<i32>>,
    properties: Properties<'a>,
    suggestion_type: SuggestionType<'a>,
}

pub enum ExtraNodeData<'a> {
    F0,
    F1(ExtraNodeDataF1<'a>),
    F2(ExtraNodeDataF2<'a>),
    Default,
}

pub struct CommandNode<'a> {
    flags: Flags,
    children: PrefixedArray<VarInt<i32>, VarInt<i32>>,
    redirect_node: RedirectNode,
    extra_node_data: ExtraNodeData<'a>,
}

pub mod handshaking {
    pub mod toClient {
        use crate::test::*;
        pub enum Params {
            Default,
        }
        pub struct Packet {
            name: &'static str,
            params: Params,
        }
    }
    pub mod toServer {
        use crate::test::*;
        pub struct PacketSetProtocol<'a> {
            protocol_version: VarInt<i32>,
            server_host: PrefixedString<'a, VarInt<i32>>,
            server_port: u16,
            next_state: VarInt<i32>,
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
    pub mod toClient {
        use crate::test::*;
        pub struct PacketServerInfo<'a> {
            response: PrefixedString<'a, VarInt<i32>>,
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
    pub mod toServer {
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
    pub mod toClient {
        use crate::test::*;
        pub struct PacketDisconnect<'a> {
            reason: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketEncryptionBegin<'a> {
            server_id: PrefixedString<'a, VarInt<i32>>,
            public_key: PrefixedBuffer<'a, VarInt<i32>>,
            verify_token: PrefixedBuffer<'a, VarInt<i32>>,
        }
        pub struct PacketSuccess<'a> {
            uuid: Uuid,
            username: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketCompress {
            threshold: VarInt<i32>,
        }
        pub struct PacketLoginPluginRequest<'a> {
            message_id: VarInt<i32>,
            channel: PrefixedString<'a, VarInt<i32>>,
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
    pub mod toServer {
        use crate::test::*;
        pub struct PacketLoginStart<'a> {
            username: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketEncryptionBegin<'a> {
            shared_secret: PrefixedBuffer<'a, VarInt<i32>>,
            verify_token: PrefixedBuffer<'a, VarInt<i32>>,
        }
        pub struct PacketLoginPluginResponse<'a> {
            message_id: VarInt<i32>,
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
    pub mod toClient {
        use crate::test::*;
        pub struct PacketSpawnEntity {
            entity_id: VarInt<i32>,
            object_u_u_i_d: Uuid,
            r_type: VarInt<i32>,
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
            entity_id: VarInt<i32>,
            x: f64,
            y: f64,
            z: f64,
            count: i16,
        }
        pub struct PacketSpawnEntityLiving {
            entity_id: VarInt<i32>,
            entity_u_u_i_d: Uuid,
            r_type: VarInt<i32>,
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
            entity_id: VarInt<i32>,
            entity_u_u_i_d: Uuid,
            title: VarInt<i32>,
            location: Position,
            direction: u8,
        }
        pub struct PacketNamedEntitySpawn {
            entity_id: VarInt<i32>,
            player_u_u_i_d: Uuid,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
        }
        pub struct PacketAnimation {
            entity_id: VarInt<i32>,
            animation: u8,
        }
        pub struct PacketStatisticsEntriesItem {
            category_id: VarInt<i32>,
            statistic_id: VarInt<i32>,
            value: VarInt<i32>,
        }
        pub struct Ident7 {
            category_id: VarInt<i32>,
            statistic_id: VarInt<i32>,
            value: VarInt<i32>,
        }
        pub struct PacketStatistics {
            entries: PrefixedArray<Ident7, VarInt<i32>>,
        }
        pub struct Ident9Flags {
            _unused: u32,
            hidden: u8,
            show_toast: u8,
            has_background_texture: u8,
        }
        pub enum BackgroundTexture<'a> {
            F1(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub struct Ident9<'a> {
            title: PrefixedString<'a, VarInt<i32>>,
            description: PrefixedString<'a, VarInt<i32>>,
            icon: Slot,
            frame_type: VarInt<i32>,
            flags: Ident9Flags,
            background_texture: BackgroundTexture<'a>,
            x_cord: f32,
            y_cord: f32,
        }
        pub struct CriteriaItem<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: Void,
        }
        pub struct Ident10<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: Void,
        }
        pub struct AdvancementMappingItemValue<'a> {
            parent_id: Option<PrefixedString<'a, VarInt<i32>>>,
            display_data: Option<Ident9<'a>>,
            criteria: PrefixedArray<Ident10<'a>, VarInt<i32>>,
            requirements: PrefixedArray<
                PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>,
                VarInt<i32>,
            >,
        }
        pub struct AdvancementMappingItem<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: AdvancementMappingItemValue<'a>,
        }
        pub struct Ident14Flags {
            _unused: u32,
            hidden: u8,
            show_toast: u8,
            has_background_texture: u8,
        }
        pub enum Ident14BackgroundTexture<'a> {
            F1(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub struct Ident14<'a> {
            title: PrefixedString<'a, VarInt<i32>>,
            description: PrefixedString<'a, VarInt<i32>>,
            icon: Slot,
            frame_type: VarInt<i32>,
            flags: Ident14Flags,
            background_texture: Ident14BackgroundTexture<'a>,
            x_cord: f32,
            y_cord: f32,
        }
        pub struct Ident12Value<'a> {
            parent_id: Option<PrefixedString<'a, VarInt<i32>>>,
            display_data: Option<Ident14<'a>>,
            criteria: PrefixedArray<Ident10<'a>, VarInt<i32>>,
            requirements: PrefixedArray<
                PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>,
                VarInt<i32>,
            >,
        }
        pub struct Ident12<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: Ident12Value<'a>,
        }
        pub struct ProgressMappingItemValueItem<'a> {
            criterion_identifier: PrefixedString<'a, VarInt<i32>>,
            criterion_progress: Option<i64>,
        }
        pub struct Ident16<'a> {
            criterion_identifier: PrefixedString<'a, VarInt<i32>>,
            criterion_progress: Option<i64>,
        }
        pub struct ProgressMappingItem<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: PrefixedArray<Ident16<'a>, VarInt<i32>>,
        }
        pub struct Ident18<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: PrefixedArray<Ident16<'a>, VarInt<i32>>,
        }
        pub struct PacketAdvancements<'a> {
            reset: bool,
            advancement_mapping: PrefixedArray<Ident12<'a>, VarInt<i32>>,
            identifiers: PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>,
            progress_mapping: PrefixedArray<Ident18<'a>, VarInt<i32>>,
        }
        pub struct PacketBlockBreakAnimation {
            entity_id: VarInt<i32>,
            location: Position,
            destroy_stage: i8,
        }
        pub struct PacketTileEntityData {
            location: Position,
            action: VarInt<i32>,
            nbt_data: OptionalNbt,
        }
        pub struct PacketBlockAction {
            location: Position,
            byte1: u8,
            byte2: u8,
            block_id: VarInt<i32>,
        }
        pub struct PacketBlockChange {
            location: Position,
            r_type: VarInt<i32>,
        }
        pub enum PacketBossBarTitle<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F3(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum Health {
            F0(f32),
            F2(f32),
            Default,
        }
        pub enum Color {
            F0(VarInt<i32>),
            F4(VarInt<i32>),
            Default,
        }
        pub enum Dividers {
            F0(VarInt<i32>),
            F4(VarInt<i32>),
            Default,
        }
        pub enum PacketBossBarFlags {
            F0(u8),
            F5(u8),
            Default,
        }
        pub struct PacketBossBar<'a> {
            entity_u_u_i_d: Uuid,
            action: VarInt<i32>,
            title: PacketBossBarTitle<'a>,
            health: Health,
            color: Color,
            dividers: Dividers,
            flags: PacketBossBarFlags,
        }
        pub struct PacketDifficulty {
            difficulty: u8,
            difficulty_locked: bool,
        }
        pub struct MatchesItem<'a> {
            r_match: PrefixedString<'a, VarInt<i32>>,
            tooltip: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub struct Ident20<'a> {
            r_match: PrefixedString<'a, VarInt<i32>>,
            tooltip: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub struct PacketTabComplete<'a> {
            transaction_id: VarInt<i32>,
            start: VarInt<i32>,
            length: VarInt<i32>,
            matches: PrefixedArray<Ident20<'a>, VarInt<i32>>,
        }
        pub struct PacketDeclareCommands<'a> {
            nodes: PrefixedArray<CommandNode<'a>, VarInt<i32>>,
            root_index: VarInt<i32>,
        }
        pub enum PacketFacePlayerEntityId {
            RTrue(VarInt<i32>),
            Default,
        }
        pub enum EntityFeetEyes<'a> {
            RTrue(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub struct PacketFacePlayer<'a> {
            feet_eyes: VarInt<i32>,
            x: f64,
            y: f64,
            z: f64,
            is_entity: bool,
            entity_id: PacketFacePlayerEntityId,
            entity_feet_eyes: EntityFeetEyes<'a>,
        }
        pub struct PacketNbtQueryResponse {
            transaction_id: VarInt<i32>,
            nbt: OptionalNbt,
        }
        pub struct PacketChat<'a> {
            message: PrefixedString<'a, VarInt<i32>>,
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
            records: PrefixedArray<VarInt<i64>, VarInt<i32>>,
        }
        pub struct PacketCloseWindow {
            window_id: u8,
        }
        pub struct PacketOpenWindow<'a> {
            window_id: VarInt<i32>,
            inventory_type: VarInt<i32>,
            window_title: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketWindowItems {
            window_id: u8,
            state_id: VarInt<i32>,
            items: PrefixedArray<Slot, VarInt<i32>>,
            carried_item: Slot,
        }
        pub struct PacketCraftProgressBar {
            window_id: u8,
            property: i16,
            value: i16,
        }
        pub struct PacketSetSlot {
            window_id: i8,
            state_id: VarInt<i32>,
            slot: i16,
            item: Slot,
        }
        pub struct PacketSetCooldown {
            item_i_d: VarInt<i32>,
            cooldown_ticks: VarInt<i32>,
        }
        pub struct PacketCustomPayload<'a> {
            channel: PrefixedString<'a, VarInt<i32>>,
            data: RestBuffer<'a>,
        }
        pub struct PacketNamedSoundEffect<'a> {
            sound_name: PrefixedString<'a, VarInt<i32>>,
            sound_category: VarInt<i32>,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }
        pub struct PacketKickDisconnect<'a> {
            reason: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketEntityStatus {
            entity_id: i32,
            entity_status: i8,
        }
        pub struct AffectedBlockOffsetsItem {
            x: i8,
            y: i8,
            z: i8,
        }
        pub struct Ident22 {
            x: i8,
            y: i8,
            z: i8,
        }
        pub struct PacketExplosion {
            x: f32,
            y: f32,
            z: f32,
            radius: f32,
            affected_block_offsets: PrefixedArray<Ident22, VarInt<i32>>,
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
            nb_slots: VarInt<i32>,
            entity_id: i32,
        }
        pub struct PacketKeepAlive {
            keep_alive_id: i64,
        }
        pub struct PacketMapChunk<'a> {
            x: i32,
            z: i32,
            heightmaps: Nbt,
            chunk_data: PrefixedBuffer<'a, VarInt<i32>>,
            block_entities: PrefixedArray<ChunkBlockEntity, VarInt<i32>>,
            trust_edges: bool,
            sky_light_mask: PrefixedArray<i64, VarInt<i32>>,
            block_light_mask: PrefixedArray<i64, VarInt<i32>>,
            empty_sky_light_mask: PrefixedArray<i64, VarInt<i32>>,
            empty_block_light_mask: PrefixedArray<i64, VarInt<i32>>,
            sky_light: PrefixedArray<PrefixedArray<u8, VarInt<i32>>, VarInt<i32>>,
            block_light: PrefixedArray<PrefixedArray<u8, VarInt<i32>>, VarInt<i32>>,
        }
        pub struct PacketWorldEvent {
            effect_id: i32,
            location: Position,
            data: i32,
            global: bool,
        }
        pub struct PacketWorldParticlesDataF2 {
            block_state: VarInt<i32>,
        }
        pub struct PacketWorldParticlesDataF3 {
            block_state: VarInt<i32>,
        }
        pub struct PacketWorldParticlesDataF14 {
            red: f32,
            green: f32,
            blue: f32,
            scale: f32,
        }
        pub struct PacketWorldParticlesDataF15 {
            from_red: f32,
            from_green: f32,
            from_blue: f32,
            scale: f32,
            to_red: f32,
            to_green: f32,
            to_blue: f32,
        }
        pub struct PacketWorldParticlesDataF24 {
            block_state: VarInt<i32>,
        }
        pub struct PacketWorldParticlesDataF35 {
            item: Slot,
        }
        pub enum PacketWorldParticlesDataF36Destination {
            MinecraftBlock(Position),
            MinecraftEntity(VarInt<i32>),
            Default,
        }
        pub struct PacketWorldParticlesDataF36<'a> {
            origin: Position,
            position_type: PrefixedString<'a, VarInt<i32>>,
            destination: PacketWorldParticlesDataF36Destination,
            ticks: VarInt<i32>,
        }
        pub enum PacketWorldParticlesData<'a> {
            F2(PacketWorldParticlesDataF2),
            F3(PacketWorldParticlesDataF3),
            F14(PacketWorldParticlesDataF14),
            F15(PacketWorldParticlesDataF15),
            F24(PacketWorldParticlesDataF24),
            F35(PacketWorldParticlesDataF35),
            F36(PacketWorldParticlesDataF36<'a>),
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
            data: PacketWorldParticlesData<'a>,
        }
        pub struct PacketUpdateLight {
            chunk_x: VarInt<i32>,
            chunk_z: VarInt<i32>,
            trust_edges: bool,
            sky_light_mask: PrefixedArray<i64, VarInt<i32>>,
            block_light_mask: PrefixedArray<i64, VarInt<i32>>,
            empty_sky_light_mask: PrefixedArray<i64, VarInt<i32>>,
            empty_block_light_mask: PrefixedArray<i64, VarInt<i32>>,
            sky_light: PrefixedArray<PrefixedArray<u8, VarInt<i32>>, VarInt<i32>>,
            block_light: PrefixedArray<PrefixedArray<u8, VarInt<i32>>, VarInt<i32>>,
        }
        pub struct PacketLogin<'a> {
            entity_id: i32,
            is_hardcore: bool,
            game_mode: u8,
            previous_game_mode: i8,
            world_names: PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>,
            dimension_codec: Nbt,
            dimension: Nbt,
            world_name: PrefixedString<'a, VarInt<i32>>,
            hashed_seed: i64,
            max_players: VarInt<i32>,
            view_distance: VarInt<i32>,
            simulation_distance: VarInt<i32>,
            reduced_debug_info: bool,
            enable_respawn_screen: bool,
            is_debug: bool,
            is_flat: bool,
        }
        pub struct Ident27Item<'a> {
            r_type: VarInt<i32>,
            x: i8,
            z: i8,
            direction: u8,
            display_name: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub struct Ident29<'a> {
            r_type: VarInt<i32>,
            x: i8,
            z: i8,
            direction: u8,
            display_name: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub enum Rows {
            F0,
            Default(u8),
        }
        pub enum PacketMapX {
            F0,
            Default(u8),
        }
        pub enum PacketMapY {
            F0,
            Default(u8),
        }
        pub enum PacketMapData<'a> {
            F0,
            Default(PrefixedBuffer<'a, VarInt<i32>>),
        }
        pub struct PacketMap<'a> {
            item_damage: VarInt<i32>,
            scale: i8,
            locked: bool,
            icons: Option<PrefixedArray<Ident29<'a>, VarInt<i32>>>,
            columns: u8,
            rows: Rows,
            x: PacketMapX,
            y: PacketMapY,
            data: PacketMapData<'a>,
        }
        pub struct TradesItem {
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
        pub struct Ident32 {
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
            window_id: VarInt<i32>,
            trades: PrefixedArray<Ident32, u8>,
            villager_level: VarInt<i32>,
            experience: VarInt<i32>,
            is_regular_villager: bool,
            can_restock: bool,
        }
        pub struct PacketRelEntityMove {
            entity_id: VarInt<i32>,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            on_ground: bool,
        }
        pub struct PacketEntityMoveLook {
            entity_id: VarInt<i32>,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        pub struct PacketEntityLook {
            entity_id: VarInt<i32>,
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
            hand: VarInt<i32>,
        }
        pub struct PacketOpenSignEntity {
            location: Position,
        }
        pub struct PacketCraftRecipeResponse<'a> {
            window_id: i8,
            recipe: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketAbilities {
            flags: i8,
            flying_speed: f32,
            walking_speed: f32,
        }
        pub struct PacketEndCombatEvent {
            duration: VarInt<i32>,
            entity_id: i32,
        }
        pub struct PacketEnterCombatEvent {}
        pub struct PacketDeathCombatEvent<'a> {
            player_id: VarInt<i32>,
            entity_id: i32,
            message: PrefixedString<'a, VarInt<i32>>,
        }
        pub enum PacketPlayerInfoDataItemName<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub struct PacketPlayerInfoDataItemPropertiesF0Item<'a> {
            name: PrefixedString<'a, VarInt<i32>>,
            value: PrefixedString<'a, VarInt<i32>>,
            signature: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub struct Ident35<'a> {
            name: PrefixedString<'a, VarInt<i32>>,
            value: PrefixedString<'a, VarInt<i32>>,
            signature: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub enum PacketPlayerInfoDataItemProperties<'a> {
            F0(PrefixedArray<Ident35<'a>, VarInt<i32>>),
            Default,
        }
        pub enum Gamemode {
            F0(VarInt<i32>),
            F1(VarInt<i32>),
            Default,
        }
        pub enum Ping {
            F0(VarInt<i32>),
            F2(VarInt<i32>),
            Default,
        }
        pub enum PacketPlayerInfoDataItemDisplayName<'a> {
            F0(Option<PrefixedString<'a, VarInt<i32>>>),
            F3(Option<PrefixedString<'a, VarInt<i32>>>),
            Default,
        }
        pub struct PacketPlayerInfoDataItem<'a> {
            u_u_i_d: Uuid,
            name: PacketPlayerInfoDataItemName<'a>,
            properties: PacketPlayerInfoDataItemProperties<'a>,
            gamemode: Gamemode,
            ping: Ping,
            display_name: PacketPlayerInfoDataItemDisplayName<'a>,
        }
        pub enum Ident39Name<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum Ident39Properties<'a> {
            F0(PrefixedArray<Ident35<'a>, VarInt<i32>>),
            Default,
        }
        pub enum Ident39Gamemode {
            F0(VarInt<i32>),
            F1(VarInt<i32>),
            Default,
        }
        pub enum Ident39Ping {
            F0(VarInt<i32>),
            F2(VarInt<i32>),
            Default,
        }
        pub enum Ident39DisplayName<'a> {
            F0(Option<PrefixedString<'a, VarInt<i32>>>),
            F3(Option<PrefixedString<'a, VarInt<i32>>>),
            Default,
        }
        pub struct Ident39<'a> {
            u_u_i_d: Uuid,
            name: Ident39Name<'a>,
            properties: Ident39Properties<'a>,
            gamemode: Ident39Gamemode,
            ping: Ident39Ping,
            display_name: Ident39DisplayName<'a>,
        }
        pub struct PacketPlayerInfo<'a> {
            action: VarInt<i32>,
            data: PrefixedArray<Ident39<'a>, VarInt<i32>>,
        }
        pub struct PacketPosition {
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            flags: i8,
            teleport_id: VarInt<i32>,
            dismount_vehicle: bool,
        }
        pub enum Recipes2<'a> {
            F0(PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>),
            Default,
        }
        pub struct PacketUnlockRecipes<'a> {
            action: VarInt<i32>,
            crafting_book_open: bool,
            filtering_craftable: bool,
            smelting_book_open: bool,
            filtering_smeltable: bool,
            blast_furnace_open: bool,
            filtering_blast_furnace: bool,
            smoker_book_open: bool,
            filtering_smoker: bool,
            recipes1: PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>,
            recipes2: Recipes2<'a>,
        }
        pub struct PacketEntityDestroy {
            entity_ids: PrefixedArray<VarInt<i32>, VarInt<i32>>,
        }
        pub struct PacketRemoveEntityEffect {
            entity_id: VarInt<i32>,
            effect_id: i8,
        }
        pub struct PacketResourcePackSend<'a> {
            url: PrefixedString<'a, VarInt<i32>>,
            hash: PrefixedString<'a, VarInt<i32>>,
            forced: bool,
            prompt_message: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub struct PacketRespawn<'a> {
            dimension: Nbt,
            world_name: PrefixedString<'a, VarInt<i32>>,
            hashed_seed: i64,
            gamemode: u8,
            previous_gamemode: u8,
            is_debug: bool,
            is_flat: bool,
            copy_metadata: bool,
        }
        pub struct PacketEntityHeadRotation {
            entity_id: VarInt<i32>,
            head_yaw: i8,
        }
        pub struct PacketCamera {
            camera_id: VarInt<i32>,
        }
        pub struct PacketHeldItemSlot {
            slot: i8,
        }
        pub struct PacketUpdateViewPosition {
            chunk_x: VarInt<i32>,
            chunk_z: VarInt<i32>,
        }
        pub struct PacketUpdateViewDistance {
            view_distance: VarInt<i32>,
        }
        pub struct PacketScoreboardDisplayObjective<'a> {
            position: i8,
            name: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketEntityMetadata<'a> {
            entity_id: VarInt<i32>,
            metadata: Vec<EntityMetadata<'a>>,
        }
        pub struct PacketAttachEntity {
            entity_id: i32,
            vehicle_id: i32,
        }
        pub struct PacketEntityVelocity {
            entity_id: VarInt<i32>,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        }
        pub struct PacketEntityEquipment {
            entity_id: VarInt<i32>,
            equipments: std::collections::HashMap<i8, Slot>,
        }
        pub struct PacketExperience {
            experience_bar: f32,
            level: VarInt<i32>,
            total_experience: VarInt<i32>,
        }
        pub struct PacketUpdateHealth {
            health: f32,
            food: VarInt<i32>,
            food_saturation: f32,
        }
        pub enum DisplayText<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F2(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum PacketScoreboardObjectiveRType {
            F0(VarInt<i32>),
            F2(VarInt<i32>),
            Default,
        }
        pub struct PacketScoreboardObjective<'a> {
            name: PrefixedString<'a, VarInt<i32>>,
            action: i8,
            display_text: DisplayText<'a>,
            r_type: PacketScoreboardObjectiveRType,
        }
        pub struct PacketSetPassengers {
            entity_id: VarInt<i32>,
            passengers: PrefixedArray<VarInt<i32>, VarInt<i32>>,
        }
        pub enum PacketTeamsName<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F2(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum FriendlyFire {
            F0(i8),
            F2(i8),
            Default,
        }
        pub enum NameTagVisibility<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F2(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum CollisionRule<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F2(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum Formatting {
            F0(VarInt<i32>),
            F2(VarInt<i32>),
            Default,
        }
        pub enum Prefix<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F2(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum Suffix<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F2(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub enum Players<'a> {
            F0(PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>),
            F3(PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>),
            F4(PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>),
            Default,
        }
        pub struct PacketTeams<'a> {
            team: PrefixedString<'a, VarInt<i32>>,
            mode: i8,
            name: PacketTeamsName<'a>,
            friendly_fire: FriendlyFire,
            name_tag_visibility: NameTagVisibility<'a>,
            collision_rule: CollisionRule<'a>,
            formatting: Formatting,
            prefix: Prefix<'a>,
            suffix: Suffix<'a>,
            players: Players<'a>,
        }
        pub enum PacketScoreboardScoreValue {
            F1,
            Default(VarInt<i32>),
        }
        pub struct PacketScoreboardScore<'a> {
            item_name: PrefixedString<'a, VarInt<i32>>,
            action: VarInt<i32>,
            score_name: PrefixedString<'a, VarInt<i32>>,
            value: PacketScoreboardScoreValue,
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
            sound_id: VarInt<i32>,
            sound_category: VarInt<i32>,
            entity_id: VarInt<i32>,
            volume: f32,
            pitch: f32,
        }
        pub enum Source {
            F3(VarInt<i32>),
            F1(VarInt<i32>),
            Default,
        }
        pub enum Sound<'a> {
            F3(PrefixedString<'a, VarInt<i32>>),
            F2(PrefixedString<'a, VarInt<i32>>),
            Default,
        }
        pub struct PacketStopSound<'a> {
            flags: i8,
            source: Source,
            sound: Sound<'a>,
        }
        pub struct PacketSoundEffect {
            sound_id: VarInt<i32>,
            sound_category: VarInt<i32>,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        }
        pub struct PacketPlayerlistHeader<'a> {
            header: PrefixedString<'a, VarInt<i32>>,
            footer: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketCollect {
            collected_entity_id: VarInt<i32>,
            collector_entity_id: VarInt<i32>,
            pickup_item_count: VarInt<i32>,
        }
        pub struct PacketEntityTeleport {
            entity_id: VarInt<i32>,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        }
        pub struct ModifiersItem {
            uuid: Uuid,
            amount: f64,
            operation: i8,
        }
        pub struct Ident43 {
            uuid: Uuid,
            amount: f64,
            operation: i8,
        }
        pub struct PacketEntityUpdateAttributesPropertiesItem<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: f64,
            modifiers: PrefixedArray<Ident43, VarInt<i32>>,
        }
        pub struct Ident44<'a> {
            key: PrefixedString<'a, VarInt<i32>>,
            value: f64,
            modifiers: PrefixedArray<Ident43, VarInt<i32>>,
        }
        pub struct PacketEntityUpdateAttributes<'a> {
            entity_id: VarInt<i32>,
            properties: PrefixedArray<Ident44<'a>, VarInt<i32>>,
        }
        pub struct PacketEntityEffect {
            entity_id: VarInt<i32>,
            effect_id: i8,
            amplifier: i8,
            duration: VarInt<i32>,
            hide_particles: i8,
        }
        pub struct PacketSelectAdvancementTab<'a> {
            id: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub struct MinecraftCraftingShapeless<'a> {
            group: PrefixedString<'a, VarInt<i32>>,
            ingredients: PrefixedArray<PrefixedArray<Slot, VarInt<i32>>, VarInt<i32>>,
            result: Slot,
        }
        pub struct MinecraftCraftingShaped<'a> {
            width: VarInt<i32>,
            height: VarInt<i32>,
            group: PrefixedString<'a, VarInt<i32>>,
            ingredients: Vec<Vec<PrefixedArray<Slot, VarInt<i32>>>>,
            result: Slot,
        }
        pub struct MinecraftStonecutting<'a> {
            group: PrefixedString<'a, VarInt<i32>>,
            ingredient: PrefixedArray<Slot, VarInt<i32>>,
            result: Slot,
        }
        pub struct MinecraftSmithing {
            base: PrefixedArray<Slot, VarInt<i32>>,
            addition: PrefixedArray<Slot, VarInt<i32>>,
            result: Slot,
        }
        pub enum RecipesItemData<'a> {
            MinecraftCraftingShapeless(MinecraftCraftingShapeless<'a>),
            MinecraftCraftingShaped(MinecraftCraftingShaped<'a>),
            MinecraftCraftingSpecialArmordye,
            MinecraftCraftingSpecialBookcloning,
            MinecraftCraftingSpecialMapcloning,
            MinecraftCraftingSpecialMapextending,
            MinecraftCraftingSpecialFireworkRocket,
            MinecraftCraftingSpecialFireworkStar,
            MinecraftCraftingSpecialFireworkStarFade,
            MinecraftCraftingSpecialRepairitem,
            MinecraftCraftingSpecialTippedarrow,
            MinecraftCraftingSpecialBannerduplicate,
            MinecraftCraftingSpecialBanneraddpattern,
            MinecraftCraftingSpecialShielddecoration,
            MinecraftCraftingSpecialShulkerboxcoloring,
            MinecraftCraftingSpecialSuspiciousstew,
            MinecraftSmelting(MinecraftSmeltingFormat<'a>),
            MinecraftBlasting(MinecraftSmeltingFormat<'a>),
            MinecraftSmoking(MinecraftSmeltingFormat<'a>),
            MinecraftCampfireCooking(MinecraftSmeltingFormat<'a>),
            MinecraftStonecutting(MinecraftStonecutting<'a>),
            MinecraftSmithing(MinecraftSmithing),
            Default,
        }
        pub struct RecipesItem<'a> {
            r_type: PrefixedString<'a, VarInt<i32>>,
            recipe_id: PrefixedString<'a, VarInt<i32>>,
            data: RecipesItemData<'a>,
        }
        pub struct Ident46DataMinecraftCraftingShapeless<'a> {
            group: PrefixedString<'a, VarInt<i32>>,
            ingredients: PrefixedArray<PrefixedArray<Slot, VarInt<i32>>, VarInt<i32>>,
            result: Slot,
        }
        pub struct Ident46DataMinecraftCraftingShaped<'a> {
            width: VarInt<i32>,
            height: VarInt<i32>,
            group: PrefixedString<'a, VarInt<i32>>,
            ingredients: Vec<Vec<PrefixedArray<Slot, VarInt<i32>>>>,
            result: Slot,
        }
        pub struct Ident46DataMinecraftStonecutting<'a> {
            group: PrefixedString<'a, VarInt<i32>>,
            ingredient: PrefixedArray<Slot, VarInt<i32>>,
            result: Slot,
        }
        pub struct Ident46DataMinecraftSmithing {
            base: PrefixedArray<Slot, VarInt<i32>>,
            addition: PrefixedArray<Slot, VarInt<i32>>,
            result: Slot,
        }
        pub enum Ident46Data<'a> {
            MinecraftCraftingShapeless(Ident46DataMinecraftCraftingShapeless<'a>),
            MinecraftCraftingShaped(Ident46DataMinecraftCraftingShaped<'a>),
            MinecraftCraftingSpecialArmordye,
            MinecraftCraftingSpecialBookcloning,
            MinecraftCraftingSpecialMapcloning,
            MinecraftCraftingSpecialMapextending,
            MinecraftCraftingSpecialFireworkRocket,
            MinecraftCraftingSpecialFireworkStar,
            MinecraftCraftingSpecialFireworkStarFade,
            MinecraftCraftingSpecialRepairitem,
            MinecraftCraftingSpecialTippedarrow,
            MinecraftCraftingSpecialBannerduplicate,
            MinecraftCraftingSpecialBanneraddpattern,
            MinecraftCraftingSpecialShielddecoration,
            MinecraftCraftingSpecialShulkerboxcoloring,
            MinecraftCraftingSpecialSuspiciousstew,
            MinecraftSmelting(MinecraftSmeltingFormat<'a>),
            MinecraftBlasting(MinecraftSmeltingFormat<'a>),
            MinecraftSmoking(MinecraftSmeltingFormat<'a>),
            MinecraftCampfireCooking(MinecraftSmeltingFormat<'a>),
            MinecraftStonecutting(Ident46DataMinecraftStonecutting<'a>),
            MinecraftSmithing(Ident46DataMinecraftSmithing),
            Default,
        }
        pub struct Ident46<'a> {
            r_type: PrefixedString<'a, VarInt<i32>>,
            recipe_id: PrefixedString<'a, VarInt<i32>>,
            data: Ident46Data<'a>,
        }
        pub struct PacketDeclareRecipes<'a> {
            recipes: PrefixedArray<Ident46<'a>, VarInt<i32>>,
        }
        pub struct PacketTagsTagsItem<'a> {
            tag_type: PrefixedString<'a, VarInt<i32>>,
            tags: PrefixedArray<Ident4<'a>, VarInt<i32>>,
        }
        pub struct Ident47<'a> {
            tag_type: PrefixedString<'a, VarInt<i32>>,
            tags: PrefixedArray<Ident4<'a>, VarInt<i32>>,
        }
        pub struct PacketTags<'a> {
            tags: PrefixedArray<Ident47<'a>, VarInt<i32>>,
        }
        pub struct PacketAcknowledgePlayerDigging {
            location: Position,
            block: VarInt<i32>,
            status: VarInt<i32>,
            successful: bool,
        }
        pub enum PacketSculkVibrationSignalDestination {
            Block(Position),
            EntityId(VarInt<i32>),
            Default,
        }
        pub struct PacketSculkVibrationSignal<'a> {
            source_position: Position,
            destination_identifier: PrefixedString<'a, VarInt<i32>>,
            destination: PacketSculkVibrationSignalDestination,
            arrival_ticks: VarInt<i32>,
        }
        pub struct PacketClearTitles {
            reset: bool,
        }
        pub struct PacketInitializeWorldBorder {
            x: f64,
            z: f64,
            old_diameter: f64,
            new_diameter: f64,
            speed: VarInt<i64>,
            portal_teleport_boundary: VarInt<i32>,
            warning_blocks: VarInt<i32>,
            warning_time: VarInt<i32>,
        }
        pub struct PacketActionBar<'a> {
            text: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketWorldBorderCenter {
            x: f64,
            z: f64,
        }
        pub struct PacketWorldBorderLerpSize {
            old_diameter: f64,
            new_diameter: f64,
            speed: VarInt<i64>,
        }
        pub struct PacketWorldBorderSize {
            diameter: f64,
        }
        pub struct PacketWorldBorderWarningDelay {
            warning_time: VarInt<i32>,
        }
        pub struct PacketWorldBorderWarningReach {
            warning_blocks: VarInt<i32>,
        }
        pub struct PacketPing {
            id: i32,
        }
        pub struct PacketSetTitleSubtitle<'a> {
            text: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketSetTitleText<'a> {
            text: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketSetTitleTime {
            fade_in: i32,
            stay: i32,
            fade_out: i32,
        }
        pub struct PacketSimulationDistance {
            distance: VarInt<i32>,
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
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
    pub mod toServer {
        use crate::test::*;
        pub struct PacketTeleportConfirm {
            teleport_id: VarInt<i32>,
        }
        pub struct PacketQueryBlockNbt {
            transaction_id: VarInt<i32>,
            location: Position,
        }
        pub struct PacketSetDifficulty {
            new_difficulty: u8,
        }
        pub struct PacketEditBook<'a> {
            hand: VarInt<i32>,
            pages: PrefixedArray<PrefixedString<'a, VarInt<i32>>, VarInt<i32>>,
            title: Option<PrefixedString<'a, VarInt<i32>>>,
        }
        pub struct PacketQueryEntityNbt {
            transaction_id: VarInt<i32>,
            entity_id: VarInt<i32>,
        }
        pub struct PacketPickItem {
            slot: VarInt<i32>,
        }
        pub struct PacketNameItem<'a> {
            name: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketSelectTrade {
            slot: VarInt<i32>,
        }
        pub struct PacketSetBeaconEffect {
            primary_effect: VarInt<i32>,
            secondary_effect: VarInt<i32>,
        }
        pub struct PacketUpdateCommandBlock<'a> {
            location: Position,
            command: PrefixedString<'a, VarInt<i32>>,
            mode: VarInt<i32>,
            flags: u8,
        }
        pub struct PacketUpdateCommandBlockMinecart<'a> {
            entity_id: VarInt<i32>,
            command: PrefixedString<'a, VarInt<i32>>,
            track_output: bool,
        }
        pub struct PacketUpdateStructureBlock<'a> {
            location: Position,
            action: VarInt<i32>,
            mode: VarInt<i32>,
            name: PrefixedString<'a, VarInt<i32>>,
            offset_x: i8,
            offset_y: i8,
            offset_z: i8,
            size_x: i8,
            size_y: i8,
            size_z: i8,
            mirror: VarInt<i32>,
            rotation: VarInt<i32>,
            metadata: PrefixedString<'a, VarInt<i32>>,
            integrity: f32,
            seed: VarInt<i64>,
            flags: u8,
        }
        pub struct PacketTabComplete<'a> {
            transaction_id: VarInt<i32>,
            text: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketChat<'a> {
            message: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketClientCommand {
            action_id: VarInt<i32>,
        }
        pub struct PacketSettings<'a> {
            locale: PrefixedString<'a, VarInt<i32>>,
            view_distance: i8,
            chat_flags: VarInt<i32>,
            chat_colors: bool,
            skin_parts: u8,
            main_hand: VarInt<i32>,
            enable_text_filtering: bool,
            enable_server_listing: bool,
        }
        pub struct PacketEnchantItem {
            window_id: i8,
            enchantment: i8,
        }
        pub struct ChangedSlotsItem {
            location: i16,
            item: Slot,
        }
        pub struct Ident49 {
            location: i16,
            item: Slot,
        }
        pub struct PacketWindowClick {
            window_id: u8,
            state_id: VarInt<i32>,
            slot: i16,
            mouse_button: i8,
            mode: VarInt<i32>,
            changed_slots: PrefixedArray<Ident49, VarInt<i32>>,
            cursor_item: Slot,
        }
        pub struct PacketCloseWindow {
            window_id: u8,
        }
        pub struct PacketCustomPayload<'a> {
            channel: PrefixedString<'a, VarInt<i32>>,
            data: RestBuffer<'a>,
        }
        pub enum X {
            F2(f32),
            Default,
        }
        pub enum PacketUseEntityY {
            F2(f32),
            Default,
        }
        pub enum Z {
            F2(f32),
            Default,
        }
        pub enum PacketUseEntityHand {
            F0(VarInt<i32>),
            F2(VarInt<i32>),
            Default,
        }
        pub struct PacketUseEntity {
            target: VarInt<i32>,
            mouse: VarInt<i32>,
            x: X,
            y: PacketUseEntityY,
            z: Z,
            hand: PacketUseEntityHand,
            sneaking: bool,
        }
        pub struct PacketGenerateStructure {
            location: Position,
            levels: VarInt<i32>,
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
            recipe: PrefixedString<'a, VarInt<i32>>,
            make_all: bool,
        }
        pub struct PacketAbilities {
            flags: i8,
        }
        pub struct PacketBlockDig {
            status: VarInt<i32>,
            location: Position,
            face: i8,
        }
        pub struct PacketEntityAction {
            entity_id: VarInt<i32>,
            action_id: VarInt<i32>,
            jump_boost: VarInt<i32>,
        }
        pub struct PacketSteerVehicle {
            sideways: f32,
            forward: f32,
            jump: u8,
        }
        pub struct PacketDisplayedRecipe<'a> {
            recipe_id: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketRecipeBook {
            book_id: VarInt<i32>,
            book_open: bool,
            filter_active: bool,
        }
        pub struct PacketResourcePackReceive {
            result: VarInt<i32>,
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
            name: PrefixedString<'a, VarInt<i32>>,
            target: PrefixedString<'a, VarInt<i32>>,
            pool: PrefixedString<'a, VarInt<i32>>,
            final_state: PrefixedString<'a, VarInt<i32>>,
            joint_type: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketUpdateSign<'a> {
            location: Position,
            text1: PrefixedString<'a, VarInt<i32>>,
            text2: PrefixedString<'a, VarInt<i32>>,
            text3: PrefixedString<'a, VarInt<i32>>,
            text4: PrefixedString<'a, VarInt<i32>>,
        }
        pub struct PacketArmAnimation {
            hand: VarInt<i32>,
        }
        pub struct PacketSpectate {
            target: Uuid,
        }
        pub struct PacketBlockPlace {
            hand: VarInt<i32>,
            location: Position,
            direction: VarInt<i32>,
            cursor_x: f32,
            cursor_y: f32,
            cursor_z: f32,
            inside_block: bool,
        }
        pub struct PacketUseItem {
            hand: VarInt<i32>,
        }
        pub enum TabId<'a> {
            F0(PrefixedString<'a, VarInt<i32>>),
            F1,
            Default,
        }
        pub struct PacketAdvancementTab<'a> {
            action: VarInt<i32>,
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
        pub struct Packet<'a> {
            name: &'static str,
            params: Params<'a>,
        }
    }
}
