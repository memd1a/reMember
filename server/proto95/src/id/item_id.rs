use std::ops::RangeInclusive;

use moople_derive::MooplePacket;

#[derive(Debug, MooplePacket, PartialEq, PartialOrd, Eq, Clone, Copy, Default)]
pub struct ItemId(pub u32);

impl ItemId {
    pub fn is_arrow_for_bow(&self) -> bool {
        (2060000..=2061000).contains(&self.0)
    }

    pub fn is_arrow_for_crossbow(&self) -> bool {
        (2061000..=2062000).contains(&self.0)
    }

    pub fn is_rechargable(&self) -> bool {
        self.0 / 10000 == 233 || self.0 / 10000 == 207
    }

    pub fn is_exp_increase(&self) -> bool {
        (2022450..=2022452).contains(&self.0)
    }

    pub fn is_rate_coupon(&self) -> bool {
        let ty = self.0 / 1000;
        ty == 5211 || ty == 5360
    }

    pub fn is_monster_card(&self) -> bool {
        let ty = self.0 / 10000;
        ty == 238
    }

    pub fn is_pyramid_buff(&self) -> bool {
        (2022585..=2022588).contains(&self.0) || (2022616..=2022617).contains(&self.0)
    }

    pub fn is_dojo_buff(&self) -> bool {
        (2022359..=2022421).contains(&self.0)
    }

    pub fn is_chair(&self) -> bool {
        Self::CHAIR_RANGE.contains(self)
    }

    pub fn is_wedding_ring(&self) -> bool {
        matches!(
            *self,
            Self::WEDDING_RING_GOLDEN
                | Self::WEDDING_RING_MOONSTONE
                | Self::WEDDING_RING_SILVER
                | Self::WEDDING_RING_STAR
        )
    }

    pub fn is_wedding_token(&self) -> bool {
        (Self::EMPTY_ENGAGEMENT_BOX_MOONSTONE..=Self::ENGAGEMENT_BOX_SILVER).contains(self)
    }

    pub fn is_party_all_cure(&self) -> bool {
        matches!(
            *self,
            Self::DOJO_PARTY_ALL_CURE | Self::CARNIVAL_PARTY_ALL_CURE
        )
    }

    pub fn is_pet(&self) -> bool {
        self.0 / 1000 == 5000
    }

    pub fn is_nx_card(&self) -> bool {
        matches!(*self, Self::NX_CARD_100 | Self::NX_CARD_250)
    }

    pub fn is_facial_expression(&self) -> bool {
        Self::FACE_EXPRESSION_RANGE.contains(self)
    }

    pub fn is_cygnus_mount(&self) -> bool {
        (Self::MIMIANA..=Self::SHINJOU).contains(self) || *self == Self::CYGNUS_SADDLE
    }

    pub fn is_explorer_mount(&self) -> bool {
        (Self::HOG..=Self::RED_DRACO).contains(self) || *self == Self::EXPLORER_SADDLE
    }
}

impl ItemId {
    // Misc
    pub const PENDANT_OF_THE_SPIRIT: ItemId = ItemId(1122017);
    pub const HEART_SHAPED_CHOCOLATE: ItemId = ItemId(5110000);
    pub const HAPPY_BIRTHDAY: ItemId = ItemId(2022153);
    pub const FISHING_CHAIR: ItemId = ItemId(3011000);
    pub const MINI_GAME_BASE: ItemId = ItemId(4080000);
    pub const MATCH_CARDS: ItemId = ItemId(4080100);
    pub const MAGICAL_MITTEN: ItemId = ItemId(1472063);
    pub const RPS_CERTIFICATE_BASE: ItemId = ItemId(4031332);
    pub const GOLDEN_MAPLE_LEAF: ItemId = ItemId(4000313);
    pub const PERFECT_PITCH: ItemId = ItemId(4310000);
    pub const MAGIC_ROCK: ItemId = ItemId(4006000);
    pub const GOLDEN_CHICKEN_EFFECT: ItemId = ItemId(4290000);
    pub const BUMMER_EFFECT: ItemId = ItemId(4290001);
    pub const ARPQ_SHIELD: ItemId = ItemId(2022269);
    pub const ROARING_TIGER_MESSENGER: ItemId = ItemId(5390006);
    // Potion
    pub const WHITE_POTION: ItemId = ItemId(2000002);
    pub const BLUE_POTION: ItemId = ItemId(2000003);
    pub const ORANGE_POTION: ItemId = ItemId(2000001);
    pub const MANA_ELIXIR: ItemId = ItemId(2000006);

