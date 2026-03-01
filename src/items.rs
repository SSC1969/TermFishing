use dyn_clone::DynClone;
use dyn_eq::DynEq;
use ratatui::{
    text::{Line, Span, Text},
    widgets::ListItem,
};

pub mod fish;

pub trait Item: DynEq + DynClone {
    fn name(&self) -> String;
    fn value(&self) -> i32;
    fn info(&self) -> String;
    fn icon(&self) -> Span<'_>;
}
dyn_eq::eq_trait_object!(Item);
dyn_clone::clone_trait_object!(Item);

impl<'a> From<&'a Box<dyn Item>> for ListItem<'a> {
    fn from(item: &'a Box<dyn Item>) -> ListItem<'a> {
        let icon = item.icon();
        let line1 = Line::from(icon);

        let line2 = Line::from(item.info());

        ListItem::new(Text::from(vec![line1, "".into(), line2]))
    }
}
