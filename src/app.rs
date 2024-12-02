use std::sync::Arc;

use tokio::net::TcpListener;

use crate::{command, config::Config, router};

pub const NAME: &str = env!("CARGO_PKG_NAME");

pub struct State {
    pub commands: command::Manager,
}

impl State {
    pub fn build(config: &Config) -> Self {
        let commands = command::Manager::new(
            &config.command.directory,
            config.command.file_extension.as_deref(),
        );
        Self { commands }
    }
}

async fn serve(
    address: &str,
    port: u16,
    cors_is_permitted: bool,
    state: State,
) -> crate::Result<()> {
    let router = router::build(Arc::new(state), cors_is_permitted);
    let listener = TcpListener::bind((address, port)).await?;
    log::info!(
        "listening on: {}",
        format!("http://{}", listener.local_addr().unwrap())
    );
    Ok(axum::serve(listener, router).await?)
}

pub async fn run(config: &Config) -> crate::Result<()> {
    let state = State::build(config);
    serve(
        &config.http.address,
        config.http.port,
        config.http.cors,
        state,
    )
    .await
}
