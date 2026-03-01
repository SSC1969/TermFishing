pub mod app;
pub mod chat;
pub mod event;
pub mod inventory;
pub mod items;
pub mod player;
pub mod ui;

use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
