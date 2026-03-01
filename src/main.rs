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
    let (p2p_tx, p2p_rx) = tokio::sync::mpsc::channel(32);
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new(p2p_tx);
    let event_tx = app.events.sender();
    tokio::spawn(chat::create_and_connect(
        "Adam".to_string(),
        p2p_rx,
        event_tx,
    ));
    let result = app.run(terminal).await;
    ratatui::restore();
    result
}
