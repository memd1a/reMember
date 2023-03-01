use moople_derive::MooplePacket;
use moople_packet::proto::{
    conditional::CondEither,
    list::{MapleIndexList8, MapleIndexListZ16, MapleIndexListZ8},
    option::MapleOption8,
    time::MapleTime,
    MapleList16, MapleList32,
};

use crate::{
    id::{
        job_id::{JobId, SubJob},
        FaceId, HairId, ItemId, MapId, SkillId, Skin,
    },
    maple_stats,
};

use super::{job::Job, Gender, NameStr};

pub type MobId = u32;

const CHAR_PET_LEN: usize = 3;
pub type CashID = u64;
pub type PetIds = [ItemId; CHAR_PET_LEN];
//TODO:
pub type Pets = [u64; CHAR_PET_LEN];
pub type PetCashIds = [CashID; CHAR_PET_LEN];
pub type Money = u32;
pub type CharacterId = u32;

#[derive(MooplePacket, Debug)]
pub struct ExtendedSP {
    pubindex: u8,
    value: u8,
}

#[derive(MooplePacket, Debug)]
pub struct CharStat {
    pub char_id: u32,
    pub name: NameStr,
    pub gender: Gender,
    pub skin_color: Skin,
    pub face: FaceId,
    pub hair: HairId,
    pub pets: Pets,
    pub level: u8,
    pub job_id: JobId,
    pub str: u16,
    pub dex: u16,
    pub int: u16,
    pub luk: u16,
    pub hp: u32,
    pub max_hp: u32,
    pub mp: u32,
    pub max_mp: u32,
    pub ap: u16,
    #[pkt(either(field = "job_id", cond = "JobId::has_extended_sp"))]
    pub sp: CondEither<ExtendedSP, u16>,
    pub exp: i32,
    pub fame: u16,
    pub tmp_exp: u32,
    pub map_id: MapId,
    pub portal: u8,
    // TODO: Is this playtime in seconds
    pub playtime: u32,
    pub sub_job: SubJob,
}

impl CharStat {
    pub fn get_job(&self) -> Job {
        Job::new(self.job_id, self.sub_job)
    }

    pub fn set_job(&mut self, job: Job) {
        //TODO: maybe define a transparent mapping layer like
        // like ignoring to (de)serialiaze job and allow mapping fields
        // for job_id, sub_job which reference to the job field
        self.job_id = job.job_id;
        self.sub_job = job.sub_job;
    }
}

#[derive(MooplePacket, Debug, Clone)]
pub struct AvatarData {
    pub gender: Gender,
    pub skin: Skin,
    pub face: FaceId,
    pub mega: bool,
    pub hair: HairId,
    pub equips: MapleIndexList8<ItemId>,
    pub masked_equips: MapleIndexList8<ItemId>,
    pub weapon_sticker_id: ItemId,
    pub pets: PetIds,
}

#[derive(Debug, MooplePacket)]
pub struct UnknownCharExtraData {
    unknown: u8,
    // IDs?
    unknown_list: MapleList32<u64>,
    timestamps: MapleList32<u64>,
}

#[derive(Debug, MooplePacket)]
pub struct SkillInfo {
    id: SkillId,
    level: u32,
    expiration: MapleTime,
    //TODO if is_skill_need_master_level, 4th job only?
    master_level: u32,
}

#[derive(Debug, MooplePacket)]
pub struct SkillCooltime {
    id: SkillId,
    time_left: u16,
}

/*
limits:
 class ZRef<GW_ItemSlotBase> aEquipped[0x3c];
    class ZRef<GW_ItemSlotBase> aEquipped2[0x3c];
    class ZRef<GW_ItemSlotBase> aDragonEquipped[0x4];
    class ZRef<GW_ItemSlotBase> aMechanicEquipped[0x5];
    class ZArray<ZRef<GW_ItemSlotBase> > aaItemSlot[0x6];


*/

pub type QuestId = u16;

#[derive(Debug, MooplePacket)]
pub struct QuestInfo {
    id: QuestId,
    value: String,
}

#[derive(Debug, MooplePacket)]
pub struct QuestCompleteInfo {
    id: QuestId,
    time: MapleTime,
}

#[derive(Debug, MooplePacket)]
pub struct MiniGameInfo {
    game_id: u32,
    win: u32,
    draw: u32,
    score: u32,
}

pub type CharId = u32;

#[derive(Debug, MooplePacket)]
pub struct CoupleRecord {
    pair_char_id: CharId,
    pair_char_name: NameStr,
    sn: CashID,
    pair_sn: CashID,
}

#[derive(Debug, MooplePacket)]
pub struct FriendRecord {
    pair_char_id: CharId,
    pair_char_name: NameStr,
    sn: CashID,
    pair_sn: CashID,
    friend_item_id: ItemId,
}

#[derive(Debug, MooplePacket)]
pub struct MarriageRecord {
    marriage_no: u32,
    groom_id: CharId,
    bride_id: CharId,
    status: u16, // 3 == married?
    groom_item_id: ItemId,
    bride_item_id: ItemId,
    groom_name: NameStr,
    bride_name: NameStr,
}

#[derive(Debug, MooplePacket)]
pub struct SocialRecords {
    couple_records: MapleList16<CoupleRecord>,
    friend_records: MapleList16<FriendRecord>,
    marriage_records: MapleList16<MarriageRecord>,
}

