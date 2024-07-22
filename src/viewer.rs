mod buffer;

use crate::{args::Args, terminal::Terminal, Error, Result};
use colored::Colorize;

use buffer::Buffer;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebouncedEvent};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

pub struct Viewer {
    buffers: HashMap<PathBuf, Buffer>,
    watch_path: PathBuf,
    args: Args,
}

impl Viewer {
    pub fn new(args: Args) -> Result<Self> {
        debug_assert!(args.path.exists());

        Ok(Self {
            buffers: HashMap::default(),
            watch_path: args.path.canonicalize()?,
            args,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let (sender, receiver) = mpsc::channel();

        let duration = Duration::from_millis(self.args.debounce as u64);
        let mut debouncer = new_debouncer(duration, sender)?;

        let dir = self.watch_dir()?;
        debug_assert!(dir.exists());

        debouncer.watcher().watch(dir, RecursiveMode::Recursive)?;

        for mut events in receiver.iter().flatten() {
            if self.watch_path.is_file() {
                events.retain(|event| event.path == self.watch_path);
            }

            if !events.is_empty() {
                self.handle(events)?;
            }
        }

        Ok(())
    }

    fn watch_dir(&self) -> Result<&Path> {
        if self.watch_path.is_dir() {
            Ok(&self.watch_path)
        } else {
            self.watch_path.parent().ok_or(Error::NoParentDirectory)
        }
    }

    fn handle(&mut self, events: Vec<DebouncedEvent>) -> Result<()> {
        Terminal::reset()?;

        for DebouncedEvent { path, .. } in events {
            let buffer = self
                .buffers
                .entry(path.clone())
                .or_insert(Buffer::new(&path)?);

            let old = buffer.data().to_string();

            buffer.refresh()?;

            let data = buffer.data();

            if self.args.diff {
                Self::print_diff(&old, data);
            } else {
                Self::print(data);
            }
        }

        Ok(())
    }

    fn print_diff(old: &str, new: &str) {
        for diff in diff::lines(old, new) {
            let result = match diff {
                diff::Result::Left(l) => format!("-{l}").bright_red(),
                diff::Result::Both(l, _) => l.normal(),
                diff::Result::Right(r) => format!("+{r}").bright_green(),
            };

            println!("{result}");
        }
    }

    fn print(data: &str) {
        println!("{data}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    type Error = Box<dyn std::error::Error>;
    type Result<T> = std::result::Result<T, Error>;

    fn args(path: &str) -> Result<Args> {
        // clap::Parser::try_parse_from doesn't read env::args(), so we need to supply the
        // execution path. The execution path itself doesn't matter in this case, so we set it to
        // an empty string.
        let mock = &["", path];
        let args = Args::try_parse_from(mock)?;

        Ok(args)
    }

    #[test]
    fn constructs() -> Result<()> {
        let viewer = Viewer::new(args(file!())?);

        assert!(viewer.is_ok());

        Ok(())
    }
}
