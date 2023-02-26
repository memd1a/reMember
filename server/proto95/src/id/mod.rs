//TODO find a way to auto generate this from wz files or verify this with files during build time

pub mod item_id;
pub mod job_id;
pub mod map_id;

use moople_derive::MooplePacket;
use moople_packet::mark_maple_enum;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub use self::item_id::ItemId;
pub use self::job_id::JobClass;
pub use self::map_id::MapId;

#[derive(Debug, MooplePacket, PartialEq, Eq, Clone, Copy)]
pub struct FaceId(pub u32);

impl FaceId {
    pub const MOTIVATED_LOOK_M: FaceId = FaceId(20000); // Face
    pub const PERPLEXED_STARE: FaceId = FaceId(20001);
    pub const LEISURE_LOOK_M: FaceId = FaceId(20002);
    pub const MOTIVATED_LOOK_F: FaceId = FaceId(21000);
    pub const FEARFUL_STARE_M: FaceId = FaceId(21001);
    pub const LEISURE_LOOK_F: FaceId = FaceId(21002);
    pub const FEARFUL_STARE_F: FaceId = FaceId(21201);
    pub const PERPLEXED_STARE_HAZEL: FaceId = FaceId(20401);
    pub const LEISURE_LOOK_HAZEL: FaceId = FaceId(20402);
    pub const MOTIVATED_LOOK_AMETHYST: FaceId = FaceId(21700);
    pub const MOTIVATED_LOOK_BLUE: FaceId = FaceId(20100);
}

#[derive(Debug, MooplePacket, PartialEq, Eq, Clone, Copy)]
pub struct HairId(pub u32);

impl HairId {
    pub const BLACK_TOBEN: HairId = HairId(30000); // Hair
    pub const ZETA: HairId = HairId(30010);
    pub const BLACK_REBEL: HairId = HairId(30020);
    pub const BLACK_BUZZ: HairId = HairId(30030);
    pub const BLACK_SAMMY: HairId = HairId(31000);
    pub const BLACK_EDGY: HairId = HairId(31040);
    pub const BLACK_CONNIE: HairId = HairId(31050);
}

#[derive(Debug, MooplePacket, PartialEq, Eq, Clone, Copy)]
pub struct SkillId(pub u32);

impl SkillId {
    pub fn is_monster_magnet(&self) -> bool {
        self.0 % 10000000 == 1004
    }

    pub fn is_charge_skill(&self) -> bool {
        //TODO
        [
            33101005, 33121009, 35001001, 35101009, 22121000, 22151001, 14111006, 15101003,
            3221001, 5201002, 5221004, 2321001, 3121004, 2121001, 4341003,
        ]
        .contains(&self.0)
    }

    pub fn is_grenade_skill(&self) -> bool {
        [14111006].contains(&self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Skin {
    Normal = 0,
    Dark = 1,
    Black = 2,
    Pale = 3,
    Blue = 4,
    Green = 5,
    White = 9,
    Pink = 10,
}

mark_maple_enum!(Skin);
