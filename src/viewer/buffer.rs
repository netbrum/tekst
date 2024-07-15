use crate::Result;

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub struct Buffer {
    path: PathBuf,
    pub data: String,
}

impl Buffer {
    pub fn new(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        Ok(Self {
            path: path.to_path_buf(),
            data,
        })
    }

    pub fn contents(&self) -> Result<String> {
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
        let buffer = Buffer::new(&PathBuf::from(file!()))?;
        let contents = buffer.contents()?;

        assert!(!contents.is_empty());

        Ok(())
    }
}
