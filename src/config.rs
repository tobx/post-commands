use std::path::{Path, PathBuf};

use config::Environment;
use serde::Deserialize;

use crate::app;

const DEFAULT_BASE_DIR: &str = "/usr/local/etc";

const FILE_NAME: &str = "config.toml";

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
    pub fn load(dir: Option<&Path>) -> crate::Result<Config> {
        let path = dir
            .map_or_else(Self::default_path, Into::into)
            .join(FILE_NAME);
        let config = config::Config::builder()
            .add_source(config::File::from(path.as_path()).format(config::FileFormat::Toml))
            .add_source(Environment::with_prefix(&app::NAME.to_uppercase()))
            .build()?;
        Ok(config.try_deserialize()?)
    }

    fn default_path() -> PathBuf {
        Path::new(DEFAULT_BASE_DIR).join(app::NAME)
    }
}
