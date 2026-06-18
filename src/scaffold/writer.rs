use std::{fs, path::Path};

use crate::error::{AxionError, Result};

pub struct Writer;

impl Writer {
    // Create the required directory
    pub fn create_dir(path: &str) -> Result<()> {
        fs::create_dir_all(path).map_err(|e| AxionError::Io(path.to_string(), e))
    }

    // Creates the file, if there isn't, and writes to it
    pub fn create_file(path: &str, content: &str) -> Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).map_err(|e| AxionError::Io(path.to_string(), e))?
        }
        fs::write(path, content).map_err(|e| AxionError::Io(path.to_string(), e))
    }
}