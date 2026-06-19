use super::steps::{
    AuthStep, BaseStep, CacheStep, DependenciesStep, DockerStep, MainStep, ScaffoldStep,
};
use crate::error::Result;
use crate::prompt::NewProject;
use colored::Colorize;

pub struct Scaffolder<'a> {
    project: &'a NewProject,
    steps: Vec<Box<dyn ScaffoldStep>>,
}

impl<'a> Scaffolder<'a> {
    pub fn new(new_project: &'a NewProject) -> Self {
        let steps: Vec<Box<dyn ScaffoldStep>> = vec![
            Box::new(BaseStep),
            Box::new(DependenciesStep),
            Box::new(MainStep),
            Box::new(AuthStep),
            Box::new(CacheStep),
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

        let total = enabled.len();

        for (i, step) in enabled.into_iter().enumerate() {
            // TODO: Add an indicator
            eprintln!(
                "  {} [{}/{}] {}",
                "→".cyan(),
                i + 1,
                total,
                step.label().dimmed()
            );

            // Run the step
            step.run(self.project)?;
        }

        Ok(())
    }
}
