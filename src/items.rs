use ratatui::{text::Line, widgets::ListItem};

use crate::items::fish::Fish;

pub mod fish;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct ItemBase {
    pub name: String,
    pub value: u32,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum ItemKind {
    Fish(Fish),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Item {
    pub base: ItemBase,
    pub kind: ItemKind,
}

impl From<&Item> for ListItem<'_> {
    fn from(item: &Item) -> Self {
        ListItem::new(Line::from(format!(
            "{} ({:?}) - ${}",
            item.base.name, item.kind, item.base.value
        )))
    }
}
