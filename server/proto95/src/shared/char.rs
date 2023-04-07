use moople_derive::MooplePacket;
use moople_packet::{
    packet_opcode, partial_data,
    proto::{
        conditional::CondEither,
        list::{MapleIndexList8, MapleIndexListZ16, MapleIndexListZ8},
        option::MapleOption8,
        partial::PartialFlag,
        time::{MapleDurationMs16, MapleDurationMs32, MapleExpiration, MapleTime},
        MapleList16, MapleList32,
    },
};

use crate::{
    game::mob::MobId,
    id::{
        job_id::{JobId, SubJob},
        FaceId, HairId, ItemId, MapId, SkillId, Skin,
    },
    send_opcodes::SendOpcodes,
};

use super::{item::Item, job::Job, Gender, NameStr};

const CHAR_PET_COUNT: usize = 3;
pub type CashID = u64;
pub type PetIds = [ItemId; CHAR_PET_COUNT];
//TODO:
pub type Pets = [u64; CHAR_PET_COUNT];
pub type PetCashIds = [CashID; CHAR_PET_COUNT];
pub type Money = u32;
pub type CharacterId = u32;

#[derive(MooplePacket, Debug)]
pub struct SkillPointPage {
    pub index: u8,
    pub value: u8,
}

pub type SkillPointPages = [SkillPointPage; 10];

