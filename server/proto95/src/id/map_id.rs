use std::ops::RangeInclusive;

use crate::moople_id;

moople_id!(MapId, u32);

impl MapId {
    pub fn is_maple_island(&self) -> bool {
        Self::MAPLE_ISLAND_RANGE.contains(self)
    }

    // Aran tutorial / burning intro / godly stat
    pub fn is_aran_tutorial_map(&self) -> bool {
        matches!(
            *self,
            Self::BURNING_FOREST_1 | Self::BURNING_FOREST_2 | Self::BURNING_FOREST_3
        )
    }

    pub fn is_cygnus_intro(&self) -> bool {
        Self::CYGNUS_INTRO_LOCATION_RANGE.contains(self)
    }

    pub fn is_physical_fitness(&self) -> bool {
        Self::PHYSICAL_FITNESS_RANGE.contains(self)
    }

    pub fn is_solo_dojo(&self) -> bool {
        Self::DOJO_RANGE.contains(self)
    }

    pub fn is_party_dojo(&self) -> bool {
        Self::DOJO_PARTY_RANGE.contains(self)
    }

    //TODO what's that?
    pub fn is_self_lootable_only(&self) -> bool {
        Self::HAPPYVILLE_TREE_RANGE.contains(self) || Self::GPQ_FOUNTAIN_RANGE.contains(self)
    }

    pub fn is_ola_ola(&self) -> bool {
        Self::OLA_OLA_RANGE.contains(self)
    }

    pub fn is_boss_rush(&self) -> bool {
        Self::BOSS_RUSH_RANGE.contains(self)
    }

    pub fn is_netts_pyramid(&self) -> bool {
        Self::NETTS_PYRAMID_RANGE.contains(self)
    }

    pub fn is_fishing_area(&self) -> bool {
        matches!(
            *self,
            Self::ON_THE_WAY_TO_THE_HARBOR | Self::PIER_ON_THE_BEACH | Self::PEACEFUL_SHIP
        )
    }
}

impl MapId {
    // Special
    pub const NONE: MapId = MapId(999999999);
    pub const GM_MAP: MapId = MapId(180000000);
    pub const JAIL: MapId = MapId(300000012); // "Cellar: Camp Conference Room"
    pub const DEVELOPERS_HQ: MapId = MapId(777777777);

    // Misc
    pub const ORBIS_TOWER_BOTTOM: MapId = MapId(200082300);
    pub const INTERNET_CAFE: MapId = MapId(193000000);
    pub const CRIMSONWOOD_VALLEY_1: MapId = MapId(610020000);
    pub const CRIMSONWOOD_VALLEY_2: MapId = MapId(610020001);
    pub const HENESYS_PQ: MapId = MapId(910010000);
    pub const ORIGIN_OF_CLOCKTOWER: MapId = MapId(220080001);
    pub const CAVE_OF_PIANUS: MapId = MapId(230040420);
    pub const GUILD_HQ: MapId = MapId(200000301);
    pub const FM_ENTRANCE: MapId = MapId(910000000);

    // Beginner
    pub const MUSHROOM_TOWN: MapId = MapId(10000);

