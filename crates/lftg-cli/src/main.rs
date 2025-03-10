#![forbid(unsafe_code)]

use std::error::Error;

mod app;
mod chapter;
mod config;
mod error;
mod extract;
mod ollama;
mod story;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let result = app::run();

    if let Err(e) = result.await {
        eprint!("Error: {e:?}");
    }

    Ok(())
}
