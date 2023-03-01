use num_enum::TryFromPrimitive;
use proto95::{id::ItemId, shared::inventory::EquippedSlot};

use crate::model::item::{EquipItem, StackItem};

use super::{Inventory, InventoryError, InventoryItem};

pub trait InventorySlotIndex {
    fn from_index(ix: usize) -> Self;
    fn to_index(&self) -> usize;
}

impl InventorySlotIndex for usize {
    fn from_index(ix: usize) -> Self {
        ix
    }

    fn to_index(&self) -> usize {
        *self
    }
}

impl InventorySlotIndex for EquippedSlot {
    fn from_index(ix: usize) -> Self {
        EquippedSlot::try_from_primitive(ix as u8).unwrap()
    }

    fn to_index(&self) -> usize {
        *self as usize
    }
}

pub trait InventoryExt<const CAP: usize> {
    type Slot: InventorySlotIndex;
    type Item: InventoryItem;

    fn get_inner(&self) -> &Inventory<CAP, Self::Item>;
    fn get_inner_mut(&mut self) -> &mut Inventory<CAP, Self::Item>;

    fn len(&self) -> usize {
        self.get_inner().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_not_empty(&self) -> bool {
        self.len() != 0
    }

    fn slots(&self) -> usize {
        self.get_inner().slots()
    }

    fn remove(&mut self, slot: Self::Slot) -> Option<Self::Item> {
        self.get_inner_mut().remove(slot.to_index()).unwrap()
    }

    fn swap(&mut self, slot_a: Self::Slot, slot_b: Self::Slot) {
        self.get_inner_mut()
            .swap(slot_a.to_index(), slot_b.to_index())
            .unwrap();
    }

    fn set(&mut self, slot: Self::Slot, item: Self::Item) {
        self.get_inner_mut().set_slot(slot.to_index(), item)
    }

    fn get_mut(&mut self, slot: Self::Slot) -> Option<&mut Self::Item> {
        self.get_inner_mut()
            .get_mut(slot.to_index())
            .ok()
            .and_then(|v| v)
    }

    fn get(&self, slot: Self::Slot) -> Option<&Self::Item> {
        self.get_inner().get(slot.to_index()).ok().and_then(|v| v)
    }

    fn load(
        &mut self,
        items: impl Iterator<Item = (Self::Slot, Self::Item)>,
    ) -> Result<(), InventoryError> {
        for (slot, item) in items {
            self.set(slot, item);
        }

        Ok(())
    }
}

const EQUIPPED_CAP: usize = 96;
const INV_ITEM_CAP: usize = 180;

#[derive(Debug, Clone)]
pub struct EquipItemSlot {
    pub item_id: ItemId,
    pub item: Box<EquipItem>,
}

impl From<EquipItem> for EquipItemSlot {
    fn from(value: EquipItem) -> Self {
        Self {
            item_id: value.item_id,
            item: Box::new(value),
        }
    }
}

impl InventoryItem for EquipItemSlot {
    fn is_one_of_a_kind(&self) -> bool {
        false
    }

    fn stack_size(&self) -> usize {
        1
    }

    fn id(&self) -> u32 {
        self.item_id.0
    }
}

#[derive(Debug, Clone)]
pub struct EquippedInventory<const CAP: usize = EQUIPPED_CAP>(Inventory<CAP, EquipItemSlot>);

impl<const CAP: usize> InventoryExt<CAP> for EquippedInventory<CAP> {
    type Slot = EquippedSlot;

    type Item = EquipItemSlot;

    fn get_inner(&self) -> &Inventory<CAP, Self::Item> {
        &self.0
    }

    fn get_inner_mut(&mut self) -> &mut Inventory<CAP, Self::Item> {
        &mut self.0
    }
}

impl<const CAP: usize> EquippedInventory<CAP> {
    pub fn new(slots: usize) -> Self {
        Self(Inventory::new(slots))
    }

    pub fn iter(&self) -> impl Iterator<Item = (EquippedSlot, &EquipItemSlot)> {
        self.0
            .items_with_slot()
            .map(|(slot, item)| (EquippedSlot::try_from(slot as u8).unwrap(), item))
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut EquipItemSlot> {
        self.0.items_mut()
    }
}

#[derive(Debug, Clone)]
pub struct EquipInventory<const CAP: usize = EQUIPPED_CAP>(Inventory<CAP, EquipItemSlot>);

impl<const CAP: usize> InventoryExt<CAP> for EquipInventory<CAP> {
    type Slot = usize;
    type Item = EquipItemSlot;

    fn get_inner(&self) -> &Inventory<CAP, Self::Item> {
        &self.0
    }

