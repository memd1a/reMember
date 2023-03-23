use moople_packet::mark_maple_enum;
use num_enum::{IntoPrimitive, TryFromPrimitive};

//TODO model sub job for dual blade
// Which is actually a beginner but sub job is set to 1

use super::{FaceId, HairId, ItemId, MapId};

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum JobId {
    Beginner = 0,

    Warrior = 100,
    Fighter = 110,
    Crusader = 111,
    Hero = 112,

    Page = 120,
    WhiteKnight = 121,
    Paladin = 122,
    Spearnman = 130,
    DragonKnight = 131,
    DarkKnight = 132,

    Magician = 200,
    WizardFirePoison = 210,
    MageFirePoison = 211,
    ArchMageFirePoinson = 212,

    WizardIceLightning = 220,
    MageIceLightning = 221,
    ArchMageIceLightning = 222,

    Cleric = 230,
    Priest = 231,
    Bishop = 232,

    Bowman = 300,
    Hunter = 310,
    Ranger = 311,
    BowMaster = 312,

    Crossbowman = 320,
    Sniper = 321,
    Marksman = 322,

    Thief = 400,
    Assassin = 410,
    Hermit = 411,
    NightLord = 412,

    Bandit = 420,
    ChiefBandit = 421,
    Shadower = 422,

    BladeRecruit = 430,
    BladeAcolyte = 431,
    BladeSpecialist = 432,
    BladeLord = 433,
    BladeMaster = 434,

    Pirate = 500,
    Brawler = 510,
    Marauder = 511,
    Buccaneer = 512,

    Gunslinger = 520,
    Outlaw = 521,
    Corsair = 522,

    //What's that?
    MapleLeafBrigadier = 800,
    GM = 900,
    SuperGM = 910,

    Noblesse = 1000,

    DawnWarrior1 = 1100,
    DawnWarrior2 = 1110,
    DawnWarrior3 = 1111,
    DawnWarrior4 = 1112,

    BlazeWizard1 = 1200,
    BlazeWizard2 = 1210,
    BlazeWizard3 = 1211,
    BlazeWizard4 = 1212,

    WindArcher1 = 1300,
    WindArcher2 = 1310,
    WindArcher3 = 1311,
    WindArcher4 = 1312,

    NightWalker1 = 1400,
    NightWalker2 = 1410,
    NightWalker3 = 1411,
    NightWalker4 = 1412,

    ThunderBreaker1 = 1500,
    ThunderBreaker2 = 1510,
    ThunderBreaker3 = 1511,
    ThunderBreaker4 = 1512,

    Legend = 2000,
    Aran1 = 2100,
    Aran2 = 2110,
    Aran3 = 2111,
    Aran4 = 2112,

    EvanBeginner = 2001,
    Evan1 = 2200,
    Evan2 = 2210,
    Evan3 = 2211,
    Evan4 = 2212,
    Evan5 = 2213,
    Evan6 = 2214,
    Evan7 = 2215,
    Evan8 = 2216,
    Evan9 = 2217,
    Evan10 = 2218,

    Citizen = 3000,

    BattleMage1 = 3100,
    BattleMage2 = 3110,
    BattleMage3 = 3111,
    BattleMage4 = 3112,

    WildHunter1 = 3300,
    WildHunter2 = 3310,
    WildHunter3 = 3311,
    WildHunter4 = 3312,

    Mechanic1 = 3500,
    Mechanic2 = 3510,
    Mechanic3 = 3511,
    Mechanic4 = 3512,
}
mark_maple_enum!(JobId);

impl JobId {
    pub fn job_group(&self) -> JobGroup {
        let id = *self as u16;
        match id / 1000 {
            0 => JobGroup::Adventurer,
            1 => JobGroup::KnightsOfCygnus,
            2 if *self == JobId::EvanBeginner || id / 100 == 22 => JobGroup::Resistance,
            2 => JobGroup::Legend,
            3 => JobGroup::Resistance,
            _ => unreachable!("Invalid job id {id} has not group "),
        }
    }

    pub fn job_class(&self) -> JobClass {
        let id = *self as u16;
        match id / 100 {
            //Adventurer
            0 => JobClass::Beginner,
            1 => JobClass::Warrior,
            2 => JobClass::Magician,
            3 => JobClass::Bowman,
            4 => JobClass::Thief,
            5 => JobClass::Pirate,
            8 => JobClass::Unknown,
            9 => JobClass::GM,

            // Cygnus
            10 => JobClass::Noblesse,
            11 => JobClass::DawnWarrior,
            12 => JobClass::BlazeWizard,
            13 => JobClass::WindArcher,
            14 => JobClass::NightWalker,
            15 => JobClass::ThunderBreaker,

            // Legends
            20 => JobClass::LegendBeginner,
            21 => JobClass::Aran,
            22 => JobClass::Evan,

            32 => JobClass::BattleMage,
            33 => JobClass::WildHunter,
            35 => JobClass::Mechanic,

            _ => unreachable!("Invalid job id {id} has not class "),
        }
    }

    pub fn job_level(&self) -> usize {
        if self.is_noob() {
            return 0;
        }

        let id = *self as u16;
        let lvl = id % 10;

        match lvl {
            0 if id % 100 == 0 => 1,
            lvl => (lvl + 2) as usize,
        }
    }

