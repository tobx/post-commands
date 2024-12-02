#![forbid(unsafe_code)]

mod app;
mod args;
mod command;
mod config;
mod error;
mod router;

use clap::Parser;

use crate::{args::Args, config::Config, error::Result};

fn init_logger() {
    use env_logger::{Builder, Env};
    use log::LevelFilter;

    let env = Env::new()
        .filter_or("LOG_LEVEL", "error")
        .write_style_or("LOG_STYLE", "auto");
    Builder::from_env(env)
        .filter_module("tower_http::trace::on_request", LevelFilter::Error)
        .init();
}

async fn run() -> crate::Result<()> {
    let args = Args::parse();
    let config = Config::load(args.config_dir.as_deref())?;
    app::run(&config).await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    init_logger();
    if let Err(error) = run().await {
        log::error!("{error}");
        std::process::exit(1);
    }
}
