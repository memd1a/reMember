use moople_derive::{MoopleEncodePacket, MooplePacket};
use moople_packet::{
    maple_packet_enum,
    proto::{time::MapleDurationMs16, DecodePacket, MapleList8},
    NetResult,
};

use super::{Rect, Vec2, FootholdId};

#[derive(Debug, MoopleEncodePacket)]
pub struct KeyPadState(u8, Vec<u8>);

impl<'de> DecodePacket<'de> for KeyPadState {
    fn decode_packet(pr: &mut moople_packet::MaplePacketReader<'de>) -> NetResult<Self> {
        let n = pr.read_u8()? as usize / 2;

        let state = (0..n)
            .map(|_| pr.read_u8())
            .collect::<NetResult<Vec<_>>>()?;

        Ok(Self(n as u8, state))
    }
}

#[derive(MooplePacket, Debug)]
pub struct MovePassiveInfo {
    pub key_pad_state: KeyPadState,
    pub bounds: Rect,
}

/*
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
);*/
pub type MovementAction = u8;

#[derive(Debug, MooplePacket)]
pub struct MovementFooter {
    pub action: MovementAction,
    pub dur: MapleDurationMs16,
}


#[derive(Debug, MooplePacket)]
pub struct AbsoluteMovement {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub fh: FootholdId,
    pub offset: Vec2,
    pub footer: MovementFooter,
}

impl AbsoluteMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, Some(self.fh)))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}
#[derive(Debug, MooplePacket)]
pub struct AbsoluteFallMovement {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub fh: FootholdId,
    pub fh_fall_start: FootholdId,
    pub offset: Vec2,
    pub footer: MovementFooter,
}

impl AbsoluteFallMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, Some(self.fh)))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, MooplePacket)]
pub struct RelativeMovement {
    pub velocity: Vec2,
    pub footer: MovementFooter,
}

impl RelativeMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        None
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, MooplePacket)]
pub struct InstantMovement {
    pub pos: Vec2,
    pub fh: FootholdId,
    pub footer: MovementFooter,
}

impl InstantMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, Some(self.fh)))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, MooplePacket)]
pub struct FallDownMovement {
    pub velocity: Vec2,
    pub fh_fall_start: FootholdId,
    pub footer: MovementFooter,
}

impl FallDownMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        None
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, MooplePacket)]
pub struct FlyingMovement {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub footer: MovementFooter,
}

impl FlyingMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        Some((self.pos, None))
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
}

#[derive(Debug, MooplePacket)]
pub struct UnknownMovement {
    pub footer: MovementFooter,
}

impl UnknownMovement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        None
    }

    pub fn get_footer(&self) -> &MovementFooter {
        &self.footer
    }
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

