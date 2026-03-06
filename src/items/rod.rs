use std::sync::LazyLock;

use ratatui::{
    style::{Color, Style},
    text::Span,
};
use serde::Deserialize;

use crate::items::Item;

static ROD_ICON: &str = "(";
static ROD_STRING: &str = "`\\";

pub static RODS: LazyLock<Vec<Rod>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("rod.json")).expect("Error deserializing rods!")
});

#[derive(Clone, PartialEq, Deserialize)]
pub struct Rod {
    pub name: String,
    pub value: i32,
    pub color: Color,
    /// Multiplier used for calculating lure time - the higher this is the faster fish will bite
    pub lure_mult: f32,
    /// Multiplier used for calculating bite time - the higher this is the longer fish will stay on
    /// the hook
    pub hook_strength: f32,
}

impl Item for Rod {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn info(&self) -> String {
        "Lure: {self.lure_mult} | Hook: {self.hook_strength}".to_string()
    }

    fn icon(&self) -> Vec<Span<'_>> {
        vec![
            Span::styled(ROD_ICON, Style::new().fg(self.color)),
            ROD_STRING.into(),
        ]
    }
}
