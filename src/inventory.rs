pub mod backpack;
pub mod dex;

use crate::items::Item;

pub trait Inventory {
    fn add_item(&mut self, i: Item);
    fn remove_item(&mut self, i: Item);
}
