use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Deserialize, Debug)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Deserialize, Debug)]
pub struct Species {
    species_name: String,
    min_len: u32,
    max_len: u32,
    min_weight: u32,
    max_weight: u32,
    icon: String, //subject to change
    colour: Colour,
    rarity: Rarity,
}

fn read_species_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Species>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let species: Vec<Species> = serde_json::from_reader(reader)?;

    Ok(species)
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
                s.species_name,
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
}
