use colored::Colorize;
use inquire::Select;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Display, Clone, PartialEq)]
pub enum Database {
    PostgreSQL,
    // MySQL,
    // SQLite,
    // MongoDB,
    None,
}

// #[derive(Debug, EnumIter, Display, Clone, PartialEq)]
// pub enum ORM {
//     SQLx,
//     Diesel,
//     SeaORM,
//     None,
// }

#[derive(Debug, EnumIter, Display, Clone, PartialEq)]
pub enum Auth {
    Jwt,
    Session,
    None,
}

#[derive(Debug, EnumIter, Display, Clone, PartialEq)]
pub enum Cache {
    Redis,
    None,
}

#[derive(Debug, EnumIter, Display, Clone, PartialEq)]
pub enum Containerize {
    Docker,
    None,
}

pub struct NewProject {
    pub name: String,
    pub directory: String,
    pub database: Database,
    // pub orm: ORM,
    pub auth: Auth,
    pub cache: Cache,
    pub containerize: Containerize,
}

impl NewProject {
    pub fn from_prompt(name: &String, directory: &String) -> Self {
        let database = Select::new("Select a database: ", Database::iter().collect())
            .prompt()
            .unwrap();

        // let orm = Select::new("Select an ORM: ", ORM::iter().collect())
        //     .prompt()
        //     .unwrap();

        let auth = Select::new("Select auth: ", Auth::iter().collect())
            .prompt()
            .unwrap();

        let cache = Select::new("Select cache: ", Cache::iter().collect())
            .prompt()
            .unwrap();

        let containerize = Select::new("Containerize?", Containerize::iter().collect())
            .prompt()
            .unwrap();

        Self {
            name: name.clone(),
            directory: String::new(),
            database,
            // orm,
            auth,
            cache,
            containerize,
        }
    }

    pub fn add(name: &str, kind: &str) {
        // TODO: to be Adjusted
        match kind {
            "entity" => {
                println!("{}", format!("Adding entity: {}", name).cyan().bold());
                // generator::entity::generate(name)
            }
            "use-case" => {
                println!("{}", format!("Adding use case: {}", name).cyan().bold());
                // generator::use_case::generate(name)
            }
            "repo" => {
                println!("{}", format!("Adding repo: {}", name).cyan().bold());
                // generator::repo::generate(name)
            }
            _ => {
                eprintln!(
                    "{}",
                    format!("Unknown type: {}. Use entity | use-case | repo", kind)
                        .red()
                        .bold()
                );
                std::process::exit(1);
            }
        }
    }
}
