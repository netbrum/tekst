use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    pub path: PathBuf,

    #[arg(long, default_value_t = 500)]
    pub debounce: usize,

    #[arg(short, long, default_value_t = false)]
    pub diff: bool,
}