#[derive(Debug, MooplePacket, Default)]
pub struct TeleportRockInfo {
    //TODO allow MapID
    pub maps: [MapId; 5],
    pub vip_maps: [MapId; 10],
}

#[derive(Debug, MooplePacket)]
pub struct NewYearCardInfo {
    id: u32, //sn
    sender_id: CharId,
    sender_name: String,
    is_sender_discarded: bool,
    data_sent: MapleTime,
    receiver_id: CharId,
    receiver_name: String,
    is_receiver_discarded: bool,
    is_receiver_received: bool,
    date_deceived: MapleTime,
    content: String,
}

#[derive(Debug, MooplePacket)]
pub struct QuestRecordExpired {
    id: QuestId,
    value: String,
}

#[derive(Debug, MooplePacket, Default)]
pub struct WildHunterInfo {
    //TODO proper typing
    pub riding_ty_id: u8,
    pub captured_mobs: [MobId; 5],
}

#[derive(Debug, MooplePacket)]
pub struct QuestCompleteOldInfo {
    id: QuestId,
    time: MapleTime,
}

#[derive(Debug, MooplePacket)]
pub struct VisitorQuestLogInfo {
    id: QuestId,
    unknown: u16,
}

pub type ItemSlot = u32;

#[derive(MooplePacket, Debug)]
pub struct CharDataStat {
    pub stat: CharStat,
    pub friend_max: u8,
    pub linked_character: MapleOption8<String>,
}

#[derive(MooplePacket, Debug, Default)]
pub struct CharDataEquipped {
    pub equipped: MapleIndexListZ16<ItemSlot>,
    pub equipped_cash: MapleIndexListZ16<ItemSlot>,
    pub equip: MapleIndexListZ16<ItemSlot>,
    pub dragon_equipped: MapleIndexListZ16<ItemSlot>,
    pub mechanic_equipped: MapleIndexListZ16<ItemSlot>,
}

// TODO always has combat orders + extra data

#[derive(MooplePacket, Debug)]
pub struct CharDataHeader {
    pub combat_orders: u8,
    pub extra_data: MapleOption8<UnknownCharExtraData>,
}



maple_stats!(
    CharData,
    CharDataFlags,
    u64,
    CharDataHeader,
    STAT(CharDataStat) => 0,
    MONEY(Money) => 1,
    INV_SIZE([u8; 5]) => 7,
    EQUIP_EXT_SLOT_EXPIRE(MapleTime) => 20,
    EQUIPPED(CharDataEquipped) => 2,
    USE_INV(MapleIndexListZ8<ItemSlot>) => 3,
    SETUP_INV(MapleIndexListZ8<ItemSlot>) => 4,
    ETC_INV(MapleIndexListZ8<ItemSlot>) => 5,
    CASH_INV(MapleIndexListZ8<ItemSlot>) => 6,
    // InvSize 7
    SKILL_RECORDS(MapleList16<SkillInfo>) => 8,
    SKILL_COOLTIME(MapleList16<SkillCooltime>) => 15,
    QUESTS(MapleList16<QuestInfo>) => 9,
    QUESTS_COMPLETED(MapleList16<QuestCompleteInfo>) => 14,
    MINI_GAME_RECORDS(MapleList16<MiniGameInfo>) => 10,
    SOCIAL_RECORDS(MapleList16<SocialRecords>) => 11,
    TELEPORT_ROCK_INFO(TeleportRockInfo) => 12,
    // Unknown 13
    // QuestsCompleted 14
    // SkillCooltimes 15
    // Monsterbook Card 16
    // Monster Book Cover  17
    NEW_YEAR_CARDS(MapleList16<NewYearCardInfo>) => 18,
    QUEST_RECORDS_EXPIRED(MapleList16<QuestRecordExpired>) => 19,
    // EquipExtExpire 20
    //TODO this has to be optional in the all struct, bneed to implement this later somehow
    //WILD_HUNTER_INFO(WildHunterInfo) => 21,
    QUEST_COMPLETE_OLD(MapleList16<QuestCompleteOldInfo>) => 22,
    VISITOR_QUEST_LOG_INFO(MapleList16<VisitorQuestLogInfo>) => 23,

);

#[cfg(test)]
mod tests {
    use moople_packet::proto::CondOption;

    use crate::stats::PartialFlagData;

    use super::*;

    #[test]
    fn basic_flags() {
        let char_data = CharDataPartial {
            stat: CondOption(None),
            money: CondOption(None),
            inv_size: CondOption(None),
            equip_ext_slot_expire: CondOption(None),
            equipped: CondOption(None),
            use_inv: CondOption(None),
            setup_inv: CondOption(None),
            etc_inv: CondOption(None),
            cash_inv: CondOption(None),
            skill_records: CondOption(None),
            skill_cooltime: CondOption(None),
            quests: CondOption(None),
            quests_completed: CondOption(None),
            mini_game_records: CondOption(None),
            social_records: CondOption(None),
            teleport_rock_info: CondOption(None),
            new_year_cards: CondOption(None),
            quest_records_expired: CondOption(None),
            quest_complete_old: CondOption(None),
            visitor_quest_log_info: CondOption(None),
        };

        assert_eq!(char_data.get_flags().0, CharDataFlags::empty().0);
    }
}
