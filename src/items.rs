use crate::items::fish::Fish;

pub mod fish;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct ItemBase {
    pub name: String,
    pub value: u32,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum ItemKind {
    Fish(Fish),
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Item {
    pub base: ItemBase,
    pub kind: ItemKind,
}