    // HP/MP recovery
    pub const SORCERERS_POTION: ItemId = ItemId(2022337);
    pub const RUSSELLONS_PILLS: ItemId = ItemId(2022198);

    // Environment
    pub const RED_BEAN_PORRIDGE: ItemId = ItemId(2022001);
    pub const SOFT_WHITE_BUN: ItemId = ItemId(2022186);
    pub const AIR_BUBBLE: ItemId = ItemId(2022040);

    // Chair
    pub const RELAXER: ItemId = ItemId(3010000);
    pub const CHAIR_MIN: ItemId = Self::RELAXER;
    pub const CHAIR_MAX: ItemId = Self::FISHING_CHAIR;
    pub const CHAIR_RANGE: RangeInclusive<ItemId> = (Self::CHAIR_MIN..=Self::CHAIR_MAX);

    // Throwing star
    pub const SUBI_THROWING_STARS: ItemId = ItemId(2070000);
    pub const HWABI_THROWING_STARS: ItemId = ItemId(2070007);
    pub const BALANCED_FURY: ItemId = ItemId(2070018);
    pub const DEVIL_RAIN_THROWING_STAR: ItemId = ItemId(2070014);
    pub const CRYSTAL_ILBI_THROWING_STARS: ItemId = ItemId(2070016);
    pub const THROWING_STAR_MIN: ItemId = Self::SUBI_THROWING_STARS;
    //TODO MAX is  wrong(balanced fury not throwing stars???)
    pub const THROWING_STAR_MAX: ItemId = ItemId(2070016);
    pub const THROWING_STAR_RANGE: RangeInclusive<ItemId> =
        (Self::THROWING_STAR_MIN..=Self::THROWING_STAR_MAX);

    // Bullet
    pub const BULLET: ItemId = ItemId(2330000);
    pub const BULLET_MIN: ItemId = Self::BULLET;
    pub const BULLET_MAX: ItemId = ItemId(2330005);
    pub const BULLET_RANGE: RangeInclusive<ItemId> = (Self::BULLET_MIN..=Self::BULLET_MAX);
    pub const BLAZE_CAPSULE: ItemId = ItemId(2331000);
    pub const GLAZE_CAPSULE: ItemId = ItemId(2332000);

    // Starter
    pub const BEGINNERS_GUIDE: ItemId = ItemId(4161001);
    pub const LEGENDS_GUIDE: ItemId = ItemId(4161048);
    pub const NOBLESSE_GUIDE: ItemId = ItemId(4161047);
    pub const SWORD: ItemId = ItemId(1302000); // Weapon
    pub const HAND_AXE: ItemId = ItemId(1312004);
    pub const WOODEN_CLUB: ItemId = ItemId(1322005);
    pub const BASIC_POLEARM: ItemId = ItemId(1442079);
    pub const WHITE_UNDERSHIRT: ItemId = ItemId(1040002); // Top
    pub const UNDERSHIRT: ItemId = ItemId(1040006);
    pub const GREY_TSHIRT: ItemId = ItemId(1040010);
    pub const WHITE_TUBETOP: ItemId = ItemId(1041002);
    pub const YELLOW_TSHIRT: ItemId = ItemId(1041006);
    pub const GREEN_TSHIRT: ItemId = ItemId(1041010);
    pub const RED_STRIPED_TOP: ItemId = ItemId(1041011);
    pub const SIMPLE_WARRIOR_TOP: ItemId = ItemId(1042167);
    pub const BLUE_JEAN_SHORTS: ItemId = ItemId(1060002); // Bottom
    pub const BROWN_COTTON_SHORTS: ItemId = ItemId(1060006);
    pub const RED_MINISKIRT: ItemId = ItemId(1061002);
    pub const INDIGO_MINISKIRT: ItemId = ItemId(1061008);
    pub const SIMPLE_WARRIOR_PANTS: ItemId = ItemId(1062115);
    pub const RED_RUBBER_BOOTS: ItemId = ItemId(1072001);
    pub const LEATHER_SANDALS: ItemId = ItemId(1072005);
    pub const YELLOW_RUBBER_BOOTS: ItemId = ItemId(1072037);
    pub const BLUE_RUBBER_BOOTS: ItemId = ItemId(1072038);
    pub const AVERAGE_MUSASHI_SHOES: ItemId = ItemId(1072383);

