use std::{
    collections::HashMap,
    hash::Hash,
    io,
    path::PathBuf,
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
            buffer.clear();
            
            if let Err(e) = io::stdin().read_line(&mut buffer) {
                println!("Error when reading input: {}", e);
            }

            let mut parser = CommandParser::new(&buffer.trim());
            let command = match parser.command() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error occured trying to get command: {}", e);
                    continue;
                }
            };

            match command {
                Command::Merge => {
                    let Ok(path) = parser.path() else {
                        continue
                    };

                    let Some(dest_project) = &mut self.project else {
                        println!("No active project");
                        continue
                    };

                    let mut source_project = match Project::open(&path) {
                        Ok(p) => p,
                        Err(e) => {
                            println!("Error opening project: {}", e);
                            continue
                        },
                    }; 
                    
                    Self::prompt_merge(&mut dest_project.functions, &mut source_project.functions);
                    Self::prompt_merge(&mut dest_project.globals, &mut source_project.globals);
                    Self::prompt_merge(&mut dest_project.types, &mut source_project.types);
                }
                Command::Open => {
                    let Ok(path) = parser.path() else {
                        continue
                    };

                    self.project = Some(match Project::open(&path) {
                        Ok(p) => p,
                        Err(e) => {
                            println!("Error opening project: {}", e);
                            continue;
                        }
                    })
                }
                Command::New => {
                    self.project = Some(Project::new());
                }
                Command::Save => {
                    let Ok(path) = parser.path() else {
                        continue
                    };

                    println!("{:?}", path);
                    
                    let Some(project) = &self.project else {
                        println!("No active project");
                        continue
                    };

                    if let Err(e) = project.save(&path) {
                        println!("Error saving project: {}", e)
                    }
                }
            }
        }
    }

    fn prompt_choice(string: &str) -> bool {
        loop {
            println!("{} (Y/N)", string);

            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_line(&mut buffer) {
                println!("Error when reading input: {}", e);
                continue;
            }

            let mut parser = CommandParser::new(&buffer);
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
}
