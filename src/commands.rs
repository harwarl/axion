use colored::Colorize;

use crate::{find::Finder, prompt::NewProject};

pub fn new(name: &String) {
    println!(
        "{}",
        format!("Create a new Axion Project, {}", name)
            .cyan()
            .bold()
    );

    // Check if the folder already exists in the current directory
    if Finder::exists(name) {
        eprintln!(
            "{}",
            format!("❌ Folder '{}' already exists", name).red().bold()
        );
        std::process::exit(1);
    }

    // Create prompts for a new project
    let project = NewProject::from_prompt(name);

    // TODO: create a struct to keep track of what has been generated
    // generate files, pages, folders needed
}

pub fn add(name: &String, kind: &String) {
    // check if the kind directory is already available

    // throw error if yes

    // Create Prompts to add files based on what is needed
    NewProject::add(name, kind);

    // Generate files and directories needed
}
