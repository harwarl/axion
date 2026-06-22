use super::writer::Writer;
use crate::contants::{DEPENDENCIES, MAIN_FILES, NEW_PROJECT_DIR};
use crate::error::Result;
use crate::prompt::{Auth, Cache, Containerize, Database, NewProject};
use crate::scaffold::template::TemplateRenderer;
use crate::utils::cargo::Cargo;
use crate::utils::docker::Docker;

pub trait ScaffoldStep {
    fn label(&self) -> &str;
    fn enabled(&self, _new_project: &NewProject) -> bool {
        true
    }
    fn run(&self, new_project: &NewProject) -> Result<()>;
}

// Base Step - Creating folders
pub struct BaseStep;
impl ScaffoldStep for BaseStep {
    fn label(&self) -> &str {
        "Creating Cargo Project ..."
    }

    fn run(&self, new_project: &NewProject) -> Result<()> {
        Cargo::init(&new_project.name)?;
        for dir in NEW_PROJECT_DIR {
            Writer::create_dir(&format!("{}/{}", &new_project.directory, dir))?;
        }
        Ok(())
    }
}

pub struct DependenciesStep;
impl ScaffoldStep for DependenciesStep {
    fn label(&self) -> &str {
        "Installing packages. This might take a couple of minutes."
    }

    fn run(&self, new_project: &NewProject) -> Result<()> {
        // Load mosut have dependencies
        let mut deps = Vec::from(DEPENDENCIES);

        // Get more dependencies based on project requirements
        match new_project.database {
            Database::PostgreSQL => {
                deps.push("sqlx --features postgres,macros,runtime-tokio-rustls")
            }
            Database::None => {}
        }

        match new_project.auth {
            Auth::Jwt => {
                deps.push("bcrypt");
                deps.push("jsonwebtoken --features rust_crypto");
            }
            Auth::None => {}
        }

        // add dependencies
        for dep in deps {
            Cargo::add(dep, &new_project.directory)?;
        }

        Ok(())
    }
}

// main.rs
pub struct TemplateStep;
impl ScaffoldStep for TemplateStep {
    fn label(&self) -> &str {
        "Generating Project files"
    }

    // Create the files needed i.e The
    fn run(&self, new_project: &NewProject) -> Result<()> {
        TemplateRenderer::render_all(&new_project)
    }
}

// Docker
pub struct DockerStep;
impl ScaffoldStep for DockerStep {
    fn label(&self) -> &str {
        "Setting up docker layer"
    }
    fn enabled(&self, project: &NewProject) -> bool {
        project.containerize != Containerize::None
    }
    fn run(&self, project: &NewProject) -> Result<()> {
        Docker::initialize(&project.directory)?;
        Ok(())
    }
}
