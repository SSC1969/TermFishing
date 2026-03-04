use crate::{
    inventory::Inventory,
    items::{Item, ItemTypes},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Backpack {
    pub items: HashMap<String, Vec<ItemTypes>>,
}

impl Backpack {
    pub fn search(&self, name: String) -> Vec<ItemTypes> {
        let found = self.items.get(&name);
        return match found {
            Some(vec) => vec.to_vec(),
            None => Vec::new(),
        };
    }

    pub fn get_all(&self) -> Vec<ItemTypes> {
        let vecs = self.items.values().cloned();
        vecs.flatten().collect()
    }
}

impl Inventory for Backpack {
    fn add_item(&mut self, item: ItemTypes) {
        let vec = self.items.entry(item.name()).or_default();
        vec.push(item);
    }

    fn remove_item(&mut self, item: ItemTypes) {
        let vec = self.items.entry(item.name()).or_default();
        vec.remove(vec.iter().position(|x| *x == item).unwrap());
    }
}
