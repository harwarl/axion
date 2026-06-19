use handlebars::Handlebars;
use serde_json::json;

use crate::{
    error::{AxionError, Result},
    prompt::{Auth, Cache, Database, NewProject},
};

pub struct Template;

impl Template {
    pub fn engine() -> Handlebars<'static> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars
    }

    pub fn cargo_toml(new_project: &NewProject) -> Result<String> {
        let hbs = Self::engine();

        // get template
        let template = include_str!("../templates/Cargo.toml.hbs");

        let data = json!({
            "project": {
                "name": &new_project.name
            },
            "database": {
                "postgresql": new_project.database == Database::PostgreSQL,
            },
            "auth": {
                "jwt": new_project.auth == Auth::Jwt
            },
            "cache": {
                "redis": new_project.cache == Cache::Redis
            }
        });

        let content = hbs
            .render_template(template, &data)
            .map_err(|e| AxionError::Template(e.to_string()))?;

        Ok(content)
    }
}
