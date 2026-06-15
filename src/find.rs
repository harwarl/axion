use crate::error::{Error, Result};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct Finder {
    pub root: PathBuf,
}

impl Finder {
    pub fn new() -> Result<Self> {
        let root = Self::find_root()?;
        Ok(Self { root })
    }

    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn find_root() -> Result<PathBuf> {
        let current = std::env::current_dir().unwrap();
        let filename = Path::new("Cargo.toml");
        let cargo_file = Self::find(&current, filename)?;

        // Determine the file root
        cargo_file.parent().map(|p| p.to_path_buf()).ok_or_else(|| {
            Error::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine the project roots",
            ))
        })
    }

    pub fn find(directory: &Path, filename: &Path) -> Result<PathBuf> {
        let candidate = directory.join(filename);

        match fs::metadata(&candidate) {
            Ok(metadata) => {
                if metadata.is_file() {
                    return Ok(candidate);
                }
            }
            Err(error) => {
                if error.kind() == io::ErrorKind::NotFound {
                    return Err(Error::Io(error));
                }
            }
        }

        if let Some(parent) = directory.parent() {
            Self::find(parent, filename)
        } else {
            // throw an Error
            return Err(Error::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "file not found",
            )));
        }
    }
}
