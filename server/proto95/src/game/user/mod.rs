pub mod remote;

use bitflags::bitflags;
use moople_derive::{MooplePacket, MoopleEncodePacket};
use moople_packet::{
    maple_packet_enum, mark_maple_bit_flags, packet_opcode,
    proto::{
        option::MapleOption8,
        time::{MapleDurationMs16, MapleExpiration, Ticks},
        CondOption, MapleList16, MapleList8, PacketWrapped,
    },
    DecodePacket, MaplePacketReader, NetError, NetResult,
};

use crate::{
    id::{ItemId, MapId, SkillId},
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{movement::MovePath, TagPoint, Vec2},
};

use super::{mob::MobId, ObjectId};

#[derive(MooplePacket, Debug)]
pub struct UserDropMoneyReq {
    pub ticks: Ticks,
    pub money: u32,
}
packet_opcode!(UserDropMoneyReq, RecvOpcodes::UserDropMoneyRequest);

#[derive(MooplePacket, Debug)]
pub struct UserDropPickUpReq {
    pub field_key: u8,
    pub ticks: Ticks,
    pub point: Vec2,
    pub drop_id: ObjectId,
    pub crc: u32,
}
packet_opcode!(UserDropPickUpReq, RecvOpcodes::DropPickUpRequest);

#[derive(MooplePacket, Debug)]
pub struct UserPortalScriptReq {
    pub field_key: u8,
    pub portal: String,
    pub pos: Vec2,
}
packet_opcode!(UserPortalScriptReq, RecvOpcodes::UserPortalScriptRequest);

fn is_not_empty(s: &str) -> bool {
    !s.is_empty()
}

#[derive(MooplePacket, Debug)]
pub struct UserTransferFieldReq {
    pub field_key: u8,
    pub target_field: MapId,
    pub portal: String,
    #[pkt(if(field = "portal", cond = "is_not_empty"))]
    pub target_pos: CondOption<Vec2>,
    pub unknown: u8,
    pub premium: bool,
    pub chase_target_pos: MapleOption8<TagPoint>,
}
packet_opcode!(UserTransferFieldReq, RecvOpcodes::UserTransferFieldRequest);

#[derive(MooplePacket, Debug)]
pub struct UserMoveReq {
    // DR 1-4?
    pub u1: u32,
    pub u2: u32,
    pub field_key: u8,
    pub u3: u32,
    pub u4: u32,
    pub field_crc: u32,
    pub rand: u32,
    pub movement_crc: u32,
    pub move_path: MovePath,
}
packet_opcode!(UserMoveReq, RecvOpcodes::UserMove);

#[derive(MooplePacket, Debug)]
pub struct UserStatChangeReq {
    pub ticks: Ticks,
    // Constant 5120
    pub flags: u32,
    pub hp: u16,
    pub mp: u16,
    pub option: u8,
}
packet_opcode!(UserStatChangeReq, RecvOpcodes::UserChangeStatRequest);

#[derive(MooplePacket, Debug)]
pub struct UserHitKnockback {
    pub powerguard: bool,
    pub mob_id: ObjectId,
    pub hit_action: u8,
    pub mob_pos: Vec2,
    pub user_pos: Vec2,
}

#[derive(MooplePacket, Debug)]
pub struct UserHitReq {
    pub damaged_ticks: Ticks,
    pub mob_atk_idx: u8, //todo: 0xfe is no atk => 1,4,2
    pub magic_elem_attr: u8,
    pub dmg_internal: u32,
    pub mob_tmpl_id: MobId,
    pub mob_id: ObjectId,
    pub left: bool,
    pub reflect: u8,
    pub guard: bool,
    pub knockback: u8,
    //TODO: If knockback | reflect  > 0 => UserHitKnockback
    //non 0xfe end here
    pub unknown: u8,
}
packet_opcode!(UserHitReq, RecvOpcodes::UserHit);

bitflags! {
    #[derive(Debug)]
    pub struct AttackFlags : u8 {
        const FINAL_ATTACK = 0x01;
        const SHADOW_PARTNER = 0x04;
        const SERIAL_ATTACK = 0x08;
        const SPARK = 0x10;
    }
}

