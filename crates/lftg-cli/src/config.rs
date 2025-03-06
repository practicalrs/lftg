use crate::{Result, app::Args, error::Error};

#[derive(Clone, Debug)]
pub struct Config {
    pub model: Option<String>,
    pub ollama_host: String,
    pub output_dir: String,
}

impl Config {
    pub fn new(model: Option<String>, ollama_host: String, output_dir: String) -> Self {
        Self {
            model,
            ollama_host,
            output_dir,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let model = args.model;
    let Ok(ollama_host) = std::env::var("OLLAMA_HOST") else {
        return Err(Box::new(Error::OllamaHostAddresMissing));
    };
    let output_dir = args.output_dir;

    let config = Config::new(model, ollama_host, output_dir);

    Ok(config)
}