    // Town
    pub const SOUTHPERRY: MapId = MapId(2000000);
    pub const AMHERST: MapId = MapId(1000000);
    pub const HENESYS: MapId = MapId(100000000);
    pub const ELLINIA: MapId = MapId(101000000);
    pub const PERION: MapId = MapId(102000000);
    pub const KERNING_CITY: MapId = MapId(103000000);
    pub const LITH_HARBOUR: MapId = MapId(104000000);
    pub const SLEEPYWOOD: MapId = MapId(105040300);
    pub const MUSHROOM_KINGDOM: MapId = MapId(106020000);
    pub const FLORINA_BEACH: MapId = MapId(110000000);
    pub const EREVE: MapId = MapId(130000000);
    pub const KERNING_SQUARE: MapId = MapId(103040000);
    pub const RIEN: MapId = MapId(140000000);
    pub const ORBIS: MapId = MapId(200000000);
    pub const EL_NATH: MapId = MapId(211000000);
    pub const LUDIBRIUM: MapId = MapId(220000000);
    pub const AQUARIUM: MapId = MapId(230000000);
    pub const LEAFRE: MapId = MapId(240000000);
    pub const NEO_CITY: MapId = MapId(240070000);
    pub const MU_LUNG: MapId = MapId(250000000);
    pub const HERB_TOWN: MapId = MapId(251000000);
    pub const OMEGA_SECTOR: MapId = MapId(221000000);
    pub const KOREAN_FOLK_TOWN: MapId = MapId(222000000);
    pub const ARIANT: MapId = MapId(260000000);
    pub const MAGATIA: MapId = MapId(261000000);
    pub const TEMPLE_OF_TIME: MapId = MapId(270000100);
    pub const ELLIN_FOREST: MapId = MapId(300000000);
    pub const SINGAPORE: MapId = MapId(540000000);
    pub const BOAT_QUAY_TOWN: MapId = MapId(541000000);
    pub const KAMPUNG_VILLAGE: MapId = MapId(551000000);
    pub const NEW_LEAF_CITY: MapId = MapId(600000000);
    pub const MUSHROOM_SHRINE: MapId = MapId(800000000);
    pub const SHOWA_TOWN: MapId = MapId(801000000);
    pub const NAUTILUS_HARBOR: MapId = MapId(120000000);
    pub const HAPPYVILLE: MapId = MapId(209000000);

    pub const SHOWA_SPA_M: MapId = MapId(809000101);
    pub const SHOWA_SPA_F: MapId = MapId(809000201);

    pub(crate) const MAPLE_ISLAND_MIN: MapId = MapId(0);
    pub(crate) const MAPLE_ISLAND_MAX: MapId = MapId(2000001);
    pub(crate) const MAPLE_ISLAND_RANGE: RangeInclusive<MapId> =
        (Self::MAPLE_ISLAND_MIN..=Self::MAPLE_ISLAND_MAX);

    // Travel
    // There are 10 of each of these travel maps in the files
    pub const FROM_LITH_TO_RIEN: MapId = MapId(200090060);
    pub const FROM_RIEN_TO_LITH: MapId = MapId(200090070);
    pub const DANGEROUS_FOREST: MapId = MapId(140020300); // Rien docks
    pub const FROM_ELLINIA_TO_EREVE: MapId = MapId(200090030);
    pub const SKY_FERRY: MapId = MapId(130000210); // Ereve platform
    pub const FROM_EREVE_TO_ELLINIA: MapId = MapId(200090031);
    pub const ELLINIA_SKY_FERRY: MapId = MapId(101000400);
    pub const FROM_EREVE_TO_ORBIS: MapId = MapId(200090021);
    pub const ORBIS_STATION: MapId = MapId(200000161);
    pub const FROM_ORBIS_TO_EREVE: MapId = MapId(200090020);

    // Aran
    pub const ARAN_TUTORIAL_START: MapId = MapId(914000000);
    pub const ARAN_TUTORIAL_MAX: MapId = MapId(914000500);
    pub const ARAN_INTRO: MapId = MapId(140090000);
    pub(crate) const BURNING_FOREST_1: MapId = MapId(914000200);
    pub(crate) const BURNING_FOREST_2: MapId = MapId(914000210);
    pub(crate) const BURNING_FOREST_3: MapId = MapId(914000220);

    // Aran intro
    pub const ARAN_TUTO_1: MapId = MapId(914090010);
    pub const ARAN_TUTO_2: MapId = MapId(914090011);
    pub const ARAN_TUTO_3: MapId = MapId(914090012);
    pub const ARAN_TUTO_4: MapId = MapId(914090013);
    pub const ARAN_POLEARM: MapId = MapId(914090100);
    pub const ARAN_MAHA: MapId = MapId(914090200); // Black screen when warped to

