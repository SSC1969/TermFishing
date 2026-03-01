use crate::inventory::Inventory;
use crate::inventory::{backpack::Backpack, dex::Dex};
use crate::items::fish::generate_fish;

#[derive(Default, Debug)]
pub struct Player {
    pub backpack: Backpack,
    pub dex: Dex,
}

impl Player {
    pub fn catch_fish(&mut self) {
        let fish = generate_fish();
        self.backpack.add_item(fish.clone());
        self.dex.add_item(fish);
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