mark_maple_bit_flags!(AttackFlags);

#[derive(Debug, MooplePacket)]
pub struct DrCheckData {
    data: [u8; 8],
}

#[derive(Debug, Clone)]
pub struct HitTargetCount {
    pub hits: u8,
    pub targets: u8,
}

impl PacketWrapped for HitTargetCount {
    type Inner = u8;

    fn packet_into_inner(&self) -> Self::Inner {
        (self.targets << 4) | (self.hits & 0xF)
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self {
            targets: v >> 4,
            hits: v & 0xF,
        }
    }
}

#[derive(Debug)]
pub struct ActionDir {
    pub left: bool,
    pub action: u16,
}

impl PacketWrapped for ActionDir {
    type Inner = u16;

    fn packet_into_inner(&self) -> Self::Inner {
        (self.left as u16) << 15 | (self.action & 0x7FFF)
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self {
            left: v >> 15 == 1,
            action: v & 0x7FFF,
        }
    }
}

#[derive(Debug)]
pub struct ForeActionDir {
    pub left: bool,
    pub action: u8,
}

impl PacketWrapped for ForeActionDir {
    type Inner = u8;

    fn packet_into_inner(&self) -> Self::Inner {
        (self.left as u8) << 7 | (self.action & 0x7F)
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self {
            left: v >> 7 == 1,
            action: v & 0x7F,
        }
    }
}

/*#[derive(PackedStruct, Debug, Clone, Copy)]
#[packed_struct(bit_numbering = "msb0")]
pub struct HitTargetCount {
    target_count: Integer<u8, packed_bits::Bits<4>>,
    hit_count: Integer<u8, packed_bits::Bits<4>>,
}*/


#[derive(Debug, MoopleEncodePacket)]
pub struct AttackTargetInfo {
    pub mob_id: ObjectId,
    pub hit_action: u8,
    pub fore_action: ForeActionDir,
    pub frame_id: u8,
    pub calc_damage_stat_ix: u8,
    pub pos_a: Vec2,
    pub pos_b: Vec2,
    pub delay: u16,
    pub hits: Vec<u32>,
    pub mob_crc: u32,
}

impl AttackTargetInfo {
    pub fn decode(
        pr: &mut MaplePacketReader<'_>,
        targets: usize,
        hits: usize,
    ) -> Result<Vec<Self>, NetError> {
        (0..targets)
            .map(|_| {
                Ok(AttackTargetInfo {
                    mob_id: ObjectId::decode_packet(pr)?,
                    hit_action: u8::decode_packet(pr)?,
                    fore_action: ForeActionDir::decode_packet(pr)?,
                    frame_id: u8::decode_packet(pr)?,
                    calc_damage_stat_ix: u8::decode_packet(pr)?,
                    pos_a: Vec2::decode_packet(pr)?,
                    pos_b: Vec2::decode_packet(pr)?,
                    delay: u16::decode_packet(pr)?,
                    hits: u32::decode_packet_n(pr, hits)?,
                    mob_crc: u32::decode_packet(pr)?,
                })
            })
            .collect()
    }
}

pub trait AttackInfo {
    fn targets(&self) -> usize;
    fn hits(&self) -> usize;
}

impl AttackInfo for MeleeAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

impl<'de, Info, Extra> DecodePacket<'de> for AttackReq<Info, Extra>
where
    Info: AttackInfo + DecodePacket<'de>,
    Extra: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let info = Info::decode_packet(pr)?;
        let targets = AttackTargetInfo::decode(pr, info.targets(), info.hits())?;
        let extra = Extra::decode_packet(pr)?;
        Ok(Self {
            info,
            targets,
            extra,
        })
    }
}

#[derive(Debug, MoopleEncodePacket)]
pub struct AttackReq<Info: AttackInfo, Extra> {
    pub info: Info,
    pub targets: Vec<AttackTargetInfo>,
    pub extra: Extra,
}


#[derive(Debug, MooplePacket)]
pub struct MeleeAttackInfo {
    pub portal: u8,            // Field key
    pub hit_target_count: DrHitTargetCount,
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rnd: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    //TODO if skill_id is keydown/charge skill
    //key_down_dur: u32,
    pub attack_flags: AttackFlags,
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32,
    //Special bmage handling
    pub affected_area_id: u32,
}


