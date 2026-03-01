use std::collections::HashMap;

use crate::{inventory::Inventory, items::Item};

pub struct Dex {
    items: HashMap<String, Vec<(Item, u16)>>,
}

impl Inventory for Dex {
    fn search(name: String, value: u32) -> Vec<Item> {
        return Vec::new();
    }

    fn add_item(i: Item) {}

    fn remove_item(i: Item) {}
}
