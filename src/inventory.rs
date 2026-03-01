pub mod backpack;
pub mod dex;

use crate::items::Item;

pub trait Inventory {
    // Search for items matching the given params
    fn search(&self, name: String) -> Vec<&Item>;
    fn add_item(&mut self, i: Item);
    fn remove_item(&mut self, i: Item);
}
