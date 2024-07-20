use crate::Result;

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub struct Buffer {
    path: PathBuf,
    data: String,
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

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut file = File::open(&self.path)?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        self.data = data;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_data() -> Result<()> {
        let buffer = Buffer::new(&PathBuf::from(file!()))?;
        let data = buffer.data();

        assert!(!data.is_empty());

        Ok(())
    }
}
