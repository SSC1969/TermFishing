pub mod app;
pub mod chat;
pub mod event;
pub mod inventory;
pub mod items;
pub mod player;
pub mod ui;

use crate::{
    app::App,
    items::fish::{Species, read_species_from_file},
};
use std::sync::LazyLock;

static SPECIES: LazyLock<Vec<Species>> = LazyLock::new(|| {
    read_species_from_file("src/items/species.json").expect("failed to load species.json")
});

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