    // Starting map Evan
    pub const STARTING_MAP_EVAN: MapId = MapId(100030100);

    // Starting map
    pub const STARTING_MAP_NOBLESSE: MapId = MapId(130030000);

    // Edelstein Starting map
    pub const STARTING_MAP_RESISTANCE: MapId = MapId(310010000);

    // Cygnus intro
    // These are the actual maps
    pub(crate) const CYGNUS_INTRO_LOCATION_MIN: MapId = MapId(913040000);
    pub(crate) const CYGNUS_INTRO_LOCATION_MAX: MapId = MapId(913040006);
    pub(crate) const CYGNUS_INTRO_LOCATION_RANGE: RangeInclusive<MapId> =
        (Self::CYGNUS_INTRO_LOCATION_MIN..=Self::CYGNUS_INTRO_LOCATION_MAX);

    // Cygnus intro video
    pub const CYGNUS_INTRO_LEAD: MapId = MapId(913040100);
    pub const CYGNUS_INTRO_WARRIOR: MapId = MapId(913040101);
    pub const CYGNUS_INTRO_BOWMAN: MapId = MapId(913040102);
    pub const CYGNUS_INTRO_MAGE: MapId = MapId(913040103);
    pub const CYGNUS_INTRO_PIRATE: MapId = MapId(913040104);
    pub const CYGNUS_INTRO_THIEF: MapId = MapId(913040105);
    pub const CYGNUS_INTRO_CONCLUSION: MapId = MapId(913040106);

    // Event
    pub const EVENT_COCONUT_HARVEST: MapId = MapId(109080000);
    pub const EVENT_OX_QUIZ: MapId = MapId(109020001);
    pub const EVENT_PHYSICAL_FITNESS: MapId = MapId(109040000);
    pub const EVENT_OLA_OLA_0: MapId = MapId(109030001);
    pub const EVENT_OLA_OLA_1: MapId = MapId(109030101);
    pub const EVENT_OLA_OLA_2: MapId = MapId(109030201);
    pub const EVENT_OLA_OLA_3: MapId = MapId(109030301);
    pub const EVENT_OLA_OLA_4: MapId = MapId(109030401);
    pub const EVENT_SNOWBALL: MapId = MapId(109060000);
    pub const EVENT_FIND_THE_JEWEL: MapId = MapId(109010000);
    pub const FITNESS_EVENT_LAST: MapId = MapId(109040004);
    pub const OLA_EVENT_LAST_1: MapId = MapId(109030003);
    pub const OLA_EVENT_LAST_2: MapId = MapId(109030103);
    pub const WITCH_TOWER_ENTRANCE: MapId = MapId(980040000);
    pub const EVENT_WINNER: MapId = MapId(109050000);
    pub const EVENT_EXIT: MapId = MapId(109050001);
    pub const EVENT_SNOWBALL_ENTRANCE: MapId = MapId(109060001);

    pub(crate) const PHYSICAL_FITNESS_MIN: MapId = Self::EVENT_PHYSICAL_FITNESS;
    pub(crate) const PHYSICAL_FITNESS_MAX: MapId = Self::FITNESS_EVENT_LAST;
    pub(crate) const PHYSICAL_FITNESS_RANGE: RangeInclusive<MapId> =
        (Self::PHYSICAL_FITNESS_MIN..=Self::PHYSICAL_FITNESS_MAX);

    pub(crate) const OLA_OLA_MIN: MapId = Self::EVENT_OLA_OLA_0;
    pub(crate) const OLA_OLA_MAX: MapId = MapId(109030403); // OLA_OLA_4 level 3
    pub(crate) const OLA_OLA_RANGE: RangeInclusive<MapId> = (Self::OLA_OLA_MIN..=Self::OLA_OLA_MAX);

    // Self lootable maps
    pub(crate) const HAPPYVILLE_TREE_MIN: MapId = MapId(209000001);
    pub(crate) const HAPPYVILLE_TREE_MAX: MapId = MapId(209000015);
    pub(crate) const HAPPYVILLE_TREE_RANGE: RangeInclusive<MapId> =
        (Self::HAPPYVILLE_TREE_MIN..=Self::HAPPYVILLE_TREE_MAX);

