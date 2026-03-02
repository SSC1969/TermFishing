use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
    widgets::ListItem,
};

use crate::items::fish::Fish;

pub mod fish;

#[derive(Clone, PartialEq, Eq)]
pub enum ItemTypes {
    Fish(Fish),
}

pub trait Item {
    fn name(&self) -> String;
    fn value(&self) -> i32;
    fn info(&self) -> String;
    fn icon(&self) -> Span<'static>;
}

impl<'a> From<&'a ItemTypes> for ListItem<'a> {
    fn from(item: &'a ItemTypes) -> ListItem<'a> {
        let icon = item.icon();
        let line1 = Line::from(vec![icon, " ".into(), item.name().into()])
            .bold()
            .underlined();
        let line2 = Line::from(item.info());

        ListItem::new(Text::from(vec![line1, line2]))
    }
}

impl Item for ItemTypes {
    fn name(&self) -> String {
        match self {
            ItemTypes::Fish(fish) => fish.name(),
        }
    }
    fn icon(&self) -> Span<'_> {
        match self {
            ItemTypes::Fish(fish) => fish.icon(),
        }
    }
    fn info(&self) -> String {
        match self {
            ItemTypes::Fish(fish) => fish.info(),
        }
    }
    fn value(&self) -> i32 {
        match self {
            ItemTypes::Fish(fish) => fish.value(),
        }
    }
}