#[derive(Debug, MooplePacket)]
pub struct MeleeAttackTail {
    pub pos: Vec2,
    // If skillid == 14111006
    //pub grenade_pos: MapleOption8<Vec2>
}

#[derive(Debug, MooplePacket)]
pub struct DrCtx {
    pub dr0: u32,
    pub dr1: u32,
    pub dr2: u32,
    pub dr3: u32,
}

#[derive(Debug)]
pub struct DrHitTargetCount {
    pub dr: DrCtx,
    pub hit_target_count: HitTargetCount,
}

impl PacketWrapped for DrHitTargetCount {
    type Inner = (u32, u32, HitTargetCount, u32, u32);

    fn packet_into_inner(&self) -> Self::Inner {
        (
            self.dr.dr0,
            self.dr.dr1,
            self.hit_target_count.clone(),
            self.dr.dr2,
            self.dr.dr3,
        )
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self {
            dr: DrCtx {
                dr0: v.0,
                dr1: v.1,
                dr2: v.3,
                dr3: v.4,
            },
            hit_target_count: v.2,
        }
    }
}

pub type UserMeleeAttackReq = AttackReq<MeleeAttackInfo, MeleeAttackTail>;
packet_opcode!(UserMeleeAttackReq, RecvOpcodes::UserMeleeAttack);

#[derive(MooplePacket, Debug)]
pub struct SkillInfoCrc {
    pub crc1: u32,
    pub crc2: u32
}


#[derive(MooplePacket, Debug)]
pub struct ValWithCrc {
    pub val: u32,
    pub crc: u32
}


#[derive(Debug, MooplePacket)]
pub struct MagicAttackInfo {
    pub portal: u8,            // Field key
    pub hit_target_count: DrHitTargetCount,
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rng: ValWithCrc,
    pub dr_ctx: DrCtx,
    pub init: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    //TODO if skill_id is keydown/charge skill
    //key_down_dur: u32,
    pub attack_flags: u8,
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32,
    pub phase: u32,
}

#[derive(Debug, MooplePacket)]
pub struct MagicAttackTail {
    pub pos: Vec2,
    pub dragon_pos: MapleOption8<Vec2>,
}

impl AttackInfo for MagicAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

pub type UserMagicAttackReq = AttackReq<MagicAttackInfo, MagicAttackTail>;
packet_opcode!(UserMagicAttackReq, RecvOpcodes::UserMagicAttack);

#[derive(Debug, MooplePacket)]
pub struct BodyAttackInfo {
    pub portal: u8,            // Field key
    pub hit_target_count: DrHitTargetCount,
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rnd: ValWithCrc,
    pub dr_ctx: DrCtx,
    pub init: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    pub attack_flags: u8, //always 0
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32,
    pub id: u32,// dwid?
}

#[derive(Debug, MooplePacket)]
pub struct BodyAttackTail {
    pub pos: Vec2,
}

impl AttackInfo for BodyAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

pub type UserBodyAttackReq = AttackReq<BodyAttackInfo, BodyAttackTail>;
packet_opcode!(UserBodyAttackReq, RecvOpcodes::UserBodyAttack);

bitflags! {
    #[derive(Debug)]
    pub struct ShotAttackFlags : u8 {
        const SOUL_ARROW = 0x02;
        const MORTAL_BLOW = 0x04;
        const SHADOW_PARTNER = 0x08;
        const SERIAL_ATTACK = 0x20;
        const SPIRIT_JAVELIN = 0x40;
        const SPARK = 0x80;
    }
}
mark_maple_bit_flags!(ShotAttackFlags);