    pub(crate) const GPQ_FOUNTAIN_MIN: MapId = MapId(990000500);
    pub(crate) const GPQ_FOUNTAIN_MAX: MapId = MapId(990000502);
    pub(crate) const GPQ_FOUNTAIN_RANGE: RangeInclusive<MapId> =
        (Self::GPQ_FOUNTAIN_MIN..=Self::GPQ_FOUNTAIN_MAX);

    // Dojo
    pub const DOJO_SOLO_BASE: MapId = MapId(925020000);
    pub const DOJO_PARTY_BASE: MapId = MapId(925030000);
    pub const DOJO_EXIT: MapId = MapId(925020002);

    pub(crate) const DOJO_MIN: MapId = Self::DOJO_SOLO_BASE;
    pub(crate) const DOJO_MAX: MapId = MapId(925033804);
    pub(crate) const DOJO_RANGE: RangeInclusive<MapId> = (Self::DOJO_MIN..=Self::DOJO_MAX);

    pub(crate) const DOJO_PARTY_MIN: MapId = MapId(925030100);
    pub const DOJO_PARTY_MAX: MapId = Self::DOJO_MAX;
    pub(crate) const DOJO_PARTY_RANGE: RangeInclusive<MapId> =
        (Self::DOJO_PARTY_MIN..=Self::DOJO_PARTY_MAX);

    // Mini dungeon
    pub const ANT_TUNNEL_2: MapId = MapId(105050100);
    pub const CAVE_OF_MUSHROOMS_BASE: MapId = MapId(105050101);
    pub const SLEEPY_DUNGEON_4: MapId = MapId(105040304);
    pub const GOLEMS_CASTLE_RUINS_BASE: MapId = MapId(105040320);
    pub const SAHEL_2: MapId = MapId(260020600);
    pub const HILL_OF_SANDSTORMS_BASE: MapId = MapId(260020630);
    pub const RAIN_FOREST_EAST_OF_HENESYS: MapId = MapId(100020000);
    pub const HENESYS_PIG_FARM_BASE: MapId = MapId(100020100);
    pub const COLD_CRADLE: MapId = MapId(105090311);
    pub const DRAKES_BLUE_CAVE_BASE: MapId = MapId(105090320);
    pub const EOS_TOWER_76TH_TO_90TH_FLOOR: MapId = MapId(221023400);
    pub const DRUMMER_BUNNYS_LAIR_BASE: MapId = MapId(221023401);
    pub const BATTLEFIELD_OF_FIRE_AND_WATER: MapId = MapId(240020500);
    pub const ROUND_TABLE_OF_KENTAURUS_BASE: MapId = MapId(240020512);
    pub const RESTORING_MEMORY_BASE: MapId = MapId(240040800);
    pub const DESTROYED_DRAGON_NEST: MapId = MapId(240040520);
    pub const NEWT_SECURED_ZONE_BASE: MapId = MapId(240040900);
    pub const RED_NOSE_PIRATE_DEN_2: MapId = MapId(251010402);
    pub const PILLAGE_OF_TREASURE_ISLAND_BASE: MapId = MapId(251010410);
    pub const LAB_AREA_C1: MapId = MapId(261020300);
    pub const CRITICAL_ERROR_BASE: MapId = MapId(261020301);
    pub const FANTASY_THEME_PARK_3: MapId = MapId(551030000);
    pub const LONGEST_RIDE_ON_BYEBYE_STATION: MapId = MapId(551030001);

    // Boss rush
    pub(crate) const BOSS_RUSH_MIN: MapId = MapId(970030100);
    pub(crate) const BOSS_RUSH_MAX: MapId = MapId(970042711);
    pub(crate) const BOSS_RUSH_RANGE: RangeInclusive<MapId> =
        (Self::BOSS_RUSH_MIN..=Self::BOSS_RUSH_MAX);

