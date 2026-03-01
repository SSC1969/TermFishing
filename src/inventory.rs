pub mod backpack;
pub mod dex;

use crate::items::Item;

trait Inventory {
    // Search for items matching the given params
    fn search(name: String, value: u32) -> Vec<Item>;
    fn add_item(i: Item);
    fn remove_item(i: Item);
}
