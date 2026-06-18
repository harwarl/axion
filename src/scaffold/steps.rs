use crate::prompt::{Auth, Cache, Containerize, Database, NewProject, ORM};
use crate::error::Result;
use super::writer::Writer;

pub trait ScaffoldStep {
    fn label(&self) -> &str;
    fn enabled(&self, new_project: &NewProject) -> bool { true }
    fn run(&self, new_project: &NewProject) -> Result<()>;
}

// Base Step - Creating folders
pub struct BaseStep;
impl ScaffoldStep for BaseStep {
    fn label(&self) -> &str {
        "Creating Project Structure"
    }

    fn run(&self, new_project: &NewProject) -> Result<()> {
        Writer::create_dir(&new_project.name)?;
        Writer::create_dir(&format!("{}/src", &new_project.name))?;
        // TODO Create other folders needed
        Ok(())
    }
}

// Cargo.toml
pub struct CargoStep;
impl ScaffoldStep for CargoStep {
    fn label(&self) -> &str {
        "Generating Cargo.toml"
    }

    fn run(&self, new_project: &NewProject) -> Result<()> {
        // TODO: Add the cargo.toml template
        Writer::write_file(&format!("{}/Cargo.toml", &new_project.name), "")?;
        Ok(())
    }
}

// main.rs
pub struct MainStep;
impl ScaffoldStep for MainStep {
    fn label(&self) -> &str {
        "Generating main.rs"
    }

    fn run(&self, new_project: &NewProject) -> Result<()> {
        // TODO: Add the main.fs template 
        Writer::write_file(&format!("{}/main.rs", &new_project.name), "")?;
        Ok(())
    }
}

// config.rs + .env
pub struct ConfigStep;
impl ScaffoldStep for ConfigStep {
    fn label(&self) -> &str {
        "Generating config and .env"
    }

    fn run(&self, new_project: &NewProject) -> Result<()> {
        // TODO: Add the config.rs and .env template
        Writer::write_file(&format!("{}/config.rs", &new_project.name), "")?;
        Writer::write_file(&format!("{}/.env", &new_project.name), "")?;
        Writer::write_file(&format!("{}/.env.example", &new_project.name), "")?;
        Ok(())
    }
}

// Database Migration and connection pool
pub struct DatabaseStep;
impl ScaffoldStep for DatabaseStep {
    fn label(&self) -> &str { "Setting up database layer" }
    fn enabled(&self, new_project: &NewProject) -> bool {
        new_project.database != Database::None
    }
    fn run(&self, new_project: &NewProject) -> Result<()> {
        Writer::create_dir(&format!("{}/migrations", new_project.directory))?;
        // let content = Template::db_rs(new_project);
        Writer::write_file(&format!("{}/src/db.rs", new_project.directory), "")?;
        Ok(())
    }
}

// ORM Models 
pub struct OrmStep;
impl ScaffoldStep for OrmStep {
    fn label(&self) -> &str { "Setting up ORM layer" }
    fn enabled(&self, project: &NewProject) -> bool {
        project.orm != ORM::None
    }
    fn run(&self, new_project: &NewProject) -> Result<()> {
        Writer::write_file(&format!("{}/src/models/mod.rs", new_project.directory), "")?;
        Ok(())
    }
}


// AuthMiddleware and guards
pub struct AuthStep;
impl ScaffoldStep for AuthStep {
    fn label(&self) -> &str { "Setting up auth layer" }
    fn enabled(&self, project: &NewProject) -> bool {
        project.auth != Auth::None
    }
    fn run(&self, project: &NewProject) -> Result<()> {
        Writer::write_file(&format!("{}/src/middleware/auth.rs", project.directory), "")?;
        Ok(())
    }
}

// Cache
pub struct CacheStep;
impl ScaffoldStep for CacheStep {
    fn label(&self) -> &str { "Setting up cache layer" }
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
    fn label(&self) -> &str { "Setting up docker layer" }
    fn enabled(&self, project: &NewProject) -> bool {
        project.containerize != Containerize::None
    }
    fn run(&self, project: &NewProject) -> Result<()> {
        Writer::write_file(&format!("{}/Dockerfile", project.directory), "")?;
        Writer::write_file(&format!("{}/compose.yam", project.directory), "")?;
        Ok(())
    }
}