use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::ListItem,
};
use std::collections::HashMap;

use crate::{
    inventory::Inventory,
    items::{
        Item, ItemTypes,
        fish::{SPECIES, Species},
    },
};

pub struct Dex {
    items: HashMap<String, DexEntries>,
}

impl Dex {
    pub fn get(&self, name: String) -> Option<&DexEntries> {
        self.items.get(&name)
    }

    pub fn get_mut(&mut self, name: String) -> Option<&mut DexEntries> {
        self.items.get_mut(&name)
    }

    pub fn get_all(&self) -> Vec<&DexEntries> {
        self.items.values().collect()
    }
}

impl Default for Dex {
    fn default() -> Self {
        let mut items: HashMap<String, DexEntries> = HashMap::new();
        for spec in SPECIES.iter() {
            items.insert(spec.name.clone(), DexEntries::Fish(FishEntry::new(spec)));
        }
        Self { items }
    }
}

impl Inventory for Dex {
    fn add_item(&mut self, item: ItemTypes) {
        if let Some(entry) = self.get_mut(item.name()) {
            entry.update(item);
        }
    }
    fn remove_item(&mut self, item: ItemTypes) {
        self.items.remove(&item.name());
    }
}

pub trait DexEntry {
    /// Updates this entry based on the newly passed in item
    fn update(&mut self, item: ItemTypes);
    /// Gets the display text for this entry
    fn get_lines(&'_ self) -> [Line<'_>; 3];
}

pub enum DexEntries {
    Fish(FishEntry),
}

impl DexEntry for DexEntries {
    fn update(&mut self, item: ItemTypes) {
        match self {
            DexEntries::Fish(entry) => entry.update(item),
        }
    }

    fn get_lines(&'_ self) -> [Line<'_>; 3] {
        match self {
            DexEntries::Fish(entry) => entry.get_lines(),
        }
    }
}
pub struct FishEntry {
    species: &'static Species,
    count: u32,
    total_value: i32,
    highest_value: i32,
    largest: u32,
    heaviest: u32,
}

impl FishEntry {
    fn new(species: &'static Species) -> Self {
        Self {
            species,
            count: 0,
            total_value: 0,
            highest_value: 0,
            largest: 0,
            heaviest: 0,
        }
    }
}

impl DexEntry for FishEntry {
    fn update(&mut self, item: ItemTypes) {
        if let ItemTypes::Fish(fish) = item {
            self.count += 1;
            self.total_value += fish.value();
            self.highest_value = i32::max(self.highest_value, fish.value());
            self.largest = u32::max(self.largest, fish.length);
            self.heaviest = u32::max(self.heaviest, fish.weight);
        }
    }

    fn get_lines(&'_ self) -> [Line<'_>; 3] {
        let l1 = Line::from(vec![
            self.species.icon(),
            " ".into(),
            self.species.name.clone().into(),
        ])
        .bold()
        .underlined();

        let l2 = Line::from(vec![
            format!("Caught: {}(${})", self.count, self.total_value,).into(),
        ]);

        let l3 = Line::from(vec![
            format!(
                "Best: {}cm | {}g | ${}",
                self.largest, self.heaviest, self.highest_value
            )
            .into(),
        ]);

        [l1, l2, l3]
    }
}

impl<'a> From<&'a DexEntries> for ListItem<'a> {
    fn from(entry: &'a DexEntries) -> Self {
        let [l1, l2, l3] = entry.get_lines();
        ListItem::new(Text::from(vec![l1, l2, l3]))
    }
}
