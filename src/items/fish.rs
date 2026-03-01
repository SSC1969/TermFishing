use crate::items::{Item, ItemBase, ItemKind};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Fish {
    pub length: u32,
    pub weight: u32,
}

impl Fish {
    pub fn new(name: &str, value: u32, length: u32, weight: u32) -> Item {
        Item {
            base: ItemBase {
                name: String::from(name),
                value,
            },
            kind: ItemKind::Fish(Fish { length, weight }),
        }
    }
}
