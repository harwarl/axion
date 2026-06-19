use crate::{
    error::{AxionError, Result},
    find::Finder,
};
use std::path::Path;
use std::process::{Command, ExitStatus};

pub struct Cargo;

impl Cargo {
    pub fn add(dep: &str, dir: &str) -> Result<()> {
        let args: Vec<&str> = dep.split_whitespace().collect();

        let status = Command::new("cargo")
            .args(["add"])
            .args(args)
            .arg("--quiet")
            .current_dir(dir)
            .status()
            .map_err(|e| AxionError::Io("Cargo add".to_string(), e))?;

        let msg: String = format!("cargo add failed for: {}", dep);
        Self::status_check(&msg, status)
    }

    // pub fn new(name: &str) -> Result<()> {
    //     let status = Command::new("cargo")
    //         .arg(format!("init {}", name))
    //         .status()
    //         .map_err(|e| AxionError::Io("Cargo new".to_string(), e))?;

    //     Self::status_check("cargo init failed", status)
    // }

    pub fn init(name: &str) -> Result<()> {
        // name can be an actual name or '.'
        let dir = if name == "." { "." } else { name };

        // Check project already exists
        if Path::new(dir).join("Cargo.toml").exists() {
            return Err(AxionError::ProjectExists(dir.to_string()));
        }

        let status = Command::new("cargo")
            .arg("init")
            .arg(name)
            .arg("--quiet")
            .status()
            .map_err(|e| AxionError::Io("Cargo init".to_string(), e))?;

        Self::status_check("project initialization failed", status)
    }

    fn status_check(msg: &str, status: ExitStatus) -> Result<()> {
        if !status.success() {
            return Err(AxionError::Template(msg.to_string()));
        }

        Ok(())
    }
}