    pub fn is_noob(&self) -> bool {
        matches!(
            *self,
            Self::Beginner | Self::Noblesse | Self::Legend | Self::EvanBeginner | Self::Citizen
        )
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Self::GM | Self::SuperGM)
    }

    pub fn has_extended_sp(&self) -> bool {
        self.job_group() == JobGroup::Resistance || self.job_class() == JobClass::Evan
    }
}

pub type SubJob = u16;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum JobGroup {
    Resistance = 0,
    Adventurer = 1,
    KnightsOfCygnus = 2,
    Legend = 3,
    Evan = 4,
}

mark_maple_enum!(JobGroup);

impl JobGroup {
    pub fn get_noob_job_id(&self) -> JobId {
        match *self {
            Self::Adventurer => JobId::Beginner,
            Self::Legend => JobId::Legend,
            Self::KnightsOfCygnus => JobId::Noblesse,
            Self::Evan => JobId::EvanBeginner,
            Self::Resistance => JobId::Citizen,
        }
    }

    pub fn get_starter_weapons(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::SWORD,
            ItemId::HAND_AXE,
            ItemId::WOODEN_CLUB,
            ItemId::BASIC_POLEARM,
        ]
        .into_iter()
    }

    pub fn get_starter_tops(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::WHITE_UNDERSHIRT,
            ItemId::UNDERSHIRT,
            ItemId::GREY_TSHIRT,
            ItemId::WHITE_TUBETOP,
            ItemId::YELLOW_TSHIRT,
            // Aran
            ItemId::SIMPLE_WARRIOR_TOP,
        ]
        .into_iter()
    }

    pub fn get_starter_bottoms(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::BLUE_JEAN_SHORTS,
            ItemId::BROWN_COTTON_SHORTS,
            ItemId::RED_MINISKIRT,
            ItemId::INDIGO_MINISKIRT,
            // Aran
            ItemId::SIMPLE_WARRIOR_PANTS,
        ]
        .into_iter()
    }

    pub fn get_starter_shoes(&self) -> impl Iterator<Item = ItemId> {
        [
            ItemId::RED_RUBBER_BOOTS,
            ItemId::LEATHER_SANDALS,
            ItemId::YELLOW_RUBBER_BOOTS,
            ItemId::BLUE_RUBBER_BOOTS,
            // Aran
            ItemId::AVERAGE_MUSASHI_SHOES,
        ]
        .into_iter()
    }

    pub fn get_starter_face(&self) -> impl Iterator<Item = FaceId> {
        [
            FaceId::MOTIVATED_LOOK_M,
            FaceId::MOTIVATED_LOOK_F,
            FaceId::PERPLEXED_STARE,
            FaceId::PERPLEXED_STARE_HAZEL,
            FaceId::LEISURE_LOOK_M,
            FaceId::LEISURE_LOOK_F,
            FaceId::FEARFUL_STARE_M,
            FaceId::FEARFUL_STARE_F,
            FaceId::LEISURE_LOOK_HAZEL,
            FaceId::MOTIVATED_LOOK_AMETHYST,
            FaceId::MOTIVATED_LOOK_BLUE,
        ]
        .into_iter()
    }

    pub fn get_starter_hair(&self) -> impl Iterator<Item = HairId> {
        [
            HairId::BLACK_TOBEN,
            HairId::ZETA,
            HairId::BLACK_REBEL,
            HairId::BLACK_BUZZ,
            HairId::BLACK_SAMMY,
            HairId::BLACK_EDGY,
            HairId::BLACK_CONNIE,
        ]
        .into_iter()
    }

    pub fn get_guide_item(&self) -> ItemId {
        match *self {
            Self::KnightsOfCygnus => ItemId::NOBLESSE_GUIDE,
            Self::Legend => ItemId::LEGENDS_GUIDE,
            Self::Evan => ItemId::LEGENDS_GUIDE,
            Self::Adventurer => ItemId::BEGINNERS_GUIDE,
            //TODO
            Self::Resistance => ItemId::BEGINNERS_GUIDE,
        }
    }

    pub fn get_start_map(&self) -> MapId {
        match *self {
            Self::Adventurer => MapId::MUSHROOM_TOWN,
            Self::Legend => MapId::ARAN_TUTORIAL_START,
            Self::Evan => MapId::STARTING_MAP_EVAN,
            Self::KnightsOfCygnus => MapId::STARTING_MAP_NOBLESSE,
            Self::Resistance => MapId::STARTING_MAP_RESISTANCE,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum JobClass {
    // Noobs
    Beginner,
    Noblesse,
    LegendBeginner,

    // Adventurer
    Warrior,
    Magician,
    Bowman,
    Thief,
    Pirate,

    //Cygnus
    DawnWarrior,
    BlazeWizard,
    WindArcher,
    NightWalker,
    ThunderBreaker,

    // Legends
    Aran,
    Evan,

    // Resistance
    BattleMage,
    WildHunter,
    Mechanic,

    //GM
    GM,
    //TODO: MAPLE LEAF BRIGADIER
    Unknown,
}