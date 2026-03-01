use std::collections::HashMap;

use crate::{inventory::Inventory, items::Item};

pub struct Backpack {
    items: HashMap<String, Vec<Item>>,
}

impl Inventory for Backpack {
    fn search(name: String, value: u32) -> Vec<Item> {
        return Vec::new();
    }

    fn add_item(i: Item) {}

    fn remove_item(i: Item) {}
}
