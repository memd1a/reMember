use moople_derive::MooplePacket;
use moople_packet::{
    packet_opcode,
    proto::{list::MapleIndexListZ8, option::MapleOption8, MapleList32},
};

use crate::{
    id::{job_id::JobId, ItemId},
    send_opcodes::SendOpcodes,
    shared::{
        char::{AvatarData, CharacterId, RemoteCharSecondaryStatPartial},
        FootholdId, Vec2,
    },
    stats::PartialFlag,
};

#[derive(MooplePacket, Default, Debug)]
pub struct GuildMarkData {
    bg: u16,
    bg_color: u8,
    mark: u16,
    mark_color: u8,
}

#[derive(MooplePacket, Debug)]
pub struct PetInitInfo {
    tmpl_id: u32,
    name: String,
    pet_locker_sn: u64,
    pos_prev: Vec2,
    move_action: u8,
    fh: FootholdId,
}

#[derive(MooplePacket, Debug, Default)]
pub struct TamingMobData {
    level: u32,
    exp: u32,
    fatigue: u32,
}

#[derive(MooplePacket, Debug)]
pub struct MiniRoomData {
    sn: u32,
    title: String,
    private: bool,
    game_kind: bool,
    cur_users: u8,
    max_users: u8,
    game_on: bool,
}

#[derive(MooplePacket, Debug)]
pub struct ADBoardRemoteData {
    title: String,
}

#[derive(MooplePacket, Debug)]
pub struct CoupleRingData {
    item_sn: u64,
    pair_item_sn: u64,
    item_id: ItemId,
}

#[derive(MooplePacket, Debug)]
pub struct FriendshipRingData {
    item_sn: u64,
    pair_item_sn: u64,
    item_id: ItemId,
}

#[derive(MooplePacket, Debug)]
pub struct MarriageData {
    char_id: CharacterId,
    pair_char_id: CharacterId,
    wedding_ring_id: ItemId,
}

pub type PartialSecondaryStats = PartialFlag<(), RemoteCharSecondaryStatPartial>;

#[derive(MooplePacket, Debug)]
pub struct UserRemoteInitData {
    pub level: u8,
    pub name: String,
    pub guild_name: String,
    pub guild_mark: GuildMarkData,
    pub secondary_stat: PartialSecondaryStats, //TODO
    pub defense_att: u8,
    pub defense_state: u8,
    pub job: JobId,
    pub avatar: AvatarData,
    pub driver_id: CharacterId,
    pub passenger_id: CharacterId,
    pub choco_count: u32,
    pub active_effect_item: ItemId, //Active Item portable chair?
    pub completed_set_item_id: ItemId,
    pub portable_chair: ItemId,
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub show_admin_effects: bool,
    pub pet_infos: MapleIndexListZ8<PetInitInfo>,
    pub taming_mob: TamingMobData,
    // TODO: in theory the u8 should be the mini room type,
    // != 0 => read data
    pub mini_room: MapleOption8<MiniRoomData>,
    pub ad_board: MapleOption8<ADBoardRemoteData>,
    pub couple: MapleOption8<CoupleRingData>,
    pub friendship: MapleOption8<FriendshipRingData>,
    pub marriage: MapleOption8<MarriageData>,
    pub load_flags: u8, // 0: load dark force, 2: load dragon, 4: swallow,
    pub new_year_cards: MapleOption8<MapleList32<CharacterId>>,
    pub phase: u32,
}

#[derive(MooplePacket, Debug)]
pub struct UserEnterFieldResp {
    pub char_id: CharacterId,
    pub user_init_data: UserRemoteInitData,
}
packet_opcode!(UserEnterFieldResp, SendOpcodes::UserEnterField);

#[derive(MooplePacket, Debug)]
pub struct UserLeaveFieldResp {
    pub char_id: CharacterId,
}
packet_opcode!(UserLeaveFieldResp, SendOpcodes::UserLeaveField);
