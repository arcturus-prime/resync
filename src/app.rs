use std::{
    collections::HashMap,
    hash::Hash,
    io,
    path::{Path, PathBuf},
};

use crate::error::Error;
use crate::ir::Project;

pub enum Command {
    Merge,
    Open,
    New,
    Save,
}

pub struct CommandParser<'a> {
    string: &'a [u8],
}

impl<'a> CommandParser<'a> {
    pub fn new(string: &'a [u8]) -> Self {
        Self { string }
    }

    pub fn command(&mut self) -> Result<Command, Error> {

        self.string = pair.1;

        match pair.0 {
            "open" => Ok(Command::Open),
            "merge" => Ok(Command::Merge),
            "new" => Ok(Command::New),
            "save" => Ok(Command::Save),
            _ => Err(Error::Binal(String::from("Invalid command found"))),
        }
    }

    pub fn path(&mut self) -> Result<PathBuf, Error> {
        let lead = self.string[0];

        let pair = if lead == b'\"' {
            match self.string[1..].split_once("\"") {
                Some(p) => p,
                None => return Err(Error::Binal(String::from("Could not find matching \""))),
            }
        } else if lead == b'\'' {
            match self.string[1..].split_once("\'") {
                Some(p) => p,
                None => return Err(Error::Binal(String::from("Could not find matching \'"))),
            }
        } else {
            match self.string.split_once(" ") {
                Some(p) => p,
                None => ("", self.string),
            }
        };

        self.string = pair.1;

        Ok(PathBuf::from(pair.0))
    }

    pub fn option(&mut self) -> Result<bool, Error> {
        if self.string[0..=1].to_lowercase() == "y" {
            self.string = &self.string[1..];
            return Ok(true);
        } else if self.string[0..=1].to_lowercase() == "n" {
            self.string = &self.string[1..];
            return Ok(false);
        }

        Err(Error::Binal(String::from("Invalid option")))
    }
}

pub struct App {
    pub project: Option<Project>,
}

impl App {
    pub fn new() -> Self {
        Self { project: None }
    }

    pub fn process(&mut self) {
        let mut buffer = String::new();

        loop {
            if let Err(e) = io::stdin().read_line(&mut buffer) {
                println!("Error when reading command: {}", e);
            }

            let mut parser = CommandParser::new(buffer.as_bytes());
            let command = match parser.command() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error occured trying to get command: {}", e);
                    return;
                }
            };

            match command {
                Command::Merge => {
                    let path = Self::prompt_path();

                    if let Err(e) = self.merge(&path) {
                        println!("Error occured while merging projects {}", e)
                    }
                }
                Command::Open => {
                    let path = Self::prompt_path();

                    self.project = Some(match Project::open(&path) {
                        Ok(p) => p,
                        Err(e) => {
                            println!("Error opening project: {}", e);
                            return;
                        }
                    })
                }
                Command::New => {
                    self.project = Some(Project::new());
                }
                Command::Save => {
                    let path = Self::prompt_path();

                    let Some(project) = &self.project else {
                        println!("No active project");
                        return
                    };

                    project.save(&path);
                }
            }
        }
    }

    fn prompt_path() -> PathBuf {
        loop {
            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_line(&mut buffer) {
                println!("Error when reading path: {}", e);
                continue;
            }

            let mut parser = CommandParser::new(buffer.as_bytes());
            let opt = match parser.path() {
                Ok(opt) => opt,
                Err(e) => {
                    println!("Error parsing path: {}", e);
                    continue;
                }
            };

            return opt
        }
    }

    fn prompt_choice(string: &str) -> bool {
        loop {
            println!("{} (Y/N)", string);

            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_line(&mut buffer) {
                println!("Error when reading option: {}", e);
                continue;
            }

            let mut parser = CommandParser::new(buffer.as_bytes());
            let opt = match parser.option() {
                Ok(opt) => opt,
                Err(e) => {
                    println!("Error parsing option: {}", e);
                    continue;
                }
            };

            return opt
        }
    }

    fn prompt_merge<A: Eq + Hash + Clone, B>(map1: &mut HashMap<A, B>, map2: &mut HashMap<A, B>) {
        let mut to_move = Vec::new();

        for pair in map2.iter() {
            if map1.contains_key(pair.0) {
                if Self::prompt_choice(&format!(
                    "{}",
                    "This could overwrite an existing object, continue?"
                )) {
                    to_move.push(pair.0.clone());
                }
            } else {
                to_move.push(pair.0.clone());
            }
        }

        for key in to_move {
            map1.insert(key.clone(), map2.remove(&key).unwrap());
        }
    }

    fn merge(&mut self, path: &Path) -> Result<(), Error> {
        let mut source_project = Project::open(path)?;

        let Some(dest_project) = &mut self.project else {
            return Err(Error::Binal(String::from("No active project")));
        };

        Self::prompt_merge(&mut dest_project.functions, &mut source_project.functions);
        Self::prompt_merge(&mut dest_project.globals, &mut source_project.globals);
        Self::prompt_merge(&mut dest_project.types, &mut source_project.types);

        Ok(())
    }
}
