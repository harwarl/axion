use clap::{Arg, Command};
use colored::Colorize;
use std::process::exit;

macro_rules! die {
    ($fmt:expr) => ({
        eprintln!($fmt);
        exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        exit(1);
    });
}

fn main() {
    let matches = Command::new("axion")
        .about("Create a onion architecture boilerplate for your rust projects")
        .override_usage("axion <COMMAND> [ARGS]")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new").about("Create a new axion").arg(
                Arg::new("name")
                    .help("Name of the project")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("add")
                .about("Add a new entity, use case or repo")
                .arg(
                    Arg::new("type")
                        .help("Type to add: entity | use-case | repo")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("name")
                        .help("Name of the type to add")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            println!(
                "{}",
                format!("Creating new Axion project: {}", name)
                    .green()
                    .bold()
            );
            // TODO: handle the new command
        }
        Some(("add", sub_matches)) => {
            let kind = sub_matches.get_one::<String>("type").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();
            println!("{}", format!("Adding {}: {}", kind, name).green().bold());
            // TODO: handle the add command
        }
        _ => die!("error: missing required argument <COMMAND>"),
    };
}