#[derive(MooplePacket, Debug)]
pub struct CharStat {
    pub char_id: CharacterId,
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
    pub sp: CondEither<SkillPointPages, u16>,
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
pub struct AvatarEquips {
    pub equips: MapleIndexList8<ItemId>,
    pub masked_equips: MapleIndexList8<ItemId>,
    pub weapon_sticker_id: ItemId,
}

#[derive(MooplePacket, Debug, Clone)]
pub struct AvatarData {
    pub gender: Gender,
    pub skin: Skin,
    pub face: FaceId,
    pub mega: bool,
    pub hair: HairId,
    pub equips: AvatarEquips,
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
    pub id: SkillId,
    pub level: u32,
    pub expiration: MapleExpiration,
    //TODO if is_skill_need_master_level, 4th job only?
    pub master_level: u32,
}

#[derive(Debug, MooplePacket)]
pub struct SkillCooltime {
    pub id: SkillId,
    pub time_left: MapleDurationMs16,
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

#[derive(MooplePacket, Debug)]
pub struct CharDataStat {
    pub stat: CharStat,
    pub friend_max: u8,
    pub linked_character: MapleOption8<String>,
}

#[derive(MooplePacket, Debug, Default)]
pub struct CharDataEquipped {
    pub equipped: MapleIndexListZ16<Item>,
    pub equipped_cash: MapleIndexListZ16<Item>,
    pub equip: MapleIndexListZ16<Item>,
    pub dragon_equipped: MapleIndexListZ16<Item>,
    pub mechanic_equipped: MapleIndexListZ16<Item>,
}

partial_data!(
    CharForcedStat,
    CharForcedStatFlags,
    u32,
    Str(u16) => 1 << 0,
    Dex(u16) => 1 << 1,
    Int(u16) => 1 << 2,
    Luk(u16) => 1 << 3,
    Pad(u16) => 1 << 4,
    Pdd(u16) => 1 << 5,
    Mad(u16) => 1 << 6,
    Mdd(u16) => 1 << 7,
    Acc(u16) => 1 << 8,
    Eva(u16) => 1 << 9,
    Speed(u8) => 1 << 10,
    Jump(u8) => 1 << 11,
    SpeedMax(u8) => 1 << 12
);

#[derive(MooplePacket, Debug)]
pub struct CharForcedStatSetResp {
    pub stats: PartialFlag<(), CharForcedStatPartial>,
}
packet_opcode!(CharForcedStatSetResp, SendOpcodes::ForcedStatSet);

#[derive(MooplePacket, Debug)]
pub struct CharForcedStatResetResp;
packet_opcode!(CharForcedStatResetResp, SendOpcodes::ForcedStatReset);

partial_data!(
    CharStat,
    CharStatFlags,
    u32,
    Skin(Skin) => 1 << 0,
    Face(FaceId) => 1 << 1,
    Hair(HairId) => 1 << 2,
    Pet1(CashID) => 1 << 3,
    Pet2(CashID) => 1 << 19,
    Pet3(CashID) => 1 << 20,
    Level(u8) => 1 << 4,
    Job(JobId) => 1 << 5,
    Str(u16) => 1 << 6,
    Dex(u16) => 1 << 7,
    Int(u16) => 1 << 8,
    Luk(u16) => 1 << 9,
    Hp(u32) => 1 << 10,
    MaxHp(u32) => 1 << 11,
    Mp(u32) => 1 << 12,
    MaxMp(u32) => 1 << 13,
    Ap(u16) => 1 << 14,
    // TODO handle extended SP
    Sp(u16) => 1 << 15,
    Exp(u32) => 1 << 17

);

#[derive(Debug, MooplePacket)]
pub struct CharStatChangedResp {
    pub excl: bool,
    pub stats: PartialFlag<(), CharStatPartial>,
    //TODO Tail has to be decoded properly
    pub secondary_stat: bool,
    pub battle_recovery: bool,
}
packet_opcode!(CharStatChangedResp, SendOpcodes::StatChanged);

#[derive(MooplePacket, Debug)]
pub struct CharTempStatSetResp {
    pub temp_stats: PartialFlag<(), CharSecondaryStatPartial>,
    pub unknown: u16, // Delay?
}
packet_opcode!(CharTempStatSetResp, SendOpcodes::TemporaryStatSet);

#[derive(MooplePacket, Debug)]
pub struct CharTempStatResetResp {
    pub flags: CharSecondaryStatFlags,
}
packet_opcode!(CharTempStatResetResp, SendOpcodes::TemporaryStatReset);

// TODO always has combat orders + extra data

#[derive(MooplePacket, Debug)]
pub struct CharDataHeader {
    pub combat_orders: u8,
    pub extra_data: MapleOption8<UnknownCharExtraData>,
}

partial_data!(
    CharData,
    CharDataFlags,
    u64,
    Stat(CharDataStat) => 1 << 0,
    Money(Money) => 1 << 1,
    InvSize([u8; 5]) => 1 << 7,
    EquipExtSlotExpiration(MapleExpiration) => 1 << 20,
    Equipped(CharDataEquipped) => 1 << 2,
    UseInv(MapleIndexListZ8<Item>) => 1 << 3,
    SetupInv(MapleIndexListZ8<Item>) => 1 << 4,
    EtcInv(MapleIndexListZ8<Item>) => 1 << 5,
    CashInv(MapleIndexListZ8<Item>) => 1 << 6,
    // InvSize 1 << 7
    SkillRecords(MapleList16<SkillInfo>) => 1 << 8,
    SkllCooltime(MapleList16<SkillCooltime>) => 1 << 15,
    Quests(MapleList16<QuestInfo>) => 1 << 9,
    QuestsCompleted(MapleList16<QuestCompleteInfo>) => 1 << 14,
    MiniGameRecords(MapleList16<MiniGameInfo>) => 1 << 10,
    SocialRecords(MapleList16<SocialRecords>) => 1 << 11,
    TeleportRockInfo(TeleportRockInfo) => 1 << 12,
    // Unknown 1 << 13
    // QuestsCompleted 1 << 14
    // SkillCooltimes 1 << 15
    // Monsterbook Card 1 << 16
    // Monster Book Cover  1 << 17
    NewYearCards(MapleList16<NewYearCardInfo>) => 1 << 18,
    QuestRecordsExpired(MapleList16<QuestRecordExpired>) => 1 << 19,
    // EquipExtExpire 1 << 20
    //TODO this has to be optional in the all struct, bneed to implement this later 1 << somehow
    // this only affects the all struct, partial struct can opt to not encode 1 << it
    //WILD_HUNTER_INFO(WildHunterInfo) => 1 << 21,
    QuestCompleteOld(MapleList16<QuestCompleteOldInfo>) => 1 << 22,
    VisitorQuestLogInfo(MapleList16<VisitorQuestLogInfo>) => 1 << 23,

);

#[derive(MooplePacket, Debug)]
pub struct TempStatValue {
    pub n: u16,
    pub r: u32,
    pub t: MapleDurationMs32,
}

partial_data!(
    RemoteCharSecondaryStat,
    RemoteCharSecondaryStatFlags,
    u128,
    // n
    Speed(u8) => CharSecondaryStatFlags::Speed.bits(),
    // n
    ComboCounter(u8) => CharSecondaryStatFlags::ComboCounter.bits(),
    // r
    WeaponCharge(u32) => CharSecondaryStatFlags::WeaponCharge.bits(),
    // r
    Stun(u32) => CharSecondaryStatFlags::Stun.bits(),
    // r
    Darkness(u32) => CharSecondaryStatFlags::Darkness.bits(),
    // r
    Seal(u32) => CharSecondaryStatFlags::Seal.bits(),
    // r
    Weakness(u32) => CharSecondaryStatFlags::Weakness.bits(),
    // r
    Curse(u32) => CharSecondaryStatFlags::Curse.bits(),
    // n, r
    Poison((u16, u32)) => CharSecondaryStatFlags::Poison.bits(),
    // r
    ShadowPartner(u32) => CharSecondaryStatFlags::ShadowPartner.bits(),
    DarkSight(()) => CharSecondaryStatFlags::DarkSight.bits(),
    SoulArrow(()) => CharSecondaryStatFlags::SoulArrow.bits(),
    // n
    Morph(u16) => CharSecondaryStatFlags::Morph.bits(),
    // n
    Ghost(u16) => CharSecondaryStatFlags::Ghost.bits(),
    // r
    Attract(u32) => CharSecondaryStatFlags::Attract.bits(),
    // n
    SpiritJavelin(u32) => CharSecondaryStatFlags::SpiritJavelin.bits(),
    // r
    BanMap(u32) => CharSecondaryStatFlags::BanMap.bits(),
    // r
    Barrier(u32) => CharSecondaryStatFlags::Barrier.bits(),
    // r
    DojangShield(u32) => CharSecondaryStatFlags::DojangShield.bits(),
    // r
    ReverseInput(u32) => CharSecondaryStatFlags::ReverseInput.bits(),
    // n
    RespectPImmune(u32) => CharSecondaryStatFlags::RespectPImmune.bits(),
    // n
    RespectMImmune(u32) => CharSecondaryStatFlags::RespectMImmune.bits(),
    // n
    DefenseAtt(u32) => CharSecondaryStatFlags::DefenseAtt.bits(),
    // n
    DefenseState(u32) => CharSecondaryStatFlags::DefenseState.bits(),
    DojangBerserk(()) => CharSecondaryStatFlags::DojangBerserk.bits(),
    DojangInvincible(()) => CharSecondaryStatFlags::DojangInvincible.bits(),
    WindWalk(()) => CharSecondaryStatFlags::WindWalk.bits(),
    // r
    RepeatEffect(u32) => CharSecondaryStatFlags::RepeatEffect.bits(),
    // r
    StopPortion(u32) => CharSecondaryStatFlags::StopPortion.bits(),
    // r
    StopMotion(u32) => CharSecondaryStatFlags::StopMotion.bits(),
    // r
    Fear(u32) => CharSecondaryStatFlags::Fear.bits(),
    // r
    MagicShield(u32) => CharSecondaryStatFlags::MagicShield.bits(),
    Flying(()) => CharSecondaryStatFlags::Flying.bits(),
    // r
    Frozen(u32) => CharSecondaryStatFlags::Frozen.bits(),
    // r
    SuddenDeath(u32) => CharSecondaryStatFlags::SuddenDeath.bits(),
    // r
    FinalCut(u32) => CharSecondaryStatFlags::FinalCut.bits(),
    // n
    Cyclone(u8) => CharSecondaryStatFlags::Cyclone.bits(),
    Sneak(()) => CharSecondaryStatFlags::Sneak.bits(),
    MorewildDamageUp(()) => CharSecondaryStatFlags::MorewildDamageUp.bits(),
    // r
    Mechanic(u32) => CharSecondaryStatFlags::Mechanic.bits(),
    // r
    DarkAura(u32) => CharSecondaryStatFlags::DarkAura.bits(),
    // r
    BlueAura(u32) => CharSecondaryStatFlags::BlueAura.bits(),
    // r
    YellowAura(u32) => CharSecondaryStatFlags::YellowAura.bits(),
    BlessingArmor(()) => CharSecondaryStatFlags::BlessingArmor.bits(),
);

pub struct RemoteCharSecondaryStatExtra {
    pub defense_att: u8, //n
    pub defense_state: u8,
    /*
       TempStatBases, see SecondaryStat
    */
}

partial_data!(
    CharSecondaryStat,
    CharSecondaryStatFlags,
    u128,
    Pad(TempStatValue) =>  1 << 0,
    Pdd(TempStatValue) =>  1 << 1,
    Mad(TempStatValue) =>  1 << 2,
    Mdd(TempStatValue) =>  1 << 3,
    Acc(TempStatValue) =>  1 << 4,
    Evasion(TempStatValue) =>  1 << 5,
    Craft(TempStatValue) =>  1 << 6,
    Speed(TempStatValue) =>  1 << 7,
    Jump(TempStatValue) =>  1 << 8,
    ExtraMaxHp(TempStatValue) =>  1 << 0x5D,
    ExtraMaxMp(TempStatValue) =>  1 << 0x5E,
    ExtraPad(TempStatValue) =>  1 << 0x5F,
    ExtraPdd(TempStatValue) =>  1 << 0x60,
    ExtraMdd(TempStatValue) =>  1 << 0x61,
    MagicGuard(TempStatValue) =>  1 << 9,
    DarkSight(TempStatValue) =>  1 << 0xa,
    Booster(TempStatValue) =>  1 << 0xb,
    PowerGuard(TempStatValue) =>  1 << 0xc,
    Guard(TempStatValue) =>  1 << 0x62,
    SafetyDamage(TempStatValue) =>  1 << 0x63,
    SafetyAbsorb(TempStatValue) =>  1 << 0x64,
    MaxHp(TempStatValue) =>  1 << 0xd,
    MaxMp(TempStatValue) =>  1 << 0xe,
    Invincible(TempStatValue) =>  1 << 0xf,
    SoulArrow(TempStatValue) =>  1 << 0x10,
    Stun(TempStatValue) =>  1 << 0x11,
    Poison(TempStatValue) =>  1 << 0x12,
    Seal(TempStatValue) =>  1 << 0x13,
    Darkness(TempStatValue) =>  1 << 0x14,
    ComboCounter(TempStatValue) =>  1 << 0x15,
    WeaponCharge(TempStatValue) =>  1 << 0x16,
    DragonBlood(TempStatValue) =>  1 << 0x17,
    HolySymbol(TempStatValue) =>  1 << 0x18,
    MesoUp(TempStatValue) =>  1 << 0x19,
    ShadowPartner(TempStatValue) =>  1 << 0x1A,
    PickPocket(TempStatValue) =>  1 << 0x1B,
    MesoGuard(TempStatValue) =>  1 << 0x1C,
    Thaw(TempStatValue) =>  1 << 0x1D,
    Weakness(TempStatValue) =>  1 << 0x1E,
    Curse(TempStatValue) =>  1 << 0x1F,
    Slow(TempStatValue) =>  1 << 0x20, // Done
    Morph(TempStatValue) =>  1 << 0x21,
    Ghost(TempStatValue) =>  1 << 0x31, // ghost morph
    Regen(TempStatValue) =>  1 << 0x22, // recovery
    BasicStatUp(TempStatValue) =>  1 << 0x23, // maple warrior
    Stance(TempStatValue) =>  1 << 0x24, // Done
    SharpEyes(TempStatValue) =>  1 << 0x25, // Done
    ManaReflection(TempStatValue) =>  1 << 0x26, // Done
    Attract(TempStatValue) =>  1 << 0x27,  // seduce
    SpiritJavelin(TempStatValue) =>  1 << 0x28, // shadow claw
    Infinity(TempStatValue) =>  1 << 0x29, // Done
    Holyshield(TempStatValue) =>  1 << 0x2A, // Done
    HamString(TempStatValue) =>  1 << 0x2B, // Done
    Blind(TempStatValue) =>  1 << 0x2C, // Done
    Concentration(TempStatValue) =>  1 << 0x2D, // Done
    BanMap(TempStatValue) =>  1 << 0x2E,
    MaxLevelBuff(TempStatValue) =>  1 << 0x2F, // echo of hero
    Barrier(TempStatValue) =>  1 << 0x32,
    DojangShield(TempStatValue) =>  1 << 0x3E,
    ReverseInput(TempStatValue) =>  1 << 0x33, // confuse
    MesoUpByItem(TempStatValue) =>  1 << 0x30, // Done
    ItemUpByItem(TempStatValue) =>  1 << 0x34, // Done
    RespectPImmune(TempStatValue) =>  1 << 0x35,
    RespectMImmune(TempStatValue) =>  1 << 0x36,
    DefenseAtt(TempStatValue) =>  1 << 0x37,
    DefenseState(TempStatValue) =>  1 << 0x38,
    DojangBerserk(TempStatValue) =>  1 << 0x3B, // berserk fury
    DojangInvincible(TempStatValue) =>  1 << 0x3C, // divine body
    Spark(TempStatValue) =>  1 << 0x3D, // Done
    SoulMasterFinal(TempStatValue) =>  1 << 0x3F, // Done ?
    WindBreakerFinal(TempStatValue) =>  1 << 0x40, // Done ?
    ElementalReset(TempStatValue) =>  1 << 0x41, // Done
    WindWalk(TempStatValue) =>  1 << 0x42, // Done
    EventRate(TempStatValue) =>  1 << 0x43,
    ComboAbilityBuff(TempStatValue) =>  1 << 0x44, // aran combo
    ComboDrain(TempStatValue) =>  1 << 0x45, // Done
    ComboBarrier(TempStatValue) =>  1 << 0x46, // Done
    BodyPressure(TempStatValue) =>  1 << 0x47, // Done
    SmartKnockback(TempStatValue) =>  1 << 0x48, // Done
    RepeatEffect(TempStatValue) =>  1 << 0x49,
    ExpBuffRate(TempStatValue) =>  1 << 0x4A, // Done
    IncEffectHPPotion(TempStatValue) =>  1 << 0x39,
    IncEffectMPPotion(TempStatValue) =>  1 << 0x3A,
    StopPortion(TempStatValue) =>  1 << 0x4B,
    StopMotion(TempStatValue) =>  1 << 0x4C,
    Fear(TempStatValue) =>  1 << 0x4D, // debuff done
    EvanSlow(TempStatValue) =>  1 << 0x4E, // Done
    MagicShield(TempStatValue) =>  1 << 0x4F, // Done
    MagicResistance(TempStatValue) =>  1 << 0x50, // Done
    SoulStone(TempStatValue) =>  1 << 0x51,
    Flying(TempStatValue) =>  1 << 0x52,
    Frozen(TempStatValue) =>  1 << 0x53,
    AssistCharge(TempStatValue) =>  1 << 0x54,
    Enrage(TempStatValue) =>  1 << 0x55, //mirror imaging
    SuddenDeath(TempStatValue) =>  1 << 0x56,
    NotDamaged(TempStatValue) =>  1 << 0x57,
    FinalCut(TempStatValue) =>  1 << 0x58,
    ThornsEffect(TempStatValue) =>  1 << 0x59,
    SwallowAttackDamage(TempStatValue) =>  1 << 0x5A,
    MorewildDamageUp(TempStatValue) =>  1 << 0x5B,
    Mine(TempStatValue) =>  1 << 0x5C,
    Cyclone(TempStatValue) =>  1 << 0x65,
    SwallowCritical(TempStatValue) =>  1 << 0x67,
    SwallowMaxMP(TempStatValue) =>  1 << 0x67,
    SwallowDefence(TempStatValue) =>  1 << 0x68,
    SwallowEvasion(TempStatValue) =>  1 << 0x69,
    Conversion(TempStatValue) =>  1 << 0x6A,
    Revive(TempStatValue) =>  1 << 0x6B, // summon reaper buff
    Sneak(TempStatValue) =>  1 << 0x6C,
    Mechanic(TempStatValue) =>  1 << 0x6D,
    Aura(TempStatValue) =>  1 << 0x6E,
    DarkAura(TempStatValue) =>  1 << 0x6F,
    BlueAura(TempStatValue) =>  1 << 0x70,
    YellowAura(TempStatValue) =>  1 << 0x71,
    SuperBody(TempStatValue) =>  1 << 0x72, // body boost
    MorewildMaxHP(TempStatValue) =>  1 << 0x73,
    Dice(TempStatValue) =>  1 << 0x74,
    BlessingArmor(TempStatValue) =>  1 << 0x75, // Paladin Divine Shield
    DamR(TempStatValue) =>  1 << 0x76,
    TeleportMasteryOn(TempStatValue) =>  1 << 0x77,
    CombatOrders(TempStatValue) =>  1 << 0x78,
    Beholder(TempStatValue) =>  1 << 0x79,
    //TODO: 0x81 overflow u128 SummonBomb(TempStatValue) =>  1 << 0x81,
);

#[derive(MooplePacket)]
pub struct CharSecondaryStatExtra {
    // TODO option If any Swallow stat is set
    pub swallow_buff_time: u8,
    // TODO option if dice is set
    pub dice_info: [u32; 0x16],
    // TODO option if blessing armor is set
    pub blessing_armor_inc_pad: u32,
    /*
    TWO State values Decode/EncodeForClient(SecondaryStat::SecondaryStat)
            EnergyCharged = 0x7A, TwoState greater equal 10_000
        Dash_Speed = 0x7B,  // Two state not equal 0
        Dash_Jump = 0x7C, // Two state not equal 0
        RideVehicle = 0x7D, // Temp Stat long
        PartyBooster = 0x7E,/ Two state not equal 0
        GuidedBullet = 0x7F, // Temp Stat long
        Undead = 0x80, // Two state not equal 0
     */
}
/*
   Additional:
       SwallowCritical = If any

*/