    // Warrior
    pub const RED_HWARANG_SHIRT: ItemId = ItemId(1040021);
    pub const BLACK_MARTIAL_ARTS_PANTS: ItemId = ItemId(1060016);
    pub const MITHRIL_BATTLE_GRIEVES: ItemId = ItemId(1072039);
    pub const GLADIUS: ItemId = ItemId(1302008);
    pub const MITHRIL_POLE_ARM: ItemId = ItemId(1442001);
    pub const MITHRIL_MAUL: ItemId = ItemId(1422001);
    pub const FIREMANS_AXE: ItemId = ItemId(1312005);
    pub const DARK_ENGRIT: ItemId = ItemId(1051010);

    // Bowman
    pub const GREEN_HUNTERS_ARMOR: ItemId = ItemId(1040067);
    pub const GREEN_HUNTRESS_ARMOR: ItemId = ItemId(1041054);
    pub const GREEN_HUNTERS_PANTS: ItemId = ItemId(1060056);
    pub const GREEN_HUNTRESS_PANTS: ItemId = ItemId(1061050);
    pub const GREEN_HUNTER_BOOTS: ItemId = ItemId(1072081);
    pub const RYDEN: ItemId = ItemId(1452005);
    pub const MOUNTAIN_CROSSBOW: ItemId = ItemId(1462000);

    // Magician
    pub const BLUE_WIZARD_ROBE: ItemId = ItemId(1050003);
    pub const PURPLE_FAIRY_TOP: ItemId = ItemId(1041041);
    pub const PURPLE_FAIRY_SKIRT: ItemId = ItemId(1061034);
    pub const RED_MAGICSHOES: ItemId = ItemId(1072075);
    pub const MITHRIL_WAND: ItemId = ItemId(1372003);
    pub const CIRCLE_WINDED_STAFF: ItemId = ItemId(1382017);

    // Thief
    pub const DARK_BROWN_STEALER: ItemId = ItemId(1040057);
    pub const RED_STEAL: ItemId = ItemId(1041047);
    pub const DARK_BROWN_STEALER_PANTS: ItemId = ItemId(1060043);
    pub const RED_STEAL_PANTS: ItemId = ItemId(1061043);
    pub const BRONZE_CHAIN_BOOTS: ItemId = ItemId(1072032);
    pub const STEEL_GUARDS: ItemId = ItemId(1472008);
    pub const REEF_CLAW: ItemId = ItemId(1332012);

    // Pirate
    pub const BROWN_PAULIE_BOOTS: ItemId = ItemId(1072294);
    pub const PRIME_HANDS: ItemId = ItemId(1482004);
    pub const COLD_MIND: ItemId = ItemId(1492004);
    pub const BROWN_POLLARD: ItemId = ItemId(1052107);

    // Three snails
    pub const SNAIL_SHELL: ItemId = ItemId(4000019);
    pub const BLUE_SNAIL_SHELL: ItemId = ItemId(4000000);
    pub const RED_SNAIL_SHELL: ItemId = ItemId(4000016);

