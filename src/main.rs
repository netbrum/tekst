mod args;
mod error;
mod printer;
mod terminal;
mod watcher;

pub use self::error::{Error, Result};

use args::Args;
use clap::Parser;
use watcher::Watcher;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut watcher = Watcher::new(args)?;

    match watcher.run() {
        Ok(()) => {}
        Err(error) => eprintln!("tekst: {error:#}"),
    }

    Ok(())
}
