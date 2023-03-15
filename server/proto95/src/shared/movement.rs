use bytes::BufMut;
use moople_derive::MooplePacket;
use moople_packet::{
    maple_enum_code, maple_packet_enum,
    proto::{DecodePacket, EncodePacket, MapleList8, PacketLen, time::MapleDurationMs16},
    NetResult,
};

use super::{Rect, Vec2};

#[derive(Debug)]
pub struct KeyPadState(Vec<u8>);

impl EncodePacket for KeyPadState {
    fn encode_packet<B: BufMut>(
        &self,
        pw: &mut moople_packet::MaplePacketWriter<B>,
    ) -> NetResult<()> {
        pw.write_u8((self.0.len() * 2) as u8);
        u8::encode_packet_n(&self.0, pw)?;

        Ok(())
    }
}

impl<'de> DecodePacket<'de> for KeyPadState {
    fn decode_packet(pr: &mut moople_packet::MaplePacketReader<'de>) -> NetResult<Self> {
        let n = pr.read_u8()? as usize / 2;

        let state = (0..n)
            .map(|_| pr.read_u8())
            .collect::<NetResult<Vec<_>>>()?;

        Ok(Self(state))
    }
}

impl PacketLen for KeyPadState {
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        1 + self.0.len()
    }
}

#[derive(MooplePacket, Debug)]
pub struct MovePassiveInfo {
    pub key_pad_state: KeyPadState,
    pub bounds: Rect,
}

maple_enum_code!(
    MovementState,
    u8,
    LeftWalk = 3,
    RightWalk = 2,
    LeftStanding = 5,
    RightStanding = 4,
    LeftFalling = 7,
    RightFalling = 6,
    LeftAttack = 9,
    RightAttack = 8,
    LeftProne = 11,
    RightProne = 10,
    LeftRope = 13,
    RightRope = 12,
    LeftLadder = 15,
    RightLadder = 14
);

/// Every movement contains this
#[derive(Debug, MooplePacket)]
pub struct MovementInfo {
    pub pos: Vec2,
    pub velocity: Vec2,
}

#[derive(Debug, MooplePacket)]
pub struct MovementFooter {
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

type FootholdId = u16;

#[derive(Debug, MooplePacket)]
pub struct AbsoluteMovement {
    pub p: Vec2,
    pub velocity: Vec2,
    pub foothold: FootholdId,
    pub offset: Vec2,
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

#[derive(Debug, MooplePacket)]
pub struct AbsoluteFallMovement {
    pub p: Vec2,
    pub velocity: Vec2,
    pub fh: FootholdId,
    pub fh_fall_start: FootholdId,
    pub offset: Vec2,
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

#[derive(Debug, MooplePacket)]
pub struct RelativeMovement {
    pub velocity: Vec2,
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

#[derive(Debug, MooplePacket)]
pub struct InstantMovement {
    pub p: Vec2,
    pub fh: FootholdId,
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

#[derive(Debug, MooplePacket)]
pub struct FallDownMovement {
    pub velocity: Vec2,
    pub fh_fall_start: FootholdId,
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

#[derive(Debug, MooplePacket)]
pub struct FlyingMovement {
    pub p: Vec2,
    pub velocity: Vec2,
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

#[derive(Debug, MooplePacket)]
pub struct UnknownMovement {
    pub state: MovementState,
    pub dur: MapleDurationMs16,
}

maple_packet_enum!(
   Movement,
   u8,
   Normal(AbsoluteMovement) => 0,
   Jump(RelativeMovement) => 1,
   Impact(RelativeMovement) => 2,
   Immediate(InstantMovement) => 0x3,
   Teleport(InstantMovement) => 0x4,
   HangOnBack(AbsoluteMovement) => 5,
   Assaulter(InstantMovement) => 0x6,
   Assassinate(InstantMovement) => 0x7,
   Rush(InstantMovement) => 0x8,
   StatChange(u8) => 0x9,
   SitDown(InstantMovement) => 0xA,
   StartFallDown(FallDownMovement) => 0xB,
   FallDown(AbsoluteFallMovement) => 0xC,
   StartWings(RelativeMovement) => 0xD,
   Wings(AbsoluteMovement) => 0xE,
   //0xF ?? -> aran adjust?
   MobToss(RelativeMovement) => 0x10,
   FlyingBlock(FlyingMovement) => 0x11,
   DashSlide(RelativeMovement) => 0x12,
   // 0x13 -> bmage adjust?
    FlashJump(UnknownMovement) => 0x14,
    RocketBooster(UnknownMovement) => 0x15,
    BackstepShot(UnknownMovement) => 0x16,
    MobPowerKnockback(UnknownMovement) => 0x17,
    VerticalJump(UnknownMovement) => 0x18,
    CustomImpact(UnknownMovement) => 0x19,
    CombatStep(UnknownMovement) => 0x1A,
    Hit(UnknownMovement) => 0x1B,
    TimeBombAttack(UnknownMovement) => 0x1C,
    SnowballTouch(UnknownMovement) => 0x1D,
    BuffZoneEffect(UnknownMovement) => 0x1E,
   MobLadder(RelativeMovement) => 0x1F,
   MobRightAngle(RelativeMovement) => 0x20,
   MobStopNodeStart(RelativeMovement) => 0x21,
   MobBeforeNode(RelativeMovement) => 0x22,
   MobAttackRush(AbsoluteMovement) => 0x23,
   MobAttackRushStop(AbsoluteMovement) => 0x24,
);

#[derive(MooplePacket, Debug)]
pub struct MovePath {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub moves: MapleList8<Movement>,
}

#[derive(MooplePacket, Debug)]
pub struct MovePassivePath {
    pub path: MovePath,
    pub passive_info: MovePassiveInfo,
}