impl Movement {
    pub fn get_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        match self {
            Movement::Normal(mv) => mv.get_pos_fh(),
            Movement::MobAttackRush(mv) => mv.get_pos_fh(),
            Movement::MobAttackRushStop(mv) => mv.get_pos_fh(),
            Movement::Jump(mv) => mv.get_pos_fh(),
            Movement::Impact(mv) => mv.get_pos_fh(),
            Movement::Immediate(mv) => mv.get_pos_fh(),
            Movement::Teleport(mv) => mv.get_pos_fh(),
            Movement::HangOnBack(mv) => mv.get_pos_fh(),
            Movement::Assaulter(mv) => mv.get_pos_fh(),
            Movement::Assassinate(mv) => mv.get_pos_fh(),
            Movement::Rush(mv) => mv.get_pos_fh(),
            Movement::StatChange(_mv) => None,
            Movement::SitDown(mv) => mv.get_pos_fh(),
            Movement::StartFallDown(mv) => mv.get_pos_fh(),
            Movement::FallDown(mv) => mv.get_pos_fh(),
            Movement::StartWings(mv) => mv.get_pos_fh(),
            Movement::Wings(mv) => mv.get_pos_fh(),
            Movement::MobToss(mv) => mv.get_pos_fh(),
            Movement::FlyingBlock(mv) => mv.get_pos_fh(),
            Movement::DashSlide(mv) => mv.get_pos_fh(),
            Movement::FlashJump(mv) => mv.get_pos_fh(),
            Movement::RocketBooster(mv) => mv.get_pos_fh(),
            Movement::BackstepShot(mv) => mv.get_pos_fh(),
            Movement::MobPowerKnockback(mv) => mv.get_pos_fh(),
            Movement::VerticalJump(mv) => mv.get_pos_fh(),
            Movement::CustomImpact(mv) => mv.get_pos_fh(),
            Movement::CombatStep(mv) => mv.get_pos_fh(),
            Movement::Hit(mv) => mv.get_pos_fh(),
            Movement::TimeBombAttack(mv) => mv.get_pos_fh(),
            Movement::SnowballTouch(mv) => mv.get_pos_fh(),
            Movement::BuffZoneEffect(mv) => mv.get_pos_fh(),
            Movement::MobLadder(mv) => mv.get_pos_fh(),
            Movement::MobRightAngle(mv) => mv.get_pos_fh(),
            Movement::MobStopNodeStart(mv) => mv.get_pos_fh(),
            Movement::MobBeforeNode(mv) => mv.get_pos_fh(),
        }
    }

    pub fn get_footer(&self) -> Option<&MovementFooter> {
        match self {
            Movement::Normal(mv) => Some(mv.get_footer()),
            Movement::MobAttackRush(mv) => Some(mv.get_footer()),
            Movement::MobAttackRushStop(mv) => Some(mv.get_footer()),
            Movement::Jump(mv) => Some(mv.get_footer()),
            Movement::Impact(mv) => Some(mv.get_footer()),
            Movement::Immediate(mv) => Some(mv.get_footer()),
            Movement::Teleport(mv) => Some(mv.get_footer()),
            Movement::HangOnBack(mv) => Some(mv.get_footer()),
            Movement::Assaulter(mv) => Some(mv.get_footer()),
            Movement::Assassinate(mv) => Some(mv.get_footer()),
            Movement::Rush(mv) => Some(mv.get_footer()),
            Movement::StatChange(_) => None,
            Movement::SitDown(mv) => Some(mv.get_footer()),
            Movement::StartFallDown(mv) => Some(mv.get_footer()),
            Movement::FallDown(mv) => Some(mv.get_footer()),
            Movement::StartWings(mv) => Some(mv.get_footer()),
            Movement::Wings(mv) => Some(mv.get_footer()),
            Movement::MobToss(mv) => Some(mv.get_footer()),
            Movement::FlyingBlock(mv) => Some(mv.get_footer()),
            Movement::DashSlide(mv) => Some(mv.get_footer()),
            Movement::FlashJump(mv) => Some(mv.get_footer()),
            Movement::RocketBooster(mv) => Some(mv.get_footer()),
            Movement::BackstepShot(mv) => Some(mv.get_footer()),
            Movement::MobPowerKnockback(mv) =>  Some(mv.get_footer()),
            Movement::VerticalJump(mv) => Some(mv.get_footer()),
            Movement::CustomImpact(mv) => Some(mv.get_footer()),
            Movement::CombatStep(mv) => Some(mv.get_footer()),
            Movement::Hit(mv) => Some(mv.get_footer()),
            Movement::TimeBombAttack(mv) => Some(mv.get_footer()),
            Movement::SnowballTouch(mv) => Some(mv.get_footer()),
            Movement::BuffZoneEffect(mv) => Some(mv.get_footer()),
            Movement::MobLadder(mv) => Some(mv.get_footer()),
            Movement::MobRightAngle(mv) => Some(mv.get_footer()),
            Movement::MobStopNodeStart(mv) => Some(mv.get_footer()),
            Movement::MobBeforeNode(mv) => Some(mv.get_footer()),
        }
    }
}

#[derive(MooplePacket, Debug)]
pub struct MovePath {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub moves: MapleList8<Movement>,
}

impl MovePath {
    pub fn get_last_pos_fh(&self) -> Option<(Vec2, Option<FootholdId>)> {
        self.moves
            .iter()
            .rev()
            .find_map(|p| p.get_pos_fh())
    }
}

#[derive(MooplePacket, Debug)]
pub struct MovePassivePath {
    pub path: MovePath,
    pub passive_info: MovePassiveInfo,
}
