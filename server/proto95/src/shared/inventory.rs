use moople_derive::MooplePacket;
use moople_packet::{
    maple_enum_code, maple_packet_enum, packet_opcode,
    proto::{
        list::{MapleIndexListZ16, MapleIndexListZ8},
        time::{MapleTime, Ticks},
        MapleList8,
    },
};

use crate::{id::ItemId, recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

use super::item::Item;

//TODO indexing
maple_enum_code!(
    InventoryType,
    u8,
    Equip = 1,
    Consume = 2,
    Install = 30,
    Etc = 4,
    Cash = 5,
    Equipped = 6,
    Special = 9,
    DragonEquipped = 10,
    MechanicEquipped = 11
);

maple_enum_code!(
    EquippedSlot,
    u8,
    None = 0,
    Hat = 1,
    Face = 2,
    EyeAccessory = 3,
    EarAccessory = 4,
    Top = 5,
    Bottom = 6,
    Shoes = 7,
    Gloves = 8,
    Cape = 9,
    Shield = 10,
    Weapon = 11,
    Ring1 = 12,
    Ring2 = 13,
    //14??
    Ring3 = 15,
    Ring4 = 16,
    Pendant1 = 17,
    TamedMob = 18,
    Saddle = 19,
    Medal = 49,
    Belt = 50,
    PetEquip = 114
);

maple_enum_code!(
    CashEquippedSlot,
    u8,
    Hat = 101,
    Face = 102,
    Eye = 103,
    Top = 104,
    Overall = 105,
    Bottom = 106,
    Shoes = 107,
    Gloves = 108,
    Cape = 109,
    Shield = 110,
    Weapon = 111,
    Ring1 = 112,
    Ring2 = 113,
    // 14??
    Ring3 = 115,
    Ring4 = 116,
    Pendant = 117,
    TamedMob = 118
);

#[derive(Debug, MooplePacket)]
pub struct InventoryInfo {
    slot_limits: [u8; 5],
    timestamp: MapleTime,
    equipped: MapleIndexListZ16<Item>,
    equipped_cash: MapleIndexListZ16<Item>,
    equip: MapleIndexListZ16<Item>,
    pad: u16,
    _use: MapleIndexListZ8<Item>,
    setup: MapleIndexListZ8<Item>,
    etc: MapleIndexListZ8<Item>,
    cash: MapleIndexListZ8<Item>,
}

#[derive(Debug, MooplePacket)]
pub struct SortItemsPacket {
    timestamp: Ticks,
    inv_ty: u8,
}

#[derive(Debug, MooplePacket)]
pub struct MoveItemsPacket {
    timestamp: Ticks,
    inv_ty: u8,
    slot: u16,
    action: u16,
    count: u16,
}

#[derive(Debug, MooplePacket)]
pub struct UseItemPacket {
    timestamp: Ticks,
    slot: u16,
    item_id: u16,
}
#[derive(Debug, MooplePacket)]
pub struct ScrollFlag(u16);

impl ScrollFlag {
    pub fn unknown(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn use_white_scroll(&self) -> bool {
        self.0 & 2 == 2
    }
}

#[derive(Debug, MooplePacket)]
pub struct ScrollEquipPacket {
    timestamp: Ticks,
    src: u16,
    dst: u16,
    flag: ScrollFlag,
}

#[derive(Debug, MooplePacket)]
pub struct InvOpAdd {
    pub inv_type: InventoryType,
    pub pos: u16,
    pub item: Item,
}

#[derive(Debug, MooplePacket)]
pub struct InvOpUpdateQuantity {
    pub inv_type: InventoryType,
    pub pos: u16,
    pub quantity: u16,
}

#[derive(Debug, MooplePacket)]
pub struct InvOpMove {
    pub inv_type: InventoryType,
    pub pos: u16,
    pub new_pos: u16,
}

#[derive(Debug, MooplePacket)]
pub struct InvOpRemove {
    pub inv_type: InventoryType,
    pub pos: u16,
}

#[derive(Debug, MooplePacket)]
pub struct InvOpUpdateExp {
    pub inv_type: InventoryType,
    pub pos: u16,
}

maple_packet_enum!(
    InventoryOperation,
    u8,
    Add(InvOpAdd) => 0,
    UpdateQuantity(InvOpUpdateQuantity) => 1,
    Move(InvOpMove) => 2,
    Remove(InvOpRemove) => 3,
    UpdateExp(InvOpUpdateExp) => 4
);

#[derive(Debug, MooplePacket)]
pub struct InventoryOperationsResp {
    pub reset_excl: bool,
    pub operations: MapleList8<InventoryOperation>,
    pub secondary_stat_changed: bool, //TODO optional tail byte
                                      // Updated when operation is done on equip inv, either Move(2), Remove(3)
}
packet_opcode!(InventoryOperationsResp, SendOpcodes::InventoryOperation);

#[derive(MooplePacket, Debug)]
pub struct InvGrowResp {
    pub inv_type: InventoryType, //TODO only first 6 inv can grow
    pub new_size: u8,
}
packet_opcode!(InvGrowResp, SendOpcodes::InventoryGrow);

#[derive(MooplePacket, Debug)]
pub struct InvChangeSlotPosReq {
    pub ticks: Ticks,
    pub inv_type: InventoryType,
    pub old_pos: u16,
    pub new_pos: u16,
    pub count: u16,
}
packet_opcode!(
    InvChangeSlotPosReq,
    RecvOpcodes::UserChangeSlotPositionRequest
);

#[derive(MooplePacket, Debug)]
pub struct InvSortRequest {
    pub ticks: Ticks,
    pub inv_type: InventoryType,
}
packet_opcode!(InvSortRequest, RecvOpcodes::UserSortItemRequest);

// Use an item like magnifying glass, maybe hammer aswell?
#[derive(MooplePacket, Debug)]
pub struct ItemReleaseReq {
    pub ticks: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
}
packet_opcode!(ItemReleaseReq, RecvOpcodes::UserItemReleaseRequest);

#[derive(Debug, MooplePacket)]
pub struct GatherItemReq {
    pub timestamp: Ticks,
    pub inv_ty: InventoryType,
}
packet_opcode!(GatherItemReq, RecvOpcodes::UserGatherItemRequest);

#[derive(Debug, MooplePacket)]
pub struct ItemOptionUpgradeReq {
    pub timestamp: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
    pub enchant_skill: bool,
}
packet_opcode!(
    ItemOptionUpgradeReq,
    RecvOpcodes::UserItemOptionUpgradeItemUseRequest
);

#[derive(Debug, MooplePacket)]
pub struct ItemHyperUpgradeReq {
    pub timestamp: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
    pub enchant_skill: bool,
}
packet_opcode!(
    ItemHyperUpgradeReq,
    RecvOpcodes::UserHyperUpgradeItemUseRequest
);

#[derive(Debug, MooplePacket)]
pub struct ItemUpgradeReq {
    pub timestamp: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
    pub white_scroll_slot: u16,
    pub enchant_skill: bool,
}
packet_opcode!(ItemUpgradeReq, RecvOpcodes::UserUpgradeItemUseRequest);

#[derive(Debug, MooplePacket)]
pub struct TamingMobUseFoodReq {
    pub timestamp: Ticks,
    pub food_slot: u16,
    pub item_id: ItemId,
}
packet_opcode!(
    TamingMobUseFoodReq,
    RecvOpcodes::UserTamingMobFoodItemUseRequest
);

#[derive(Debug, MooplePacket)]
pub struct ItemOpenUIReq {
    pub timestamp: Ticks,
    pub slot: u16,
    pub item_id: ItemId,
}
packet_opcode!(ItemOpenUIReq, RecvOpcodes::UserUIOpenItemUseRequest);

#[derive(Debug, MooplePacket)]
pub struct ItemLearnSkillReq {
    pub timestamp: Ticks,
    pub slot: u16,
    pub item_id: ItemId,
}
packet_opcode!(ItemLearnSkillReq, RecvOpcodes::UserSkillLearnItemUseRequest);

#[derive(Debug, MooplePacket)]
pub struct UserSitReq {
    pub seat_id: u16,
}

impl UserSitReq {
    pub fn get_up() -> Self {
        Self::seat(u16::MAX)
    }

    pub fn seat(seat_id: u16) -> Self {
        Self { seat_id }
    }
}
packet_opcode!(UserSitReq, RecvOpcodes::UserSitRequest);
