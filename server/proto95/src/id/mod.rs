//TODO find a way to auto generate this from wz files or verify this with files during build time

pub mod item_id;
pub mod job_id;
pub mod map_id;

use moople_packet::maple_enum_code;

pub use self::item_id::ItemId;
pub use self::job_id::JobClass;
pub use self::map_id::MapId;

#[macro_export]
macro_rules! moople_id {
    ($name:ident, $ty:ty) => {
        #[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
        pub struct $name(pub $ty);

        impl moople_packet::proto::MapleWrapped for $name {
            type Inner = $ty;

            fn maple_into_inner(&self) -> Self::Inner {
                self.0
            }

            fn maple_from(v: Self::Inner) -> Self {
                Self(v)
            }
        }
    };
}

moople_id!(FaceId, u32);

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

moople_id!(HairId, u32);

impl HairId {
    pub const BLACK_TOBEN: HairId = HairId(30000); // Hair
    pub const ZETA: HairId = HairId(30010);
    pub const BLACK_REBEL: HairId = HairId(30020);
    pub const BLACK_BUZZ: HairId = HairId(30030);
    pub const BLACK_SAMMY: HairId = HairId(31000);
    pub const BLACK_EDGY: HairId = HairId(31040);
    pub const BLACK_CONNIE: HairId = HairId(31050);
}

moople_id!(SkillId, u32);

impl SkillId {
    pub fn is_dispel(&self) -> bool {
        self.0 == 2311001
    }

    pub fn is_anti_repeat_buff_skill(&self) -> bool {
        //TODO
        false
    }

    pub fn is_spirit_javelin(&self) -> bool {
        self.0 == 4121006
    }

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

maple_enum_code!(
    Skin,
    u8,
    Normal = 0,
    Dark = 1,
    Black = 2,
    Pale = 3,
    Blue = 4,
    Green = 5,
    White = 9,
    Pink = 10
);
