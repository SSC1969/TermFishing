use rand::{Rng, RngExt};
use std::array;

use crate::inventory::Inventory;
use crate::inventory::{backpack::Backpack, dex::Dex};
use crate::items::fish::{self, Fish};
use crate::items::{self, Item, ItemBase, ItemKind};

#[derive(Default, Debug)]
pub struct Player {
    backpack: Backpack,
    dex: Dex,
}

const species: [&str; 5] = ["Halibut", "Guppy", "Goldfish", "Carp", "Salmon"];

fn generate_fish() -> Item {
    let mut rng = rand::rng();
    return Fish::new(species[rng.random_range(0..1)], 10, 10, 10);
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
    fn test_generate_fish() {
        let fish = generate_fish();
        assert!(species.contains(&fish.base.name.as_str()))
    }

    #[test]
    fn test_catch_fish() {
        let mut p = Player::default();
        p.catch_fish();
        assert!(p.backpack.search("Halibut".to_string()).len() == 1)
    }
}
