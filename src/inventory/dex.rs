use std::collections::HashMap;

use crate::{inventory::Inventory, items::Item};

#[derive(Default, Debug)]
pub struct Dex {
    pub items: HashMap<String, DexEntry>,
}
#[derive(Debug, Default)]
pub struct DexEntry {
    count: u32,
    total_value: i32,
}

impl DexEntry {
    fn add(&mut self, i: Box<dyn Item>) {
        self.count += 1;
        self.total_value += i.value();
    }

    fn remove(&mut self, i: Box<dyn Item>) {
        self.count -= 1;
        self.total_value -= i.value();
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
    fn add_item(&mut self, i: Box<dyn Item>) {
        let entry = self.items.entry(i.name()).or_default();
        entry.add(i);
    }

    fn remove_item(&mut self, i: Box<dyn Item>) {
        if let Some(entry) = self.items.get_mut(&i.name()) {
            entry.remove(i);
        }
    }
}
