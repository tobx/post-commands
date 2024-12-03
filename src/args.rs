use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Config directory
    #[arg(long, value_name = "FILE")]
    pub config_file: PathBuf,
}
