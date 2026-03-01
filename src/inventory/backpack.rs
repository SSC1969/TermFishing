use crate::{inventory::Inventory, items::Item};
use std::collections::HashMap;

#[derive(Default)]
pub struct Backpack {
    pub items: HashMap<String, Vec<Box<dyn Item>>>,
}

impl Backpack {
    pub fn search(&self, name: String) -> Vec<Box<dyn Item>> {
        let found = self.items.get(&name);
        return match found {
            Some(vec) => vec.to_vec(),
            None => Vec::new(),
        };
    }

    pub fn get_all(&self) -> Vec<Box<dyn Item>> {
        let vecs = self.items.values().cloned();
        vecs.flatten().collect()
    }
}

impl Inventory for Backpack {
    fn add_item(&mut self, i: Box<dyn Item>) {
        let vec = self.items.entry(i.name()).or_default();
        vec.push(i);
    }

    fn remove_item(&mut self, i: Box<dyn Item>) {
        let vec = self.items.entry(i.name()).or_default();
        vec.remove(vec.iter().position(|x| *x == i).unwrap());
    }
}
