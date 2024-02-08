use serde::Serialize;
use std::{
    fs::{self, read_to_string, write},
    io::{self},
    path::{Path, PathBuf},
};

/// A crate file represents a file in a crate
#[derive(Debug, Clone, Serialize, Hash)]
pub struct CrateFile {
    /// The path of the file relative to the crate root
    pub path: PathBuf,
    /// The content of the file
    pub content: String,
}

impl CrateFile {
    /// Create a new CrateFile with the given path and content
    ///
    /// Use `write_to_crate` to write it to the crate
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }
    /// Load a CrateFile from a path
    ///
    /// Assumes that your working directory is the crate root
    pub fn from_file(path: &PathBuf) -> Result<Self, io::Error> {
        Ok(Self {
            path: path.clone(),
            content: read_to_string(path)?,
        })
    }
    /// Write this file to the crate
    ///
    /// Assumes that your working directory is the crate root
    pub fn write(&self) -> Result<(), io::Error> {
        let parent = self.path.parent().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get parent directory",
        ))?;
        fs::create_dir_all(parent)?;
        write(&self.path, &self.content)
    }
    /// Write this file to the crate
    pub fn write_to_crate(&self, crate_root: &Path) -> Result<(), io::Error> {
        let full_path = crate_root.join(&self.path);
        let parent = full_path.parent().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get parent directory",
        ))?;
        fs::create_dir_all(parent)?;
        write(&full_path, &self.content)
    }
}
