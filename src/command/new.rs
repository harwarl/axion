use colored::Colorize;

use crate::error::Result;
use crate::{find::Finder, prompt::NewProject};

pub fn new(name: &String) -> Result<()> {
    println!(
        "{}",
        format!("Create a new Axum project, {}", name)
            .cyan()
            .italic()
    );

    if Finder::exists(name) {
        eprintln!(
            "{}",
            format!("❌ Folder '{}' already exists", name).red().bold()
        );
        std::process::exit(1)
    };

    // Generate Prompts and get the New project Struct
    let project = NewProject::from_prompt(name);

    // Create a scaffold based on the project struct
    Ok(())
}