    fn get_inner_mut(&mut self) -> &mut Inventory<CAP, Self::Item> {
        &mut self.0
    }
}

impl<const CAP: usize> EquipInventory<CAP> {
    pub fn new(slots: usize) -> Self {
        Self(Inventory::new(slots))
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &EquipItemSlot)> {
        self.0.items_with_slot().map(|(slot, item)| (slot, item))
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut EquipItemSlot> {
        self.0.items_mut()
    }
}

#[derive(Debug, Clone)]
pub struct StackItemSlot {
    pub item_id: ItemId,
    pub quantity: usize,
    pub item: Box<StackItem>,
}

impl From<StackItem> for StackItemSlot {
    fn from(value: StackItem) -> Self {
        Self {
            item_id: value.item_id,
            quantity: value.quantity as usize,
            item: Box::new(value),
        }
    }
}

impl InventoryItem for StackItemSlot {
    fn is_one_of_a_kind(&self) -> bool {
        false
    }

    fn stack_size(&self) -> usize {
        5000
    }

    fn id(&self) -> u32 {
        self.item_id.0
    }
}

#[derive(Debug, Clone)]
pub struct StackInventory<const CAP: usize = INV_ITEM_CAP>(Inventory<CAP, StackItemSlot>);

impl<const CAP: usize> InventoryExt<CAP> for StackInventory<CAP> {
    type Slot = usize;
    type Item = StackItemSlot;

    fn get_inner(&self) -> &Inventory<CAP, Self::Item> {
        &self.0
    }

    fn get_inner_mut(&mut self) -> &mut Inventory<CAP, Self::Item> {
        &mut self.0
    }
}

impl<const CAP: usize> StackInventory<CAP> {
    pub fn new(slots: usize) -> Self {
        Self(Inventory::new(slots))
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, &StackItemSlot)> {
        self.0.items_with_slot()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut StackItemSlot> {
        self.0.items_mut()
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum InventoryType {
    Equipped = 1,
    MaskedEquipped = 2,
    Equip = 3,
    Use = 4,
    Misc = 5,
    Etc = 6,
    Cash = 7,
}

impl InventoryType {
    pub fn is_equip(&self) -> bool {
        matches!(
            self,
            InventoryType::Equipped | InventoryType::Equip | InventoryType::Cash
        )
    }

    pub fn is_stack(&self) -> bool {
        !self.is_equip()
    }
}

#[derive(Debug, Clone)]
pub struct InventorySet {
    pub equipped: EquippedInventory,
    pub masked_equipped: EquippedInventory,
    pub equip: EquipInventory,
    pub use_: StackInventory,
    pub misc: StackInventory,
    pub etc: StackInventory,
    pub cash: StackInventory,
}

impl InventorySet {
    pub fn with_default_slots() -> Self {
        const DEFAULT_SLOTS: usize = 48;
        Self {
            equipped: EquippedInventory::new(EQUIPPED_CAP),
            masked_equipped: EquippedInventory::new(EQUIPPED_CAP),
            equip: EquipInventory::new(DEFAULT_SLOTS),
            use_: StackInventory::new(DEFAULT_SLOTS),
            misc: StackInventory::new(DEFAULT_SLOTS),
            etc: StackInventory::new(DEFAULT_SLOTS),
            cash: StackInventory::new(DEFAULT_SLOTS),
        }
    }

    pub fn get_stack_inventory_mut(
        &mut self,
        ty: InventoryType,
    ) -> anyhow::Result<&mut StackInventory> {
        Ok(match ty {
            InventoryType::Cash => &mut self.cash,
            InventoryType::Use => &mut self.use_,
            InventoryType::Misc => &mut self.misc,
            InventoryType::Etc => &mut self.etc,
            _ => anyhow::bail!("Invalid stack inventory"),
        })
    }

    pub fn get_stack_inventory(&self, ty: InventoryType) -> anyhow::Result<&StackInventory> {
        Ok(match ty {
            InventoryType::Cash => &self.cash,
            InventoryType::Use => &self.use_,
            InventoryType::Misc => &self.misc,
            InventoryType::Etc => &self.etc,
            _ => anyhow::bail!("Invalid stack inventory"),
        })
    }

    pub fn get_equipped_inventory_mut(
        &mut self,
        ty: InventoryType,
    ) -> anyhow::Result<&mut EquippedInventory> {
        Ok(match ty {
            InventoryType::Equipped => &mut self.equipped,
            InventoryType::MaskedEquipped => &mut self.equipped,
            _ => anyhow::bail!("Invalid equipped inventory"),
        })
    }

    pub fn get_equipped_inventory(&self, ty: InventoryType) -> anyhow::Result<&EquippedInventory> {
        Ok(match ty {
            InventoryType::Equipped => &self.equipped,
            InventoryType::MaskedEquipped => &self.equipped,
            _ => anyhow::bail!("Invalid equipped inventory"),
        })
    }

    pub fn slots(&self, ty: InventoryType) -> usize {
        if ty.is_stack() {
            self.get_stack_inventory(ty).unwrap().slots()
        } else {
            self.get_equipped_inventory(ty).unwrap().slots()
        }
    }
}
