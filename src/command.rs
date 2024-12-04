use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use axum::http::StatusCode;
use serde::Deserialize;

type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("command not found")]
    NotFound,

    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

impl Error {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Deserialize)]
pub struct Spec {
    name: String,
    args: Option<Vec<String>>,
}

pub struct Manager {
    directory: PathBuf,
    file_extension: Option<String>,
}

impl Manager {
    pub fn new(directory: &Path, file_extension: Option<&str>) -> Self {
        let file_extension = file_extension.map(|extension| {
            if extension.starts_with('.') {
                extension.into()
            } else {
                format!(".{extension}")
            }
        });
        Self {
            directory: directory.into(),
            file_extension,
        }
    }

    pub fn execute(&self, spec: Spec) -> Result<()> {
        let path = self
            .find_matching_path(&spec.name)?
            .ok_or(Error::NotFound)?;
        let mut command = Command::new(path);
        if let Some(args) = spec.args {
            command.args(args);
        }
        Command::spawn(&mut command)?;
        Ok(())
    }

    fn find_matching_path(&self, name: &str) -> Result<Option<PathBuf>> {
        for entry in fs::read_dir(&self.directory)? {
            let path = entry?.path();
            if self.name_matches_path(name, &path) {
                return Ok(Some(path));
            }
        }
        Ok(None)
    }

    fn name_matches_path(&self, name: &str, path: &Path) -> bool {
        let Some(file_name) = path.file_name().and_then(OsStr::to_str) else {
            return false;
        };
        if let Some(extension) = &self.file_extension {
            let stem_length = file_name.len() - extension.len();
            file_name.ends_with(extension) && file_name[..stem_length] == *name
        } else {
            file_name == name
        }
    }
}
