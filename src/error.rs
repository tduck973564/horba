use colored::Colorize as Colourise;
use std::fmt::{Display, Formatter};

pub fn report(
    line: u32,
    column: u32,
    log_level: LogLevel,
    location: &str,
    message: &str,
    source: &str,
) {
    eprintln!(
        "{} {}: {}\n {} line {} : column {}\n   {} {}\t{}\n",
        log_level.to_string(),
        location.bold(),
        message,
        "-->".bold().blue(),
        &line.to_string().bold(),
        &column.to_string().bold(),
        line.to_string().blue().bold(),
        "|".blue().bold(),
        source.lines().nth(line as usize - 1).unwrap_or(""),
    );
}

#[derive(Clone, Copy)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => f.write_str(&"error".bold().red().to_string()),
            LogLevel::Warning => f.write_str(&"warning".bold().yellow().to_string()),
            LogLevel::Info => f.write_str(&"info".bold().blue().to_string()),
        }
    }
}

pub trait Error {
    fn report(&self, source: &str);
}