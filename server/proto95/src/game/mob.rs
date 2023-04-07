use moople_derive::MooplePacket;
use moople_packet::{
    maple_enum_code, maple_packet_enum, packet_opcode,
    proto::{
        option::MapleOption8,
        time::{MapleDurationMs16, MapleDurationMs32},
        CondOption, MapleList32, PacketWrapped, partial::PartialFlag,
    }, partial_data,
};

use crate::{
    id::{ItemId, SkillId},
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, movement::{MovePassivePath, MovePath}, FootholdId, TagPoint, Vec2},
};

use super::ObjectId;

pub type MobId = u32;

#[derive(MooplePacket, Debug)]
pub struct TempStatValue {
    pub n: u16,
    pub r: u32,
    pub t: MapleDurationMs16,
}

#[derive(MooplePacket, Debug)]
pub struct BurnedInfo {
    pub char_id: CharacterId,
    pub skill_id: SkillId,
    pub n_dmg: u32,
    pub interval: MapleDurationMs32,
    pub end: MapleDurationMs32,
    pub dot_count: u32,
}

partial_data!(
    MobTemporaryStat,
    MobTemporaryStatFlags,
    u128,
    Pad(TempStatValue) => 1 << 0,
    Pdr(TempStatValue) => 1 << 1,
    Mad(TempStatValue) => 1 << 2,
    Mdr(TempStatValue) => 1 << 3,
    Acc(TempStatValue) => 1 << 4,
    Eva(TempStatValue) => 1 << 5,
    Speed(TempStatValue) => 1 << 6,
    Stun(TempStatValue) => 1 << 7,
    Freeze(TempStatValue) => 1 << 8,
    Poison(TempStatValue) => 1 << 9,
    Seal(TempStatValue) => 1 << 10,
    Darkness(TempStatValue) => 1 << 11,
    PowerUp(TempStatValue) => 1 << 12,
    MagicUp(TempStatValue) => 1 << 13,
    PGuardUp(TempStatValue) => 1 << 14,
    MGuardUp(TempStatValue) => 1 << 15,
    Doom(TempStatValue) => 1 << 16,
    Web(TempStatValue) => 1 << 17,
    PImmune(TempStatValue) => 1 << 18,
    MImmune(TempStatValue) => 1 << 19,
    HardSkin(TempStatValue) => 1 << 21,
    Ambush(TempStatValue) => 1 << 22,
    Venom(TempStatValue) => 1 << 24,
    Blind(TempStatValue) => 1 << 25,
    SealSkill(TempStatValue) => 1 << 26,
    Dazzle(TempStatValue) => 1 << 28,
    PCounter(TempStatValue) => 1 << 29,
    MCounter(TempStatValue) => 1 << 30,
    RiseByToss(TempStatValue) => 1 << 32,
    BodyPressure(TempStatValue) => 1 << 33,
    Weakness(TempStatValue) => 1 << 34,
    TimeBomb(TempStatValue) => 1 << 35,
    Showdown(TempStatValue) => 1 << 20,
    MagicCrash(TempStatValue) => 1 << 36,
    DamagedElemAttr(TempStatValue) => 1 << 23,
    HealByDamage(TempStatValue) => 1 << 37,
    Burned(MapleList32<BurnedInfo>) => 1 << 27,
);

#[derive(MooplePacket, Debug)]
pub struct MobTemporaryStatTail {
    // If PCounter is set
    pub w_pcounter: u32,
    // If PCounter is set
    pub w_mcounter: u32,
    // If either counter is set
    pub counter_prob: u32,
    // If disable is set
    pub invincible: bool,
    pub disable: bool,
}

pub type PartialMobTemporaryStat = PartialFlag<(), MobTemporaryStatPartial>;

//TODO figure out what the u32 is, summon id?

maple_packet_enum!(
    MobSummonType,
    i8,
    Effect(u32) => 0,
    Normal(()) => -1,
    Regen(()) => -2,
    Revived(u32) => -3,
    Suspended(()) => -4,
    Delay(()) => -5,
);

maple_enum_code!(CarnivalTeam, u8, None = 0xff, Blue = 0, Red = 1);

#[derive(MooplePacket, Debug)]
pub struct MobInitData {
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub origin_fh: FootholdId,
    pub summon_type: MobSummonType,
    pub carnival_team: CarnivalTeam,
    pub effect_id: u32,
    pub phase: u32,
}

#[derive(MooplePacket, Debug)]
pub struct MobEnterFieldResp {
    pub id: ObjectId,
    pub calc_dmg_index: u8,
    pub tmpl_id: MobId,
    pub stats: PartialMobTemporaryStat,
    pub init_data: MobInitData,
}
packet_opcode!(MobEnterFieldResp, SendOpcodes::MobEnterField);

