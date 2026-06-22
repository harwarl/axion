use colored::Colorize;

pub fn ask(msg: &str) -> String {
    let question_mark = "?".cyan().bold();
    format!("{} {}", question_mark, msg)
}
