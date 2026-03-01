use rand::{Rng, RngExt};
use std::array;

use crate::inventory::Inventory;
use crate::inventory::{backpack::Backpack, dex::Dex};
use crate::items::fish::{self, Fish, generate_fish};
use crate::items::{self, Item, ItemBase, ItemKind};

#[derive(Default, Debug)]
pub struct Player {
    backpack: Backpack,
    dex: Dex,
}

impl Player {
    fn catch_fish(&mut self) {
        let fish = generate_fish();
        self.backpack.add_item(fish.clone());
        self.dex.add_item(fish);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_catch_fish() {
        let mut p = Player::default();
        p.catch_fish();
        assert!(p.backpack.search("Halibut".to_string()).len() == 1)
    }
}