maple_packet_enum!(
    MobLeaveType,
    u8,
    RemainHp(()) => 0,
    Etc(()) => 1, //Fadeout?
    SelfDestruct(()) => 2,
    DestructByMiss(()) => 3,
    Swallow(CharacterId) => 4,
    SummonTimeout(()) => 5
);

#[derive(MooplePacket, Debug)]
pub struct MobLeaveFieldResp {
    pub id: ObjectId,
    pub leave_type: MobLeaveType,
}
packet_opcode!(MobLeaveFieldResp, SendOpcodes::MobLeaveField);

#[derive(MooplePacket, Debug)]
pub struct LocalMobData {
    pub calc_damage_index: u8,
    pub tmpl_id: MobId,
    pub stats: PartialMobTemporaryStat,
    // TODO: Only when mob is not existing for the client pub init_data: MobInitData,
}

fn has_local_mob_data(level: &u8) -> bool {
    *level > 0
}

//
#[derive(MooplePacket, Debug)]
pub struct MobChangeControllerResp {
    // // 0 = None | 1 = Control | 2 = Aggro
    pub level: u8,
    //TODO only if level != 0
    //pub seed: CrcSeed,
    pub id: ObjectId,
    #[pkt(if(field = "level", cond = "has_local_mob_data"))]
    pub local_mob_data: CondOption<LocalMobData>,
}
packet_opcode!(MobChangeControllerResp, SendOpcodes::MobChangeController);

#[derive(Debug, Clone, Copy)]
pub struct FlyTargetPoint(pub Option<TagPoint>);

pub const NONE_FLY_TARGET_POS: TagPoint = TagPoint {
    x: 0xffddcc,
    y: 0xffddcc,
};

impl PacketWrapped for FlyTargetPoint {
    type Inner = TagPoint;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0.unwrap_or(NONE_FLY_TARGET_POS)
    }

    fn packet_from(v: Self::Inner) -> Self {
        match v {
            NONE_FLY_TARGET_POS => Self(None),
            _ => Self(Some(v)),
        }
    }
}

#[derive(MooplePacket, Debug)]
pub struct MobMoveResp {
    pub id: ObjectId,
    pub not_force_landing: bool,
    pub not_change_action: bool,
    pub next_attack_possible: bool,
    pub action_dir: u8,
    pub data: u32,
    pub multi_target: MapleList32<TagPoint>,
    pub rand_time: MapleList32<u32>,
    pub move_path: MovePath,
}
packet_opcode!(MobMoveResp, SendOpcodes::MobMove);

#[derive(MooplePacket, Debug)]
pub struct MobMoveReq {
    pub id: ObjectId,
    pub ctrl_sn: u16,
    pub flag: u8, // bSomeRand | 4 * (bRushMove | 2 * (bRiseByToss | 2 * nMobCtrlState));
    pub action_dir: u8,
    pub data: u32,
    pub multi_target: MapleList32<TagPoint>,
    pub rand_time: MapleList32<u32>,
    pub move_flags: u8,
    pub hacked_code: u32,
    pub fly_target_pos: FlyTargetPoint,
    pub hacked_code_crc: u32,
    pub move_path: MovePassivePath,
    pub chasing: bool,
    pub has_target: bool,
    pub chasing2: bool,
    pub chasing_hack: bool,
    pub chase_duration: u32,
}
packet_opcode!(MobMoveReq, RecvOpcodes::MobMove);

#[derive(MooplePacket, Debug)]
pub struct MobMoveCtrlAckResp {
    pub id: ObjectId,
    pub ctrl_sn: u16,
    pub next_atk_possible: bool,
    pub mp: u16,
    pub skill_id: u8,
    pub slv: u8,
}
packet_opcode!(MobMoveCtrlAckResp, SendOpcodes::MobCtrlAck);

#[derive(MooplePacket, Debug)]
pub struct MobDamagedResp {
    pub id: ObjectId,
    pub ty: u8,
    pub dec_hp: u32,
    // If template->DamagedByMob !=  false
    pub hp: u32,
    pub max_hp: u32,
}
packet_opcode!(MobDamagedResp, SendOpcodes::MobDamaged);

#[derive(MooplePacket, Debug)]
pub struct MobOnStatSet {
    pub id: ObjectId,
    //TODO if (MobStat::IsMovementAffectingStat(uFlag: var_44) != 0 && this->m_bDoomReserved != 0)
    pub stats: PartialMobTemporaryStat,
}
packet_opcode!(MobOnStatSet, SendOpcodes::MobStatSet);

#[derive(MooplePacket, Debug)]
pub struct MobOnStatReset {
    pub id: ObjectId,
    //TODO
}
packet_opcode!(MobOnStatReset, SendOpcodes::MobStatReset);

#[derive(MooplePacket, Debug)]
pub struct MobOnSuspendReset {
    pub id: ObjectId,
    pub suspend_reset: bool,
}
packet_opcode!(MobOnSuspendReset, SendOpcodes::MobSuspendReset);

