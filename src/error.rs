use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::command;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("command error: {0}")]
    Command(#[from] command::Error),

    #[error("config error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

impl Error {
    fn status(&self) -> StatusCode {
        match self {
            Self::Command(error) => error.status(),
            Self::Config(_) | Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = self.status();
        if status_code == StatusCode::INTERNAL_SERVER_ERROR {
            log::error!("{self}");
        } else {
            log::info!("{self}");
        }
        let body = self.to_string().into();
        Response::builder().status(status_code).body(body).unwrap()
    }
}
