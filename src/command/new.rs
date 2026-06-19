use colored::Colorize;

use crate::error::Result;
use crate::prompt::NewProject;
use crate::scaffold::scaffolder::Scaffolder;

pub fn new(name: &String, directory: &String) -> Result<()> {
    println!(
        "{}",
        format!("Create a new Axum project, {}", name)
            .cyan()
            .italic()
    );

    // Generate Prompts and get the New project Struct
    let project = NewProject::from_prompt(name, directory);

    // Create a scaffold based on the project struct
    let scaffolder = Scaffolder::new(&project);
    scaffolder.run()?;
    Ok(())
}