#[derive(Debug, MooplePacket)]
pub struct ShotAttackInfo {
    pub portal: u8,            // Field key
    pub hit_target_count: DrHitTargetCount,
    pub skill_id: SkillId,
    pub combat_orders: u8,
    pub rnd: ValWithCrc,
    pub dr_ctx: DrCtx,
    pub init: ValWithCrc,
    pub skill_crc: SkillInfoCrc,
    //TODO if skill_id is keydown/charge skill
    //key_down_dur: u32,
    pub attack_flags: ShotAttackFlags,
    pub jablin: bool,// v291->m_bNextShootExJablin && CUserLocal::CheckApplyExJablin(v291, pSkill, nAttackAction)
    pub action_dir: ActionDir,
    pub unknown_crc_1: u32,
    pub attack_action_type: u8,
    pub atk_speed: u8,
    pub atk_time: u32,
    pub id: u32,// dwid?
    pub bullet_slot: u16,
    pub cash_bullet_slot: u16,
    pub shot_range: u8,
    // If spirt_javeling && skill consuming bullet
    pub spirit_javelin_bullet_id: ItemId,
}

#[derive(Debug, MooplePacket)]
pub struct ShootAttackTail {
    pub pos: Vec2,
    // If wildhunter
    // body_rel_y_move: u16
    pub atk_pos: Vec2,
    // If skill_id == 15111006 (spark) -> reserve_spark_delay: u32

}

impl AttackInfo for ShotAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hit_target_count.hits as usize
    }
}

pub type UserShotAttackReq = AttackReq<ShotAttackInfo, ShootAttackTail>;
packet_opcode!(UserShotAttackReq, RecvOpcodes::UserShootAttack);



#[derive(MooplePacket, Debug)]
pub struct UserSkillUpReq {
    pub ticks: Ticks,
    pub skill_id: SkillId,
}
packet_opcode!(UserSkillUpReq, RecvOpcodes::UserSkillUpRequest);

#[derive(MooplePacket, Debug)]
pub struct UserSkillUseReq {
    pub ticks: Ticks,
    pub skill_id: SkillId,
    #[pkt(if(field = "skill_id", cond = "SkillId::is_anti_repeat_buff_skill"))]
    pub pos: CondOption<Vec2>,
    #[pkt(if(field = "skill_id", cond = "SkillId::is_spirit_javelin"))]
    pub spirit_javelin_item: CondOption<ItemId>,
    // If has affected -> u8 affectedMemberBitmap, this is tricky
    // cause no way to detect if affectedMemberBitmap is not encoded
    // if skill id == 2311001 -> dispel => Delay
    pub affected_mobs: MapleList8<ObjectId>,
    pub delay: MapleDurationMs16,
}
packet_opcode!(UserSkillUseReq, RecvOpcodes::UserSkillUseRequest);

#[derive(MooplePacket, Debug)]
pub struct UpdatedSkillRecord {
    pub id: SkillId,
    pub level: u32,
    pub master_level: u32,
    pub expiration: MapleExpiration,
}

#[derive(MooplePacket, Debug)]
pub struct ChangeSkillRecordResp {
    pub reset_excl: bool,
    pub skill_records: MapleList16<UpdatedSkillRecord>,
    pub updated_secondary_stat: bool,
}
packet_opcode!(ChangeSkillRecordResp, SendOpcodes::ChangeSkillRecordResult);

#[derive(MooplePacket, Debug)]
pub struct CharGivePopularityResult {
    pub code: u8,     // Code 0-5, code 0 and 5 are success
    pub name: String, // Code 0,5
    pub up: bool,     // Code 0,5
    pub pop: u32,     // Only used when code == 0
}
packet_opcode!(CharGivePopularityResult, SendOpcodes::GivePopularityResult);

maple_packet_enum!(
    DropPickUpMsg,
    u8,
    // item, quantity
    PickUp((ItemId, u32)) => 0,
    PickUp1((u8, ItemId, u16)) => 1,// What's that?
    PickUpEq(ItemId) => 0,
);

maple_packet_enum!(
    MessageResp,
    u8,
    DropPickUp(DropPickUpMsg) => 0,
);

packet_opcode!(MessageResp, SendOpcodes::Message);

#[cfg(test)]
mod tests {
    use moople_packet::DecodePacket;

    use crate::game::user::{UserMagicAttackReq, UserMeleeAttackReq};

    use super::UserHitReq;

