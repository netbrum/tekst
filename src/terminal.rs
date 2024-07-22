use crate::Result;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use std::io;

pub struct Terminal;

impl Terminal {
    pub fn reset() -> Result<()> {
        let mut stdout = io::stdout();

        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(MoveTo(0, 0))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resets() {
        assert!(Terminal::reset().is_ok());
    }
}
