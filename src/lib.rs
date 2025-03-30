#![crate_type = "lib"]
#![crate_name = "cpeck_logger"]

use std::{fmt, io::Write};

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    ALL = 0,
    DEBUG = 10,
    INFO = 20,
    WARNING = 30,
    ERROR = 40,
}

pub struct Logger {
    pub out: Box<dyn Write>,
    pub level: LogLevel,
    pub formatter: fn (&str, LogLevel) -> String
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LogLevel::DEBUG => fmt::write(f, format_args!("DEBUG")),
            LogLevel::INFO => fmt::write(f, format_args!("INFO")),
            LogLevel::WARNING => fmt::write(f, format_args!("WARNING")),
            LogLevel::ERROR => fmt::write(f, format_args!("ERROR")),
            LogLevel::ALL => fmt::write(f, format_args!("ALL")),
        }
    }
}

impl Logger {
    pub fn debug(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::DEBUG)
    }

    pub fn info(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::INFO)
    }

    pub fn warning(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::WARNING)
    }

    pub fn error(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::ERROR)
    }

    fn log(&mut self, message: &str, level: LogLevel) -> Result<(), Box<dyn std::error::Error>> {
        if self.level <= level {
            let format_string = (self.formatter)(message, level);
            self.out.write_all(format_string.as_bytes())?;
        }

        Ok(())
    }
}