    // Special SCROLL
    pub const COLD_PROTECTION_SCROLL: ItemId = ItemId(2041058);
    pub const SPIKES_SCROLL: ItemId = ItemId(2040727);
    pub const VEGAS_SPELL_10: ItemId = ItemId(5610000);
    pub const VEGAS_SPELL_60: ItemId = ItemId(5610001);
    pub const CHAOS_SCROLL_60: ItemId = ItemId(2049100);
    pub const LIAR_TREE_SAP: ItemId = ItemId(2049101);
    pub const MAPLE_SYRUP: ItemId = ItemId(2049102);
    pub const WHITE_SCROLL: ItemId = ItemId(2340000);
    pub const CLEAN_SLATE_1: ItemId = ItemId(2049000);
    pub const CLEAN_SLATE_3: ItemId = ItemId(2049001);
    pub const CLEAN_SLATE_5: ItemId = ItemId(2049002);
    pub const CLEAN_SLATE_20: ItemId = ItemId(2049003);
    pub const RING_STR_100_SCROLL: ItemId = ItemId(2041100);
    pub const DRAGON_STONE_SCROLL: ItemId = ItemId(2041200);
    pub const BELT_STR_100_SCROLL: ItemId = ItemId(2041300);

    // Cure debuff
    pub const ALL_CURE_POTION: ItemId = ItemId(2050004);
    pub const EYEDROP: ItemId = ItemId(2050001);
    pub const TONIC: ItemId = ItemId(2050002);
    pub const HOLY_WATER: ItemId = ItemId(2050003);
    pub const ANTI_BANISH_SCROLL: ItemId = ItemId(2030100);
    pub const DOJO_PARTY_ALL_CURE: ItemId = ItemId(2022433);
    pub const CARNIVAL_PARTY_ALL_CURE: ItemId = ItemId(2022163);
    pub const WHITE_ELIXIR: ItemId = ItemId(2022544);

    // Special effect
    pub const PHARAOHS_BLESSING_1: ItemId = ItemId(2022585);
    pub const PHARAOHS_BLESSING_2: ItemId = ItemId(2022586);
    pub const PHARAOHS_BLESSING_3: ItemId = ItemId(2022587);
    pub const PHARAOHS_BLESSING_4: ItemId = ItemId(2022588);

    // Evolve pet
    pub const DRAGON_PET: ItemId = ItemId(5000028);
    pub const ROBO_PET: ItemId = ItemId(5000047);

    // Pet equip
    pub const MESO_MAGNET: ItemId = ItemId(1812000);
    pub const ITEM_POUCH: ItemId = ItemId(1812001);
    pub const ITEM_IGNORE: ItemId = ItemId(1812007);

    // Expirable pet
    pub const PET_SNAIL: ItemId = ItemId(5000054);

    // Permanent pet
    pub const PERMA_PINK_BEAN: ItemId = ItemId(5000060);
    pub const PERMA_KINO: ItemId = ItemId(5000100);
    pub const PERMA_WHITE_TIGER: ItemId = ItemId(5000101);
    pub const PERMA_MINI_YETI: ItemId = ItemId(5000102);

    // Maker
    pub const BASIC_MONSTER_CRYSTAL_1: ItemId = ItemId(4260000);
    pub const BASIC_MONSTER_CRYSTAL_2: ItemId = ItemId(4260001);
    pub const BASIC_MONSTER_CRYSTAL_3: ItemId = ItemId(4260002);
    pub const INTERMEDIATE_MONSTER_CRYSTAL_1: ItemId = ItemId(4260003);
    pub const INTERMEDIATE_MONSTER_CRYSTAL_2: ItemId = ItemId(4260004);
    pub const INTERMEDIATE_MONSTER_CRYSTAL_3: ItemId = ItemId(4260005);
    pub const ADVANCED_MONSTER_CRYSTAL_1: ItemId = ItemId(4260006);
    pub const ADVANCED_MONSTER_CRYSTAL_2: ItemId = ItemId(4260007);
    pub const ADVANCED_MONSTER_CRYSTAL_3: ItemId = ItemId(4260008);

