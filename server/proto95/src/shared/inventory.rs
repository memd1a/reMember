use moople_derive::MooplePacket;
use moople_packet::{
    maple_packet_enum,
    proto::{
        time::{MapleTime, Ticks},
        MapleList8, list::{MapleIndexListZ16, MapleIndexListZ8},
    },
};
use num_enum::TryFromPrimitive;

use super::item::Item;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum EquippedSlot {
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
    PetEquip = 114,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum CashEquippedSlot {
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
    TamedMob = 118,
}

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
pub struct GatherItemsPacket {
    timestamp: Ticks,
    inv_ty: u8,
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
pub struct InventoryOperationInfo {
    inventory_type: u8,
    pos: u16,
}

#[derive(Debug, MooplePacket)]
pub struct InventoryOperationInfoWithArg {
    inventory_type: u8,
    pos: u16,
    arg: u16,
}

maple_packet_enum!(
    InventoryOperation,
    u8,
    Add(InventoryOperationInfo) => 0,
    ChangeCount(InventoryOperationInfoWithArg) => 1,
    Swap(InventoryOperationInfoWithArg) => 2,
    Remove(InventoryOperationInfo) => 3
);

#[derive(Debug, MooplePacket)]
pub struct InventoryOperationsPacket {
    update_tick: bool,
    operations: MapleList8<InventoryOperation>, //TODO move internal tail byte
}
