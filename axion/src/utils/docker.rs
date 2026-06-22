use std::process::{Command, ExitStatus};

use crate::error::{AxionError, Result};

pub struct Docker;

impl Docker {
    pub fn initialize(dir: &str) -> Result<()> {
        let status = Command::new("docker")
            .arg("init")
            .current_dir(dir)
            .status()
            .map_err(|e| AxionError::Io("docker init".to_string(), e))?;

        Self::status_check("docker init failed", status)
    }

    fn status_check(msg: &str, status: ExitStatus) -> Result<()> {
        if !status.success() {
            return Err(AxionError::Template(msg.to_string()));
        }

        Ok(())
    }
}