    // NPC weather (PQ)
    pub const NPC_WEATHER_GROWLIE: ItemId = ItemId(5120016); // Henesys PQ

    // Safety charm
    pub const SAFETY_CHARM: ItemId = ItemId(5130000);
    pub const EASTER_BASKET: ItemId = ItemId(4031283);
    pub const EASTER_CHARM: ItemId = ItemId(4140903);

    // Engagement box
    pub const ENGAGEMENT_BOX_MOONSTONE: ItemId = ItemId(2240000);
    pub const ENGAGEMENT_BOX_STAR: ItemId = ItemId(2240001);
    pub const ENGAGEMENT_BOX_GOLDEN: ItemId = ItemId(2240002);
    pub const ENGAGEMENT_BOX_SILVER: ItemId = ItemId(2240003);
    pub const EMPTY_ENGAGEMENT_BOX_MOONSTONE: ItemId = ItemId(4031357);
    pub const ENGAGEMENT_RING_MOONSTONE: ItemId = ItemId(4031358);
    pub const EMPTY_ENGAGEMENT_BOX_STAR: ItemId = ItemId(4031359);
    pub const ENGAGEMENT_RING_STAR: ItemId = ItemId(4031360);
    pub const EMPTY_ENGAGEMENT_BOX_GOLDEN: ItemId = ItemId(4031361);
    pub const ENGAGEMENT_RING_GOLDEN: ItemId = ItemId(4031362);
    pub const EMPTY_ENGAGEMENT_BOX_SILVER: ItemId = ItemId(4031363);
    pub const ENGAGEMENT_RING_SILVER: ItemId = ItemId(4031364);

    // Wedding etc
    pub const PARENTS_BLESSING: ItemId = ItemId(4031373);
    pub const OFFICIATORS_PERMISSION: ItemId = ItemId(4031374);
    pub const ONYX_CHEST_FOR_COUPLE: ItemId = ItemId(4031424);

    // Wedding ticket
    pub const NORMAL_WEDDING_TICKET_CATHEDRAL: ItemId = ItemId(5251000);
    pub const NORMAL_WEDDING_TICKET_CHAPEL: ItemId = ItemId(5251001);
    pub const PREMIUM_WEDDING_TICKET_CHAPEL: ItemId = ItemId(5251002);
    pub const PREMIUM_WEDDING_TICKET_CATHEDRAL: ItemId = ItemId(5251003);

    // Wedding reservation
    pub const PREMIUM_CATHEDRAL_RESERVATION_RECEIPT: ItemId = ItemId(4031375);
    pub const PREMIUM_CHAPEL_RESERVATION_RECEIPT: ItemId = ItemId(4031376);
    pub const NORMAL_CATHEDRAL_RESERVATION_RECEIPT: ItemId = ItemId(4031480);
    pub const NORMAL_CHAPEL_RESERVATION_RECEIPT: ItemId = ItemId(4031481);

    // Wedding invite
    pub const INVITATION_CHAPEL: ItemId = ItemId(4031377);
    pub const INVITATION_CATHEDRAL: ItemId = ItemId(4031395);
    pub const RECEIVED_INVITATION_CHAPEL: ItemId = ItemId(4031406);
    pub const RECEIVED_INVITATION_CATHEDRAL: ItemId = ItemId(4031407);

    pub const CARAT_RING_BASE: ItemId = ItemId(1112300); // Unsure about math on this and the following one
    pub const CARAT_RING_BOX_BASE: ItemId = ItemId(2240004);
    pub const CARAT_RING_BOX_MAX: ItemId = ItemId(2240015);

