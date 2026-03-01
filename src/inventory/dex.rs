use std::collections::HashMap;

use crate::{inventory::Inventory, items::Item};
#[derive(Default, Debug)]
pub struct Dex {
    pub items: HashMap<String, Vec<Item>>,
}

impl Inventory for Dex {
    fn search(&self, name: String) -> Vec<&Item> {
        return Vec::new();
    }

    fn add_item(&mut self, i: Item) {}

    fn remove_item(&mut self, i: Item) {}
}
