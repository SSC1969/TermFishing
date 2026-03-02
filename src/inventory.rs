pub mod backpack;
pub mod dex;

use crate::items::ItemTypes;

pub trait Inventory {
    fn add_item(&mut self, i: ItemTypes);

    fn remove_item(&mut self, i: ItemTypes);
}
