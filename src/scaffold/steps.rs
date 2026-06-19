use super::template::Template;
use super::writer::Writer;
use crate::contants::{DEPENDENCIES, MAIN_FILES, NEW_PROJECT_DIR};
use crate::error::Result;
use crate::prompt::{Auth, Cache, Containerize, Database, NewProject};
use crate::utils::cargo::Cargo;

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
        for dir in NEW_PROJECT_DIR {
            Writer::create_dir(&format!("{}/{}", &new_project.directory, dir))?;
        }
        Ok(())
    }
}

pub struct DependenciesStep;
impl ScaffoldStep for DependenciesStep {
    fn label(&self) -> &str {
        "Installing dependencies"
    }

    fn run(&self, new_project: &NewProject) -> Result<()> {
        let mut deps = Vec::from(DEPENDENCIES);

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
pub struct MainStep;
impl ScaffoldStep for MainStep {
    fn label(&self) -> &str {
        "Generating needed folders and files"
    }

    // Create the files needed i.e create_app.rs, state.rs, lib.rs
    fn run(&self, new_project: &NewProject) -> Result<()> {
        for file in MAIN_FILES {
            Writer::write_file(&format!("{}/{}", &new_project.directory, file), "")?;
        }

        Ok(())
    }
}

// Database Migration and connection pool
pub struct DatabaseStep;
impl ScaffoldStep for DatabaseStep {
    fn label(&self) -> &str {
        "Setting up database layer"
    }
    fn enabled(&self, new_project: &NewProject) -> bool {
        new_project.database != Database::None
    }
    fn run(&self, new_project: &NewProject) -> Result<()> {
        Writer::create_dir(&format!("{}/migrations", new_project.directory))?;
        // let content = Template::db_rs(new_project);
        Writer::write_file(
            &format!(
                "{}/src/infrastructure/databases/db.rs",
                new_project.directory
            ),
            "",
        )?;
        Ok(())
    }
}

// ORM Models
// pub struct OrmStep;
// impl ScaffoldStep for OrmStep {
//     fn label(&self) -> &str {
//         "Setting up ORM layer"
//     }

//     fn enabled(&self, project: &NewProject) -> bool {
//         project.orm != ORM::None
//     }

//     fn run(&self, new_project: &NewProject) -> Result<()> {
//         Writer::write_file(&format!("{}/src/models/mod.rs", new_project.directory), "")?;
//         Ok(())
//     }
// }

// AuthMiddleware and guards
pub struct AuthStep;
impl ScaffoldStep for AuthStep {
    fn label(&self) -> &str {
        "Setting up auth layer"
    }

    fn enabled(&self, project: &NewProject) -> bool {
        project.auth != Auth::None
    }

    fn run(&self, project: &NewProject) -> Result<()> {
        Writer::write_file(
            &format!("{}/src/api/middleware/auth.rs", project.directory),
            "",
        )?;
        Ok(())
    }
}

// Cache
pub struct CacheStep;
impl ScaffoldStep for CacheStep {
    fn label(&self) -> &str {
        "Setting up cache layer"
    }
    fn enabled(&self, project: &NewProject) -> bool {
        project.cache != Cache::None
    }
    fn run(&self, project: &NewProject) -> Result<()> {
        Writer::write_file(&format!("{}/src/cache.rs", project.directory), "")?;
        Ok(())
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
        Writer::write_file(&format!("{}/Dockerfile", project.directory), "")?;
        Writer::write_file(&format!("{}/compose.yaml", project.directory), "")?;
        Ok(())
    }
}
