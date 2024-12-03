use std::path::{Path, PathBuf};

use config::Environment;
use serde::Deserialize;

use crate::app;

#[derive(Deserialize)]
pub struct Command {
    pub directory: PathBuf,
    pub file_extension: Option<String>,
}

#[derive(Deserialize)]
pub struct Http {
    pub address: String,
    pub cors: bool,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub command: Command,
    pub http: Http,
}

impl Config {
    pub fn load(file: &Path) -> crate::Result<Config> {
        let config = config::Config::builder()
            .add_source(config::File::from(file).format(config::FileFormat::Toml))
            .add_source(Environment::with_prefix(&app::NAME.to_uppercase()))
            .build()?;
        Ok(config.try_deserialize()?)
    }
}
