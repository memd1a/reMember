pub mod inv;
mod item_stack;

use std::fmt::Debug;

use arrayvec::ArrayVec;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InventoryError {
    #[error("Empty slot {0}")]
    EmptySlot(usize),
    #[error("Remove quantity({remove_quantity}) higher than quantity({quantity}) for slot {slot}")]
    RemoveTooMuch {
        remove_quantity: usize,
        quantity: usize,
        slot: usize,
    },
    #[error("Slot is out of range: {0}")]
    OutOfRange(usize),
    #[error("Inventory is full")]
    Full,
    #[error("Item {0} is one-of-a-kind and already exists in the inventory")]
    OneOfAKindConflict(u32),
    #[error("Item stacks left({left}) and right({right}) are not merge-able")]
    InvalidMergeId { left: u32, right: u32 },
}
pub trait InventoryItem {
    fn is_one_of_a_kind(&self) -> bool;
    fn stack_size(&self) -> usize;
    fn id(&self) -> u32;
}

/// Storage with items sorted by their ID
#[derive(Debug, Clone)]
pub struct SortedItemVec<const CAP: usize, Item>(ArrayVec<Item, CAP>);

impl<const CAP: usize, Item: InventoryItem> Default for SortedItemVec<CAP, Item> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<const CAP: usize, Item: InventoryItem> SortedItemVec<CAP, Item> {
    /// Binary searches the vec for the given Item ID
    /// If an item with the ID is found It returns Ok
    /// containing the first position of the item with the given ID
    /// Otherwise It returns the position where the item is supposed to be inserted
    fn binary_search_by_id(&self, id: &u32) -> Result<usize, usize> {
        self.0.binary_search_by(|item| item.id().cmp(id))
    }

    fn find_index_by_id(&self, id: &u32) -> Option<usize> {
        self.binary_search_by_id(id).ok()
    }

    fn find_first_index_by_id(&self, id: &u32) -> Option<usize> {
        // Since the array is always sorted the partition point item.id < id
        // is the first index
        let ix = self.0.partition_point(|item| item.id() < *id);
        (ix < CAP).then_some(ix)
    }

    fn find_insert_index_by_id(&self, id: &u32) -> usize {
        self.binary_search_by_id(id).unwrap_or_else(|ix| ix)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.0.is_full()
    }

    pub fn get(&self, ix: usize) -> &Item {
        &self.0[ix]
    }

    pub fn get_mut(&mut self, ix: usize) -> &mut Item {
        &mut self.0[ix]
    }

    pub fn contains_id(&self, id: u32) -> bool {
        self.find_index_by_id(&id).is_some()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, item: Item) -> usize {
        if self.is_full() {
            panic!("Item Vector is full")
        }

        let insert_ix = self.find_insert_index_by_id(&item.id());
        self.0.insert(insert_ix, item);
        self.0.len() - 1
    }

    pub fn try_add(&mut self, item: Item) -> Result<usize, Item> {
        if self.is_full() {
            return Err(item);
        }

        let insert_ix = self.find_insert_index_by_id(&item.id());
        self.0.insert(insert_ix, item);
        Ok(self.0.len() - 1)
    }

    pub fn remove(&mut self, ix: usize) -> Item {
        self.0.remove(ix)
    }

    pub fn get_all_by_id(&self, id: u32) -> impl Iterator<Item = &Item> + '_ {
        let ix = self.find_first_index_by_id(&id).unwrap_or(0);
        self.0.iter().skip(ix).take_while(move |i| i.id() == id)
    }

    pub fn get_all_by_id_mut(&mut self, id: u32) -> impl Iterator<Item = &mut Item> + '_ {
        let ix = self.find_first_index_by_id(&id).unwrap_or(0);
        self.0.iter_mut().skip(ix).take_while(move |i| i.id() == id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Item> + '_ {
        self.0.iter()
    }

    #[cfg(test)]
    pub(crate) fn test_check_sorted(&self) -> bool {
        use crate::util::iter_is_sorted;

        iter_is_sorted(self.iter().map(|i| i.id()))
    }
}

const MAX_CAP: usize = u8::MAX as usize;

#[derive(Clone)]
pub struct Inventory<const CAP: usize, Item> {
    pub(crate) slots: usize,
    pub(crate) items: SortedItemVec<CAP, Item>,
    slot_mapping: [Option<u8>; CAP],
}

impl<const CAP: usize, Item> Debug for Inventory<CAP, Item>
where
    Item: Debug + InventoryItem,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.items_with_slot().map(|(_, item)| item))
            .finish()
    }
}

