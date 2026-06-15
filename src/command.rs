use colored::Colorize;

use crate::prompt::NewProject;

pub fn new(name: &String) {
    println!(
        "{}",
        format!("Create a new Axion Project, {}", name)
            .cyan()
            .bold()
    );
    // Check if the folder already exists in the current directory

    // Create prompts for a new project
    let _project = NewProject::from_prompt(name);

    // generate files, pages, folders needed
}

pub fn add(name: &String, kind: &String) {
    // check if the kind directoory is already available
    // throw error if yes

    // Create Prompts to add files based on what is needed
    NewProject::add(name, kind);

    // Generate files and directories needed
}
