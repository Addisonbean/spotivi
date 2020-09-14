#[macro_use]
extern crate anyhow;

use anyhow::Result;

mod api;
mod app;
mod config;
mod keybindings;
mod views;

use api::SpotifyApi;
use app::App;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    let client = SpotifyApi::new(&config).await.expect("auth failed...");

    let mut app = App::new(config, client);
    app.start()?;
    app.init_data();
    app.handle_keys();
    app.main_loop().await
}
