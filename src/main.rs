mod app;
mod chat;
mod inventory;
mod items;
mod player;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // app::errors::init()?;
    // app::logging::init()?;

    Ok(())
}
