use crate::{Result, config, story};
use clap::Parser;
use std::sync::Arc;

#[derive(Debug, Parser)]
#[command(about, author, long_about = None, version)]
pub struct Args {
    /// Ollama model
    #[arg(long, short)]
    pub model: Option<String>,

    /// Output directory
    #[arg(long, short)]
    pub output_dir: String,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    story::generate(config).await?;

    Ok(())
}
