use std::time::Duration;

use super::steps::{BaseStep, DependenciesStep, DockerStep, ScaffoldStep, TemplateStep};
use crate::error::Result;
use crate::prompt::NewProject;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Scaffolder<'a> {
    project: &'a NewProject,
    steps: Vec<Box<dyn ScaffoldStep>>,
}

impl<'a> Scaffolder<'a> {
    pub fn new(new_project: &'a NewProject) -> Self {
        let steps: Vec<Box<dyn ScaffoldStep>> = vec![
            Box::new(BaseStep),
            Box::new(DependenciesStep),
            Box::new(TemplateStep),
            Box::new(DockerStep), //TODO: Add More Steps in Here
        ];

        Self {
            project: new_project,
            steps,
        }
    }

    pub fn run(self) -> Result<()> {
        // Iterate self.steps get only enabled projects and print the label and run the steps
        let enabled: Vec<Box<dyn ScaffoldStep>> = self
            .steps
            .into_iter()
            .filter(|e| e.enabled(self.project))
            .collect();

        for (_, step) in enabled.into_iter().enumerate() {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::with_template("{spinner:.cyan} {msg}")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            );
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_message(step.label().to_string());

            let result = step.run(self.project);

            match &result {
                Ok(_) => pb.finish_with_message(format!("✅ {}", step.label())),
                Err(e) => pb.finish_with_message(format!("❌ {} — {}", step.label(), e)),
            }

            result?;
        }

        Ok(())
    }
}
