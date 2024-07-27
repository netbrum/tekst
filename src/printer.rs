use colored::Colorize;
use diff::Result;

pub struct Printer;

impl Printer {
    pub fn print(message: &str) {
        println!("{message}");
    }

    pub fn diff(old: &str, new: &str) {
        for diff in diff::lines(old, new) {
            let result = match diff {
                Result::Left(l) => format!("-{l}").bright_red(),
                Result::Both(l, _) => l.normal(),
                Result::Right(r) => format!("+{r}").bright_green(),
            };

            println!("{result}");
        }
    }
}
