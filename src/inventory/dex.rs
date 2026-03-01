use std::collections::HashMap;

use crate::{inventory::Inventory, items::Item};
#[derive(Default, Debug)]
pub struct Dex {
    pub items: HashMap<String, DexEntry>,
}
#[derive(Debug, Default)]
pub struct DexEntry {
    count: u32,
    total_value: u32,
}

impl DexEntry {
    fn add(&mut self, i: Item) {
        self.count += 1;
        self.total_value += i.base.value;
    }

    fn remove(&mut self, i: Item) {
        self.count -= 1;
        self.total_value -= i.base.value;
    }
}

impl Dex {
    pub fn search(&self, name: String) -> Option<&DexEntry> {
        return self.items.get(&name);
    }

    pub fn get_all(&self) -> Vec<&DexEntry> {
        self.items.values().collect()
    }
}

impl Inventory for Dex {
    fn add_item(&mut self, i: Item) {
        let entry = self.items.entry(i.base.name.clone()).or_default();
        entry.add(i);
    }

    fn remove_item(&mut self, i: Item) {
        if let Some(entry) = self.items.get_mut(&i.base.name) {
            entry.remove(i);
        }
    }
}
