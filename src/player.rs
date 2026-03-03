use ratatui::text::Span;

use crate::inventory::Inventory;
use crate::inventory::{backpack::Backpack, dex::Dex};
use crate::items::fish::Fish;
use crate::items::{Item, ItemTypes};

#[derive(Default)]
pub struct Player {
    pub name: String,
    pub backpack: Backpack,
    pub dex: Dex,

    pub time_of_last_catch: u32,
    pub ticks_until_next_bite: u32,
}

impl Player {
    pub fn catch_fish(&mut self) -> Span {
        let fish = Fish::generate();
        self.backpack.add_item(ItemTypes::Fish(fish.clone()));
        self.dex.add_item(ItemTypes::Fish(fish.clone()));

        fish.icon()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catch_fish() {
        let mut p = Player::default();
        p.catch_fish();
        assert!(p.backpack.get_all().len() == 1)
    }
}