    pub const ENGAGEMENT_BOX_MIN: ItemId = Self::ENGAGEMENT_BOX_MOONSTONE;
    pub const ENGAGEMENT_BOX_MAX: ItemId = Self::CARAT_RING_BOX_MAX;
    pub const ENGAGEMENT_BOX_RANGE: RangeInclusive<ItemId> =
        (Self::ENGAGEMENT_BOX_MIN..=Self::ENGAGEMENT_BOX_MAX);

    // Wedding ring
    pub const WEDDING_RING_MOONSTONE: ItemId = ItemId(1112803);
    pub const WEDDING_RING_STAR: ItemId = ItemId(1112806);
    pub const WEDDING_RING_GOLDEN: ItemId = ItemId(1112807);
    pub const WEDDING_RING_SILVER: ItemId = ItemId(1112809);

    // Priority buff
    pub const ROSE_SCENT: ItemId = ItemId(2022631);
    pub const FREESIA_SCENT: ItemId = ItemId(2022632);
    pub const LAVENDER_SCENT: ItemId = ItemId(2022633);

    // Cash shop
    pub const WHEEL_OF_FORTUNE: ItemId = ItemId(5510000);
    pub const CASH_SHOP_SURPRISE: ItemId = ItemId(5222000);
    pub const EXP_COUPON_2X_4H: ItemId = ItemId(5211048);
    pub const DROP_COUPON_2X_4H: ItemId = ItemId(5360042);
    pub const EXP_COUPON_3X_2H: ItemId = ItemId(5211060);
    pub const QUICK_DELIVERY_TICKET: ItemId = ItemId(5330000);
    pub const CHALKBOARD_1: ItemId = ItemId(5370000);
    pub const CHALKBOARD_2: ItemId = ItemId(5370001);
    pub const REMOTE_GACHAPON_TICKET: ItemId = ItemId(5451000);
    pub const AP_RESET: ItemId = ItemId(5050000);
    pub const NAME_CHANGE: ItemId = ItemId(5400000);
    pub const WORLD_TRANSFER: ItemId = ItemId(5401000);
    pub const MAPLE_LIFE_B: ItemId = ItemId(5432000);
    pub const VICIOUS_HAMMER: ItemId = ItemId(5570000);

    pub const NX_CARD_100: ItemId = ItemId(4031865);
    pub const NX_CARD_250: ItemId = ItemId(4031866);

    // <Face> expression
    pub const FACE_EXPRESSION_MIN: ItemId = ItemId(5160000);
    pub const FACE_EXPRESSION_MAX: ItemId = ItemId(5160014);
    pub const FACE_EXPRESSION_RANGE: RangeInclusive<ItemId> =
        (Self::FACE_EXPRESSION_MIN..=Self::FACE_EXPRESSION_MAX);

    // New Year card
    pub const NEW_YEARS_CARD: ItemId = ItemId(2160101);
    pub const NEW_YEARS_CARD_SEND: ItemId = ItemId(4300000);
    pub const NEW_YEARS_CARD_RECEIVED: ItemId = ItemId(4301000);

    // Popular owl items
    pub const WORK_GLOVES: ItemId = ItemId(1082002);
    pub const STEELY_THROWING_KNIVES: ItemId = ItemId(2070005);
    pub const ILBI_THROWING_STARS: ItemId = ItemId(2070006);
    pub const OWL_BALL_MASK: ItemId = ItemId(1022047);
    pub const PINK_ADVENTURER_CAPE: ItemId = ItemId(1102041);
    pub const CLAW_30_SCROLL: ItemId = ItemId(2044705);
    pub const HELMET_60_ACC_SCROLL: ItemId = ItemId(2040017);
    pub const MAPLE_SHIELD: ItemId = ItemId(1092030);
    pub const GLOVES_ATT_60_SCROLL: ItemId = ItemId(2040804);

