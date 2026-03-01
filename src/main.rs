mod app;
mod chat;
mod items;
mod inventory;
mod player;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    app::errors::init()?;
    app::logging::init()?;

    Ok(())
}
