mod buffer;

use crate::{args::Args, printer::Printer, terminal::Terminal, Error, Result};
use buffer::Buffer;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebouncedEvent};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

pub struct Watcher {
    buffers: HashMap<PathBuf, Buffer>,
    path: PathBuf,
    args: Args,
}

impl Watcher {
    pub fn new(args: Args) -> Result<Self> {
        debug_assert!(args.path.exists());

        Ok(Self {
            buffers: HashMap::default(),
            path: args.path.canonicalize()?,
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
            if self.path.is_file() {
                events.retain(|event| event.path == self.path);
            }

            if !events.is_empty() {
                self.handle(events)?;
            }
        }

        Ok(())
    }

    fn watch_dir(&self) -> Result<&Path> {
        if self.path.is_dir() {
            Ok(&self.path)
        } else {
            self.path.parent().ok_or(Error::NoParentDirectory)
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
                Printer::diff(&old, data);
            } else {
                Printer::print(data);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use notify_debouncer_mini::DebouncedEventKind;

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
        let watcher = Watcher::new(args(file!())?);

        assert!(watcher.is_ok());

        Ok(())
    }

    #[test]
    fn handles_event() -> Result<()> {
        let mut watcher = Watcher::new(args(file!())?)?;

        let event = DebouncedEvent {
            path: PathBuf::from(file!()),
            kind: DebouncedEventKind::Any,
        };

        assert!(watcher.handle(vec![event]).is_ok());

        Ok(())
    }
}
