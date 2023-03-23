//TODO
#![allow(dead_code)]

use super::{InventoryItem, InventoryError};

#[derive(Debug, Clone)]
pub struct StackSlot<Item> {
    capacity: usize,
    quantity: usize,
    item: Item,
}

impl<Item> StackSlot<Item> {
    pub fn new(capacity: usize, quantity: usize, item: Item) -> Self {
        Self {
            capacity,
            quantity,
            item,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn quantity(&self) -> usize {
        self.quantity
    }

    pub fn item(&self) -> &Item {
        &self.item
    }

    pub fn free_space(&self) -> usize {
        self.capacity - self.quantity
    }
}

impl<Item> StackSlot<Item>
where
    Item: InventoryItem,
{
    pub fn is_same_id(&self, other: &Self) -> bool {
        self.item.id() == other.item.id()
    }

    pub fn left_merge(&mut self, other: &mut Self) -> Result<usize, InventoryError> {
        if !self.is_same_id(other) {
            return Err(InventoryError::InvalidMergeId {
                left: self.id(),
                right: other.id(),
            });
        }

        let move_count = self.free_space().min(other.quantity);
        other.quantity -= move_count;
        self.quantity += self.quantity;
        Ok(move_count)
    }

    pub fn right_merge(&mut self, other: &mut Self) -> Result<usize, InventoryError> {
        other.left_merge(self)
    }
}

impl<Item: InventoryItem> InventoryItem for StackSlot<Item> {
    fn is_one_of_a_kind(&self) -> bool {
        self.item.is_one_of_a_kind()
    }

    fn stack_size(&self) -> usize {
        self.item.stack_size()
    }

    fn id(&self) -> u32 {
        self.item.id()
    }
}