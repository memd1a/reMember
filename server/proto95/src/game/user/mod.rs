pub mod remote;

use bitflags::bitflags;
use bytes::BufMut;
use moople_derive::MooplePacket;
use moople_packet::{
    mark_maple_bit_flags, packet_opcode,
    proto::{option::MapleOption8, time::Ticks, CondOption, MapleWrapped},
    DecodePacket, EncodePacket, MaplePacketReader, MaplePacketWriter, NetError, NetResult,
    PacketLen,
};

use crate::{
    id::{MapId, SkillId},
    recv_opcodes::RecvOpcodes,
    shared::{char::MobId, movement::MovePath, TagPoint, Vec2},
};

use super::ObjectId;

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

#[derive(Debug)]
pub struct HitTargetCount {
    pub hits: u8,
    pub targets: u8,
}

impl MapleWrapped for HitTargetCount {
    type Inner = u8;

    fn maple_into_inner(&self) -> Self::Inner {
        (self.targets << 4) | (self.hits & 0xF)
    }

    fn maple_from(v: Self::Inner) -> Self {
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

impl MapleWrapped for ActionDir {
    type Inner = u16;

    fn maple_into_inner(&self) -> Self::Inner {
        (self.left as u16) << 15 | (self.action & 0x7FFF)
    }

    fn maple_from(v: Self::Inner) -> Self {
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

impl MapleWrapped for ForeActionDir {
    type Inner = u8;

    fn maple_into_inner(&self) -> Self::Inner {
        (self.left as u8) << 7 | (self.action & 0x7F)
    }

    fn maple_from(v: Self::Inner) -> Self {
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

#[derive(Debug, MooplePacket)]
pub struct MeleeAttackInfo {
    portal: u8,            // Field key
    dr_check: DrCheckData, // dr0,1
    hit_target_count: HitTargetCount,
    dr_check2: DrCheckData, // dr23
    skill_id: SkillId,
    combat_orders: u8,
    rnd: u32,
    crc: u32,
    skill_crc1: u32,
    skill_crc2: u32,
    //TODO if skill_id is keydown/charge skill
    //key_down_dur: u32,
    attack_flags: AttackFlags,
    action_dir: ActionDir,
    unknown_crc_1: u32,
    attack_action_type: u8,
    atk_speed: u8,
    atk_time: u32,
    //Special bmage handling
    affected_area_id: u32,
}

#[derive(Debug)]
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

    pub fn encode<T: BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> Result<(), NetError> {
        self.mob_id.encode_packet(pw)?;
        self.hit_action.encode_packet(pw)?;
        self.fore_action.encode_packet(pw)?;
        self.frame_id.encode_packet(pw)?;
        self.calc_damage_stat_ix.encode_packet(pw)?;
        self.pos_a.encode_packet(pw)?;
        self.pos_b.encode_packet(pw)?;
        self.delay.encode_packet(pw)?;
        self.hits.encode_packet(pw)?;
        self.mob_crc.encode_packet(pw)?;
        Ok(())
    }

    pub fn packet_len(&self) -> usize {
        //TODO use derive crate to handle this shit
        22 + self.hits.len() * 4
    }
}

pub trait AttackInfo {
    fn targets(&self) -> usize;
    fn hits(&self) -> usize;
}

impl AttackInfo for MeleeAttackInfo {
    fn targets(&self) -> usize {
        self.hit_target_count.targets as usize
    }

    fn hits(&self) -> usize {
        self.hit_target_count.hits as usize
    }
}

impl<Info, Extra> EncodePacket for AttackReq<Info, Extra>
where
    Info: AttackInfo + EncodePacket,
    Extra: EncodePacket,
{
    fn encode_packet<T: bytes::BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
        self.info.encode_packet(pw)?;
        for target in self.targets.iter() {
            target.encode(pw)?;
        }
        self.extra.encode_packet(pw)?;
        Ok(())
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

impl<Info, Extra> PacketLen for AttackReq<Info, Extra>
where
    Info: PacketLen + AttackInfo,
    Extra: PacketLen,
{
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.extra.packet_len()
            + self.info.packet_len()
            + self.targets.iter().map(|t| t.packet_len()).sum::<usize>()
    }
}

#[derive(Debug)]
pub struct AttackReq<Info: AttackInfo, Extra> {
    pub info: Info,
    pub targets: Vec<AttackTargetInfo>,
    pub extra: Extra,
}

pub type UserMeleeAttackReq = AttackReq<MeleeAttackInfo, u32>;
packet_opcode!(UserMeleeAttackReq, RecvOpcodes::UserMeleeAttack);

#[cfg(test)]
mod tests {
    use moople_packet::DecodePacket;

    use crate::game::user::UserMeleeAttackReq;

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
}
