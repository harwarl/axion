use crate::{
    error::{AxionError, Result},
    prompt::{Auth, Cache, Containerize, Database, NewProject},
    scaffold::writer::Writer,
};
use handlebars::Handlebars;
use serde_json::{Value, json};
use axo_template::TemplateAssets;


// Render the embedded assets
pub struct TemplateRenderer;

impl TemplateRenderer {
    pub fn engine() -> Handlebars<'static> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars
    }

    pub fn render_all(project: &NewProject) -> Result<()> {
        for file_path in TemplateAssets::iter() {
            // Get the file
            let file = TemplateAssets::get(&file_path).ok_or_else(|| {
                AxionError::Template(format!("Missing embedded file: {}", file_path))
            })?;

            // get the file content
            let content = std::str::from_utf8(&file.data)
                .map_err(|_| AxionError::Template(format!("Non-UTF8 template: {}", file_path)))?;

            // render the file
            let rendered = Self::render(content, project)?;

            // strip .hbs extension to get the output path
            let output_rel = file_path.strip_suffix(".hbs").unwrap_or(&file_path);

            // get the destination dir
            let dest = format!("{}/{}", project.directory, output_rel);

            Writer::write_file(&dest, &rendered)?;
        }

        Ok(())
    }

    pub fn render(content: &str, project: &NewProject) -> Result<String> {
        let data = Self::build_context(project);

        let hbs = Self::engine();

        hbs.render_template(content, &data)
            .map_err(|e| AxionError::Template(e.to_string()))
    }

    fn build_context(project: &NewProject) -> Value {
        json!({
            "project": {
                "name": project.name,
            },
            "database": {
                "none":     project.database == Database::None,
                "postgres": project.database == Database::PostgreSQL,
                // Add more cases in here
            },
            "auth": {
                "none":    project.auth == Auth::None,
                "jwt":     project.auth == Auth::Jwt,
                // Add more cases in here
            },
            "cache": {
                "none":  project.cache == Cache::None,
                "redis": project.cache == Cache::Redis,
                // Add more cases in here
            },
            "containerize": {
                "none":   project.containerize == Containerize::None,
                "docker": project.containerize == Containerize::Docker,
                // Add more cases in here
            }
        })
    }
}
