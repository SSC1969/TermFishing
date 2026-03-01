use std::collections::{HashMap, HashSet};

use crate::{inventory::Inventory, items::Item};

pub struct Backpack {
    items: HashMap<String, HashSet<Item>>,
}

impl Inventory for Backpack {
    fn search(&self, name: String) -> Vec<&Item> {
        let found = self.items.get(&name);
        return match found {
            Some(i) => i.iter().collect(),
            None => Vec::new(),
        };
    }

    fn add_item(&mut self, i: Item) {
        self.items.entry(i.base.name.clone()).or_default().insert(i);
    }

    fn remove_item(&mut self, i: Item) {
        let entry = self.items.entry(i.base.name.clone()).or_default();
        entry.remove(&i);
    }
}
