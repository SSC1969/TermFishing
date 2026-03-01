use crate::inventory::Inventory;
use crate::inventory::{backpack::Backpack, dex::Dex};
use crate::items::fish::Fish;

#[derive(Default)]
pub struct Player {
    pub backpack: Backpack,
    pub dex: Dex,
}

impl Player {
    pub fn catch_fish(&mut self) {
        let fish = Fish::generate();
        self.backpack.add_item(Box::new(fish.clone()));
        self.dex.add_item(Box::new(fish));
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