#[derive(MooplePacket, Debug)]
pub struct MobAffectedResp {
    pub id: ObjectId,
    pub skill_id: u32,
    pub start_delay: MapleDurationMs16,
}
packet_opcode!(MobAffectedResp, SendOpcodes::MobAffected);

#[derive(MooplePacket, Debug)]
pub struct MobSpecialEffectBySkillResp {
    pub id: ObjectId,
    pub skill_id: u32,
    pub char_id: CharacterId,
    pub start_delay: MapleDurationMs16,
}
packet_opcode!(
    MobSpecialEffectBySkillResp,
    SendOpcodes::MobSpecialEffectBySkill
);

#[derive(MooplePacket, Debug)]
pub struct MobHPIndicatorResp {
    pub id: ObjectId,
    pub hp_perc: u8,
}
packet_opcode!(MobHPIndicatorResp, SendOpcodes::MobHPIndicator);

#[derive(MooplePacket, Debug)]
pub struct MobCatchEffectResp {
    pub id: ObjectId,
    pub success: bool,
    pub delay: u8, // TODO
}
packet_opcode!(MobCatchEffectResp, SendOpcodes::MobCatchEffect);

#[derive(MooplePacket, Debug)]
pub struct MobEffectByItem {
    pub id: ObjectId,
    pub item: ItemId,
    pub success: bool,
}
packet_opcode!(MobEffectByItem, SendOpcodes::MobEffectByItem);

#[derive(MooplePacket, Debug)]
pub struct MobSpeakingResp {
    pub id: ObjectId,
    pub speak_info: u32,
    pub speech: u32,
}
packet_opcode!(MobSpeakingResp, SendOpcodes::MobSpeaking);

#[derive(MooplePacket, Debug)]
pub struct MobIncChargeCount {
    pub id: ObjectId,
    pub mob_charge_count: u32,
    pub attack_ready: bool,
}
packet_opcode!(MobIncChargeCount, SendOpcodes::MobChargeCount);

#[derive(MooplePacket, Debug)]
pub struct MobSkillDelayResp {
    pub id: ObjectId,
    pub skill_delay: MapleDurationMs32,
    pub skill_id: u32,
    pub slv: u32,
    pub skill_option: u32,
}
packet_opcode!(MobSkillDelayResp, SendOpcodes::MobSkillDelay);

#[derive(MooplePacket, Debug)]
pub struct MobEscortPathResp {
    pub id: ObjectId,
    pub u1: u32,
    pub u2: u32,
    pub u3: u32,
    pub u4: u32,
    pub u5: u32,
    pub u6: u32,
    //TODO
}
packet_opcode!(MobEscortPathResp, SendOpcodes::MobRequestResultEscortInfo);

#[derive(MooplePacket, Debug)]
pub struct MobEscortStopSayResp {
    pub id: ObjectId,
    pub stop_escort: MapleDurationMs32,
    pub chat_ballon: u32,
    pub chat_msg: MapleOption8<String>,
}
packet_opcode!(
    MobEscortStopSayResp,
    SendOpcodes::MobEscortStopEndPermmision
);

#[derive(MooplePacket, Debug)]
pub struct MobEscortReturnBeforeResp {
    pub id: ObjectId,
    pub u: u32,
}
packet_opcode!(MobEscortReturnBeforeResp, SendOpcodes::MobEscortStopSay);

#[derive(MooplePacket, Debug)]
pub struct MobNextAttackResp {
    pub id: ObjectId,
    pub force_atk_id: u32,
}
packet_opcode!(MobNextAttackResp, SendOpcodes::MobNextAttack);

#[derive(MooplePacket, Debug)]
pub struct MobAttackedByMobResp {
    pub id: ObjectId,
    pub mob_atk_id: u8,
    pub dmg: u32,
    //  Only read if
    pub mob_tmpl_id: MobId,
    pub left: bool,
}
packet_opcode!(MobAttackedByMobResp, SendOpcodes::MobAttackedByMob);

#[derive(MooplePacket, Debug)]
pub struct MobDropPickUpReq {
    pub mob_id: ObjectId,
    pub drop_id: ObjectId,
}
packet_opcode!(MobDropPickUpReq, RecvOpcodes::MobDropPickUpRequest);

#[cfg(test)]
mod tests {
    use moople_packet::DecodePacket;

    use super::MobMoveReq;

    #[test]
    fn decode_move_req() {
        let data = [
            227, 0, 4, 0, 0, 0, 1, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
            204, 221, 255, 0, 204, 221, 255, 0, 13, 140, 181, 156, 211, 0, 231, 255, 0, 0, 0, 0, 1,
            0, 0, 1, 231, 255, 43, 0, 0, 0, 46, 0, 0, 0, 0, 0, 2, 56, 4, 0, 211, 0, 231, 255, 0, 1,
            231, 255, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let move_data = MobMoveReq::decode_from_data_complete(&data[2..]).unwrap();

        dbg!(move_data);
    }
}
