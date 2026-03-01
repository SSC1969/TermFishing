use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rand::RngExt;
use rand_distr::Distribution;
use rand_distr::weighted::WeightedIndex;
use serde::Deserialize;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    SPECIES,
    items::{Item, ItemBase, ItemKind},
};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Fish {
    pub length: u32,
    pub weight: u32,
    pub rarity: Rarity,
}

impl Fish {
    pub fn new(name: &str, value: u32, length: u32, weight: u32, rarity: Rarity) -> Item {
        Item {
            base: ItemBase {
                name: String::from(name),
                value,
            },
            kind: ItemKind::Fish(Fish {
                length,
                weight,
                rarity,
            }),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash, Clone, EnumIter)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn multiplier(&self) -> f32 {
        match self {
            Rarity::Common => 1.0,
            Rarity::Rare => 1.5,
            Rarity::Epic => 2.0,
            Rarity::Legendary => 2.5,
        }
    }

    pub fn odds(&self) -> f32 {
        match self {
            Rarity::Common => 0.5,
            Rarity::Rare => 0.3,
            Rarity::Epic => 0.15,
            Rarity::Legendary => 0.05,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Species {
    name: String,
    base_value: u32,
    min_len: u32,
    max_len: u32,
    min_weight: u32,
    max_weight: u32,
    icon: String,
    colour: Colour,
    rarity: Rarity,
}

pub fn read_species_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Species>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let species: Vec<Species> = serde_json::from_reader(reader)?;

    Ok(species)
}

fn generate_rarity() -> Rarity {
    let mut rng = rand::rng();
    let rarities: Vec<Rarity> = Rarity::iter().collect();
    let weights: Vec<f32> = rarities.iter().map(|r| r.odds()).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let rarity = rarities[dist.sample(&mut rng)].clone();
    return rarity;
}

pub fn generate_fish() -> Item {
    let mut rng = rand::rng();
    let s = &SPECIES[rng.random_range(0..SPECIES.len())];
    let length = rng.random_range(s.min_len..s.max_len);
    let weight = rng.random_range(s.min_weight..s.max_weight);
    let weight_factor = (weight - s.min_weight) as f32 / (s.max_weight - s.min_weight) as f32;
    let value = s.base_value as f32 * s.rarity.multiplier() * (weight_factor + 0.5);
    let rarity = generate_rarity();
    return Fish::new(s.name.as_str(), value as u32, length, weight, rarity);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_species() {
        let species = read_species_from_file("src/items/species.json");
        assert!(species.is_ok());

        let species = species.unwrap();

        println!("{} species read\n", species.len());
        for s in &species {
            println!(
                "{}: icon {} len {}–{}cm, weight {}–{}kg, {:?}, {:?}",
                s.name,
                s.icon,
                s.min_len,
                s.max_len,
                s.min_weight,
                s.max_weight,
                s.rarity,
                s.colour
            );
        }
        println!("");
    }

    #[test]
    fn test_generate_fish() {
        for _ in 0..10 {
            let fish = generate_fish();
            println!("{:?}", fish);
        }
    }
}
