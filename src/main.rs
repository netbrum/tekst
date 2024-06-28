mod args;
mod error;
mod viewer;

pub use self::error::{Error, Result};

use args::Args;
use clap::Parser;
use viewer::Viewer;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut viewer = Viewer::new(args)?;

    match viewer.run() {
        Ok(()) => {}
        Err(error) => eprintln!("tekst: {error:#}"),
    }

    Ok(())
}