impl<const CAP: usize, Item> Inventory<CAP, Item>
where
    Item: InventoryItem,
{
    pub fn new(slots: usize) -> Self {
        if CAP > MAX_CAP {
            panic!("Inventory Capacity {CAP} higher than the maximum CAP: {MAX_CAP}");
        }

        if slots > CAP {
            panic!("Inventory slots({slots}) must be lower than the Capacity({CAP})");
        }

        Self {
            slots,
            slot_mapping: [None; CAP],
            items: SortedItemVec::default(),
        }
    }

    fn check_slot(&self, slot: usize) -> Result<(), InventoryError> {
        if slot >= self.slots {
            return Err(InventoryError::OutOfRange(slot));
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn slots(&self) -> usize {
        self.slots
    }

    pub fn is_full(&self) -> bool {
        self.items.len() <= self.slots
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn find_free_slot(&self) -> Option<usize> {
        self.slot_mapping
            .iter()
            .take(self.slots)
            .position(Option::is_none)
    }

    pub fn get(&self, slot: usize) -> Result<Option<&Item>, InventoryError> {
        self.check_slot(slot)?;
        Ok(self.slot_mapping[slot].map(|ix| self.items.get(ix as usize)))
    }

    pub fn get_mut(&mut self, slot: usize) -> Result<Option<&mut Item>, InventoryError> {
        self.check_slot(slot)?;
        Ok(self.slot_mapping[slot].map(|ix| self.items.get_mut(ix as usize)))
    }

    fn update_add(&mut self, add_index: usize) {
        let add_index = add_index as u8;
        self.slot_mapping
            .iter_mut()
            .filter_map(|ix| ix.as_mut())
            .for_each(|ix| {
                if *ix >= add_index {
                    *ix += 1;
                }
            })
    }

    fn update_remove(&mut self, rm_index: usize) {
        let rm_index = rm_index as u8;
        self.slot_mapping
            .iter_mut()
            .filter_map(|ix| ix.as_mut())
            .for_each(|ix| {
                if *ix > rm_index {
                    *ix -= 1;
                }
            })
    }

    pub fn try_add(&mut self, item: Item) -> Result<usize, Item> {
        let Some(free_slot) = self.find_free_slot()  else {
            return Err(item);
        };

        if !self.can_insert_item(&item) {
            return Err(item);
        }

        let ix = self.items.add(item);
        self.update_add(ix);
        self.slot_mapping[free_slot] = Some(ix as u8);
        Ok(ix)
    }

    pub fn remove(&mut self, slot: usize) -> Result<Option<Item>, InventoryError> {
        self.check_slot(slot)?;
        let item = self.slot_mapping[slot]
            .take()
            .map(|ix| {
                self.update_remove(ix as usize);
                self.items.remove(ix as usize)
            });

        Ok(item)
    }

    pub fn swap_with_other(
        &mut self,
        slot_a: usize,
        other: &mut Self,
        slot_b: usize,
    ) -> Result<(), InventoryError> {
        let item_a = self.get(slot_a)?;
        if let Some(item) = item_a {
            if !other.can_insert_item(item) {
                return Err(InventoryError::OneOfAKindConflict(item.id()));
            }
        }

        let item_b = other.get(slot_b)?;
        if let Some(item) = item_b {
            if !self.can_insert_item(item) {
                return Err(InventoryError::OneOfAKindConflict(item.id()));
            }
        }

        let item_a = self.remove(slot_a)?;
        let item_b = other.remove(slot_b)?;

        if let Some(item) = item_a {
            other.set_slot(slot_b, item);
        }

        if let Some(item) = item_b {
            self.set_slot(slot_a, item);
        }

        Ok(())
    }

    pub fn swap(&mut self, slot_a: usize, slot_b: usize) -> Result<(), InventoryError> {
        self.check_slot(slot_a)?;
        self.check_slot(slot_b)?;

        self.slot_mapping.swap(slot_a, slot_b);
        Ok(())
    }

    pub fn contains_id(&self, id: u32) -> bool {
        self.items.contains_id(id)
    }

    pub fn try_set_slot(&mut self, slot: usize, item: Item) -> Result<(), Item> {
        //TODO check slot here
        if self.slot_mapping[slot].is_some() {
            return Err(item);
        }

        let ix = self.items.add(item);
        self.slot_mapping[slot] = Some(ix as u8);
        Ok(())
    }

    pub fn set_slot(&mut self, slot: usize, item: Item) {
        if self.slot_mapping[slot].is_some() {
            panic!("Slot not empty for add");
        }

        let ix = self.items.add(item);
        self.slot_mapping[slot] = Some(ix as u8);
    }

    pub fn can_insert_item(&self, item: &Item) -> bool {
        if item.is_one_of_a_kind() {
            !self.contains_id(item.id())
        } else {
            true
        }
    }

    pub fn find_first_shift_slot(&self, after_slot: usize) -> Option<usize> {
        self.slot_mapping
            .iter()
            .skip(after_slot)
            .position(|slot| slot.is_some())
    }

    pub fn shift_slots(&mut self) {
        // Stores the last known position where an item was
        let mut last_shift_ix = 0;
        for gap in 0..CAP {
            //Check if this  is a gap
            if self.slot_mapping[gap].is_none() {
                let after_slot = last_shift_ix.max(gap + 1);
                //Find shift item
                let Some(shift_ix) = self.find_first_shift_slot(after_slot) else {
                    // No more items available
                    return;
                };

                //Indices are checked must work
                self.slot_mapping.swap(gap, shift_ix);
                last_shift_ix = shift_ix;
            }
        }
    }

    pub fn sort(&mut self) {
        // We know the underlying item array is already sorted
        // so sorting the inventory is as simple
        // as sorting the mappings by the index
        for slot in 0..self.slots {
            // If the index is set to a slot then we update it's slot
            let Some(ix) = self.slot_mapping[slot] else {
                continue
            };

            let ix = ix as usize;

            if ix != slot {
                self.slot_mapping.swap(slot, ix)
                //Log swap here
            }
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> + '_ {
        self.items.0.iter()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> + '_ {
        self.items.0.iter_mut()
    }

    pub fn items_with_slot(&self) -> impl Iterator<Item = (usize, &Item)> + '_ {
        self.slot_mapping
            .iter()
            .enumerate()
            .filter_map(|(slot, ix)| ix.map(|ix| (slot, self.items.get(ix as usize))))
    }
}

#[cfg(test)]
mod tests {

    use crate::helper::intentory::SortedItemVec;

    use super::{Inventory, InventoryItem};

    impl InventoryItem for u32 {
        fn is_one_of_a_kind(&self) -> bool {
            //Even IDs are one-of-a-kind
            self % 2 == 0
        }

        fn stack_size(&self) -> usize {
            if self.is_one_of_a_kind() {
                1
            } else {
                500
            }
        }

        fn id(&self) -> u32 {
            *self
        }
    }

    // Test sorted item list

    #[test]
    fn item_vec_insert_delete() {
        const CAP: usize = 32;
        const CAP32: u32 = CAP as u32;

        // Ascending order insert
        let mut items = SortedItemVec::<CAP, u32>::default();
        assert!(items.is_empty());
        //Insert in reverse order
        (1..=CAP).for_each(|i| {
            items.try_add(i as u32).unwrap();
        });
        //Check that items are in ascending sorted order
        itertools::assert_equal(items.iter().cloned(), 1..=CAP32);
        assert!(items.is_full());

        // Delete from start
        for i in 1..=CAP {
            assert_eq!(items.remove(0) as usize, i);
        }

        // Descending order insert
        let mut items = SortedItemVec::<CAP, u32>::default();
        //Insert in reverse order
        (1..=CAP).rev().for_each(|i| {
            items.try_add(i as u32).unwrap();
        });
        //Check that items are in ascending sorted order
        itertools::assert_equal(items.iter().cloned(), 1..=CAP32);
    }

    #[test]
    fn item_get_by_id() {
        const CAP: usize = 32;

        let mut items = SortedItemVec::<CAP, u32>::default();
        // Add 2 ids for 1 to 15 and one 0 + one 16
        for i in 1..=CAP {
            items.try_add((i % 17) as u32).unwrap();
        }

        assert_eq!(items.get_all_by_id(17).count(), 0);
        assert_eq!(items.get_all_by_id(0).count(), 1);
        assert_eq!(items.get_all_by_id(16).count(), 1);

        for i in 1..=15 {
            items.contains_id(i);
            assert_eq!(items.get_all_by_id(i).count(), 2);
        }
    }

    #[test]
    fn inventory_one_of_a_kind() {
        const SLOTS: usize = 4;
        let mut inv = Inventory::<8, u32>::new(SLOTS);

        // Odd item id works
        inv.try_add(1).unwrap();
        inv.try_add(1).unwrap();

        // Even is one-of-a-kind
        inv.try_add(2).unwrap();
        assert!(!inv.can_insert_item(&2));
        assert!(inv.try_add(2).is_err());
    }

    #[test]
    fn test_insert() {
        const SLOTS: usize = 4;
        let mut inv = Inventory::<8, u32>::new(SLOTS);

        for i in (1..=4).rev() {
            inv.try_add(i).unwrap();
        }

        assert_eq!(inv.is_full(), true);
        itertools::assert_equal(inv.items().cloned(), [1, 2, 3, 4]);
        assert!(inv.items.test_check_sorted());
    }
}
