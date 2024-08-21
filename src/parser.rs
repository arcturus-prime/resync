use std::path::PathBuf;

use crate::error::Error;

pub enum Command {
    Merge,
    Open,
    New,
    Save,
}

pub struct CommandParser<'a> {
    string: &'a str,
}

impl<'a> CommandParser<'a> {
    pub fn new(string: &'a str) -> Self {
        Self { string }
    }
    #[allow(dead_code)]
    pub fn peek(&self, num: usize) -> &str {
        self.string.split_at(num).0
    }

    pub fn consume(&mut self, num: usize) -> &str {
        let pair = self.string.split_at(num);
        self.string = pair.1;

        pair.0
    }

    pub fn command(&mut self) -> Result<Command, Error> {
        let nextgap = self.string.chars().position(|c| c == ' ' || c == '\n' || c == '\r').unwrap_or(self.string.len());

        match self.consume(nextgap) {
            "open" => Ok(Command::Open),
            "merge" => Ok(Command::Merge),
            "new" => Ok(Command::New),
            "save" => Ok(Command::Save),
            _ => Err(Error::Binal(String::from("Invalid command"))),
        }
    }

    pub fn path(&mut self) -> Result<PathBuf, Error> {
        let lead = self.consume(1);

        let pair = if lead == "\"" {
            match self.string.split_once("\"") {
                Some(p) => p,
                None => return Err(Error::Binal(String::from("Could not find matching \""))),
            }
        } else if lead == "\'" {
            match self.string.split_once("\'") {
                Some(p) => p,
                None => return Err(Error::Binal(String::from("Could not find matching \'"))),
            }
        } else {
            match self.string.split_once(" ") {
                Some(p) => p,
                None => (self.string, ""),
            }
        };

        self.string = pair.1;

        Ok(PathBuf::from(pair.0))
    }

    pub fn option(&mut self) -> Result<bool, Error> {
        let option = self.consume(1);
        
        if option.to_lowercase() == "y" {
            return Ok(true);
        } else if option.to_lowercase() == "n" {
            return Ok(false);
        }

        Err(Error::Binal(String::from("Invalid option")))
    }
}