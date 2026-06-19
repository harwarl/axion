use axion::command::{add, new};
use axion::error::Result;
use clap::{Arg, Command};
use colored::Colorize;
use std::process::exit;

macro_rules! die {
    ($fmt:expr) => ({
        eprintln!("{}", $fmt.red().bold());
        exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        exit(1);
    });
}

fn main() -> Result<()> {
    let matches = Command::new("axion")
        .about("Create a onion architecture boilerplate for your rust projects")
        .override_usage("axion <COMMAND> [ARGS]")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new").about("Create a new axion").arg(
                Arg::new("name")
                    .help("Name of the project")
                    .required(false)
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
        .subcommand(
            Command::new("init")
                .about("Creates a new axion project in the specified directory")
                .arg(
                    Arg::new("directory")
                        .help("Directory to scaffold in. Use . for current folder")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            // TODO: get the directory for the project name
            let directory = format!(
                "{}/{}",
                std::env::current_dir().unwrap().to_string_lossy(),
                name
            );
            new::new(name, &directory)?
        }
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("directory").unwrap();
            let directory = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string();
            new::new(name, &directory)?
        }
        Some(("add", sub_matches)) => {
            let kind = sub_matches.get_one::<String>("type").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();
            add::add(name, kind)?
        }
        _ => die!("error: missing required argument <COMMAND>"),
    };

    Ok(())
}
