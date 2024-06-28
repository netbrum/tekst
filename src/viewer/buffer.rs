use crate::Result;

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub struct Buffer {
    path: PathBuf,
}

impl Buffer {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn contents(&mut self) -> Result<String> {
        let mut file = File::open(&self.path)?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_contents() -> Result<()> {
        let mut buffer = Buffer::new(&PathBuf::from(file!()));
        let contents = buffer.contents()?;

        assert!(!contents.is_empty());

        Ok(())
    }
}