    // ARPQ
    pub const ARPQ_LOBBY: MapId = MapId(980010000);
    pub const ARPQ_ARENA_1: MapId = MapId(980010101);
    pub const ARPQ_ARENA_2: MapId = MapId(980010201);
    pub const ARPQ_ARENA_3: MapId = MapId(980010301);
    pub const ARPQ_KINGS_ROOM: MapId = MapId(980010010);

    // Nett's pyramid
    pub const NETTS_PYRAMID: MapId = MapId(926010001);
    pub const NETTS_PYRAMID_SOLO_BASE: MapId = MapId(926010100);
    pub const NETTS_PYRAMID_PARTY_BASE: MapId = MapId(926020100);
    pub(crate) const NETTS_PYRAMID_MIN: MapId = Self::NETTS_PYRAMID_SOLO_BASE;
    pub(crate) const NETTS_PYRAMID_MAX: MapId = MapId(926023500);
    pub(crate) const NETTS_PYRAMID_RANGE: RangeInclusive<MapId> =
        (Self::NETTS_PYRAMID_MIN..=Self::NETTS_PYRAMID_MAX);

    // Fishing
    pub(crate) const ON_THE_WAY_TO_THE_HARBOR: MapId = MapId(120010000);
    pub(crate) const PIER_ON_THE_BEACH: MapId = MapId(251000100);
    pub(crate) const PEACEFUL_SHIP: MapId = MapId(541010110);

    // Wedding
    pub const AMORIA: MapId = MapId(680000000);
    pub const CHAPEL_WEDDING_ALTAR: MapId = MapId(680000110);
    pub const CATHEDRAL_WEDDING_ALTAR: MapId = MapId(680000210);
    pub const WEDDING_PHOTO: MapId = MapId(680000300);
    pub const WEDDING_EXIT: MapId = MapId(680000500);

    // Statue
    pub const HALL_OF_WARRIORS: MapId = MapId(102000004); // Explorer
    pub const HALL_OF_MAGICIANS: MapId = MapId(101000004);
    pub const HALL_OF_BOWMEN: MapId = MapId(100000204);
    pub const HALL_OF_THIEVES: MapId = MapId(103000008);
    pub const NAUTILUS_TRAINING_ROOM: MapId = MapId(120000105);
    pub const KNIGHTS_CHAMBER: MapId = MapId(130000100); // Cygnus
    pub const KNIGHTS_CHAMBER_2: MapId = MapId(130000110);
    pub const KNIGHTS_CHAMBER_3: MapId = MapId(130000120);
    pub const KNIGHTS_CHAMBER_LARGE: MapId = MapId(130000101);
    pub const PALACE_OF_THE_MASTER: MapId = MapId(140010110); // Aran

    // gm-goto
    pub const EXCAVATION_SITE: MapId = MapId(990000000);
    pub const SOMEONE_ELSES_HOUSE: MapId = MapId(100000005);
    pub const GRIFFEY_FOREST: MapId = MapId(240020101);
    pub const MANONS_FOREST: MapId = MapId(240020401);
    pub const HOLLOWED_GROUND: MapId = MapId(682000001);
    pub const CURSED_SANCTUARY: MapId = MapId(105090900);
    pub const DOOR_TO_ZAKUM: MapId = MapId(211042300);
    pub const DRAGON_NEST_LEFT_BEHIND: MapId = MapId(240040511);
    pub const HENESYS_PARK: MapId = MapId(100000200);
    pub const ENTRANCE_TO_HORNTAILS_CAVE: MapId = MapId(240050400);
    pub const FORGOTTEN_TWILIGHT: MapId = MapId(270050000);
    pub const CRIMSONWOOD_KEEP: MapId = MapId(610020006);
    pub const MU_LUNG_DOJO_HALL: MapId = MapId(925020001);
    pub const EXCLUSIVE_TRAINING_CENTER: MapId = MapId(970030000);
}