    #[test]
    fn user_hit_req() {
        let data = [
            52, 0, 232, 211, 221, 3, 255, 0, 1, 0, 0, 0, 160, 134, 1, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let hit = UserHitReq::decode_from_data_complete(&data[2..]).unwrap();
        dbg!(hit);
    }

    #[test]
    fn user_melee_atk() {
        let data = [
            47, 0, 0, 55, 29, 230, 255, 31, 127, 55, 139, 17, 59, 132, 173, 136, 215, 117, 129,
            160, 0, 0, 0, 0, 0, 117, 25, 20, 0, 125, 21, 153, 165, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
            128, 194, 165, 88, 168, 1, 4, 228, 215, 221, 3, 0, 0, 0, 0, 18, 0, 0, 0, 7, 128, 7, 5,
            187, 2, 139, 1, 187, 2, 139, 1, 137, 1, 10, 0, 0, 0, 225, 199, 157, 247, 241, 2, 139,
            1,
        ];
        let atk = UserMeleeAttackReq::decode_from_data_complete(&data[2..]).unwrap();
        dbg!(atk);
    }

    #[test]
    fn user_magic_atk() {
        // No target
        let data = [
            49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 1, 43, 26, 230, 255, 27, 25, 230, 255,
            232, 3, 0, 0, 0, 70, 110, 165, 0, 160, 86, 130, 241, 255, 255, 255, 255, 35, 21, 76,
            174, 11, 150, 35, 251, 71, 25, 230, 255, 211, 225, 182, 41, 80, 154, 220, 44, 238, 211,
            198, 119, 238, 211, 198, 119, 0, 37, 0, 194, 165, 88, 168, 1, 6, 45, 220, 157, 2, 0, 0,
            0, 0, 58, 1, 18, 1, 0,
        ];
        let atk = UserMagicAttackReq::decode_from_data_complete(&data[2..]).unwrap();
        dbg!(atk);

        // 1 targets
        let data = [
            49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 17, 43, 26, 230, 255, 27, 25, 230, 255,
            232, 3, 0, 0, 0, 194, 254, 171, 2, 234, 36, 91, 175, 255, 255, 255, 255, 35, 21, 76,
            174, 11, 150, 35, 251, 71, 25, 230, 255, 140, 87, 162, 88, 61, 93, 199, 49, 238, 211,
            198, 119, 238, 211, 198, 119, 0, 37, 128, 194, 165, 88, 168, 1, 6, 139, 155, 158, 2, 0,
            0, 0, 0, 14, 0, 0, 0, 7, 128, 6, 5, 187, 2, 139, 1, 187, 2, 139, 1, 8, 2, 10, 0, 0, 0,
            225, 199, 157, 247, 29, 3, 139, 1, 0,
        ];
        let atk = UserMagicAttackReq::decode_from_data_complete(&data[2..]).unwrap();
        dbg!(atk);
    }
    /*
    Unhandled packet: [49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 1, 43, 26, 230, 255, 27, 25, 230, 255, 232, 3, 0, 0, 0, 70, 110, 165, 0, 160, 86, 130, 241, 255, 255, 255, 255, 35, 21, 76, 174, 11, 150, 35, 251, 71, 25, 230, 255, 211, 225, 182, 41, 80, 154, 220, 44, 238, 211, 198, 119, 238, 211, 198, 119, 0, 37, 0, 194, 165, 88, 168, 1, 6, 45, 220, 157, 2, 0, 0, 0, 0, 58, 1, 18, 1, 0]
    [49, 0, 0, 192, 92, 220, 251, 95, 23, 76, 174, 17, 43, 26, 230, 255, 27, 25, 230, 255, 232, 3, 0, 0, 0, 194, 254, 171, 2, 234, 36, 91, 175, 255, 255, 255, 255, 35, 21, 76, 174, 11, 150, 35, 251, 71, 25, 230, 255, 140, 87, 162, 88, 61, 93, 199, 49, 238, 211, 198, 119, 238, 211, 198, 119, 0, 37, 128, 194, 165, 88, 168, 1, 6, 139, 155, 158, 2, 0, 0, 0, 0, 14, 0, 0, 0, 7, 128, 6, 5, 187, 2, 139, 1, 187, 2, 139, 1, 8, 2, 10, 0, 0, 0, 225, 199, 157, 247, 29, 3, 139, 1, 0]
     */

    // use skill: [103, 0, 67, 16, 159, 2, 234, 3, 0, 0, 1, 0, 0]
}