    // Henesys PQ
    pub const GREEN_PRIMROSE_SEED: ItemId = ItemId(4001095);
    pub const PURPLE_PRIMROSE_SEED: ItemId = ItemId(4001096);
    pub const PINK_PRIMROSE_SEED: ItemId = ItemId(4001097);
    pub const BROWN_PRIMROSE_SEED: ItemId = ItemId(4001098);
    pub const YELLOW_PRIMROSE_SEED: ItemId = ItemId(4001099);
    pub const BLUE_PRIMROSE_SEED: ItemId = ItemId(4001100);
    pub const MOON_BUNNYS_RICE_CAKE: ItemId = ItemId(4001101);

    // Catch mobs items
    pub const PHEROMONE_PERFUME: ItemId = ItemId(2270000);
    pub const POUCH: ItemId = ItemId(2270001);
    pub const GHOST_SACK: ItemId = ItemId(4031830);
    pub const ARPQ_ELEMENT_ROCK: ItemId = ItemId(2270002);
    pub const ARPQ_SPIRIT_JEWEL: ItemId = ItemId(4031868);
    pub const MAGIC_CANE: ItemId = ItemId(2270003);
    pub const TAMED_RUDOLPH: ItemId = ItemId(4031887);
    pub const TRANSPARENT_MARBLE_1: ItemId = ItemId(2270005);
    pub const MONSTER_MARBLE_1: ItemId = ItemId(2109001);
    pub const TRANSPARENT_MARBLE_2: ItemId = ItemId(2270006);
    pub const MONSTER_MARBLE_2: ItemId = ItemId(2109002);
    pub const TRANSPARENT_MARBLE_3: ItemId = ItemId(2270007);
    pub const MONSTER_MARBLE_3: ItemId = ItemId(2109003);
    pub const EPQ_PURIFICATION_MARBLE: ItemId = ItemId(2270004);
    pub const EPQ_MONSTER_MARBLE: ItemId = ItemId(4001169);
    pub const FISH_NET: ItemId = ItemId(2270008);
    pub const FISH_NET_WITH_A_CATCH: ItemId = ItemId(2022323);

    // Mount
    pub const BATTLESHIP: ItemId = ItemId(1932000);

    // Explorer mount
    pub const HOG: ItemId = ItemId(1902000);
    pub const SILVER_MANE: ItemId = ItemId(1902001);
    pub const RED_DRACO: ItemId = ItemId(1902002);
    pub const EXPLORER_SADDLE: ItemId = ItemId(1912000);

    // Cygnus mount
    pub const MIMIANA: ItemId = ItemId(1902005);
    pub const MIMIO: ItemId = ItemId(1902006);
    pub const SHINJOU: ItemId = ItemId(1902007);
    pub const CYGNUS_SADDLE: ItemId = ItemId(1912005);

    // Dev equips
    pub const GREEN_HEADBAND: ItemId = ItemId(1002067);
    pub const TIMELESS_NIBLEHEIM: ItemId = ItemId(1402046);
    pub const BLUE_KORBEN: ItemId = ItemId(1082140);
    pub const MITHRIL_PLATINE_PANTS: ItemId = ItemId(1060091);
    pub const BLUE_CARZEN_BOOTS: ItemId = ItemId(1072154);
    pub const MITHRIL_PLATINE: ItemId = ItemId(1040103);

    pub const PERMANENT_PETS: [ItemId; 4] = [
        Self::PERMA_PINK_BEAN,
        Self::PERMA_KINO,
        Self::PERMA_WHITE_TIGER,
        Self::PERMA_MINI_YETI,
    ];

    pub const OWL_ITEMS: [ItemId; 10] = [
        Self::WORK_GLOVES,
        Self::STEELY_THROWING_KNIVES,
        Self::ILBI_THROWING_STARS,
        Self::OWL_BALL_MASK,
        Self::PINK_ADVENTURER_CAPE,
        Self::CLAW_30_SCROLL,
        Self::WHITE_SCROLL,
        Self::HELMET_60_ACC_SCROLL,
        Self::MAPLE_SHIELD,
        Self::GLOVES_ATT_60_SCROLL,
    ];
}
