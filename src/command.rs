use colored::Colorize;

pub fn new(name: Option<&String>) {
    println!("{}", format!("Create a new Axion Project, {}", name.unwrap()).purple().bold());
}

pub fn add(name: &String, kind: &String) {}