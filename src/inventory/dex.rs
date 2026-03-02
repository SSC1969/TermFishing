use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::ListItem,
};
use std::collections::HashMap;

use crate::{
    SPECIES,
    inventory::Inventory,
    items::{
        Item,
        fish::{Fish, Species},
    },
};

pub struct Dex {
    pub items: HashMap<String, DexEntry>,
}

impl Default for Dex {
    fn default() -> Self {
        let mut items: HashMap<String, DexEntry<Fish>> = HashMap::new();

        for spec in SPECIES.iter() {
            items.insert(
                spec.name.clone(),
                DexEntry {
                    stats: Box::new(FishStats {
                        species: spec.clone(),
                        ..Default::default()
                    }),
T
                    ..Default::default()
                },
            );
        }

        Self { items }
    }
}

pub struct DexEntry<T> {
    pub name: String,
    pub count: u32,
    pub total_value: i32,
    pub highest_value: i32,
    pub stats: Box<dyn ItemStat<It = T>>,
}

pub trait ItemStat {
    type It;

    fn update(&mut self, item: Self::It);
    fn get_lines(&self) -> Vec<Line>;
}

#[derive(Debug)]
struct FishStats {
    pub species: Species,
    pub largest: u32,
    pub heaviest: u32,
}

impl Default for FishStats {
    fn default() -> Self {
        FishStats {
            species: SPECIES[0].clone(),
            largest: 0,
            heaviest: 0,
        }
    }
}

impl ItemStat for FishStats {
    type It = Fish;

    fn update(&mut self, item: Self::It) {
        self.largest = u32::max(self.largest, item.length);
        self.heaviest = u32::max(self.heaviest, item.weight);
    }

    fn get_lines(&self) -> Vec<Line> {
        let l1 = Line::from(vec![
            self.species.icon(),
            " ".into(),
            self.species.name.clone().into(),
        ])
        .bold()
        .underlined();

        let l2 = Line::from(vec![
            format!("Biggest: {}cm | Heaviest: {}g", self.largest, self.heaviest).into(),
        ]);

        vec![l1, l2]
    }
}

impl<T: Item> DexEntry<T> {
    fn add(&mut self, i: T) {
        self.count += 1;
        self.total_value += i.value();
        self.highest_value = i32::max(self.highest_value, i.value());
        self.stats.update(i);
    }

    fn remove(&mut self, i: T) {
        self.count -= 1;
        self.total_value -= i.value();
    }
}

impl<T> Default for DexEntry<T> {
    fn default() -> Self {
        DexEntry {
            name: "???".to_string(),
            count: 0,
            total_value: 0,
            highest_value: 0,
            stats: Box::new(FishStats::default()),
        }
    }
}

impl<'a> From<&'a DexEntry> for ListItem<'a> {
    fn from(entry: &'a DexEntry) -> Self {
        let mut lines = entry.stats.get_lines();

        let l2 = Line::from(vec![
            format!(
                "Caught: {} | Most Expensive: ${} | Total Value: ${}",
                entry.count, entry.highest_value, entry.total_value
            )
            .into(),
        ]);

        lines.push(l2);

        ListItem::new(Text::from(lines))
    }
}

impl Dex {
    pub fn search(&self, name: String) -> Option<&DexEntry> {
        return self.items.get(&name);
    }

    pub fn get_all(&self) -> Vec<&DexEntry> {
        self.items.values().collect()
    }
}

impl Inventory for Dex {
    fn add_item(&mut self, i: Box<dyn Item>) {
        let entry = self.items.entry(i.name()).or_default();
        entry.add(i);
    }

    fn remove_item(&mut self, i: Box<dyn Item>) {
        if let Some(entry) = self.items.get_mut(&i.name()) {
            entry.remove(i);
        }
    }
}
