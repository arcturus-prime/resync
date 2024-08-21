mod ir;
mod parser;
mod error;

use std::{collections::HashMap, hash::Hash, io};

use parser::{Command, CommandParser};
use ir::Project;

fn main() {
    let mut buffer = String::new();
    let mut project: Option<Project> = None;

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

                let Some(dest_project) = &mut project else {
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
                
                prompt_merge(&mut dest_project.functions, &mut source_project.functions);
                prompt_merge(&mut dest_project.globals, &mut source_project.globals);
                prompt_merge(&mut dest_project.types, &mut source_project.types);
            }
            Command::Open => {
                let Ok(path) = parser.path() else {
                    continue
                };

                println!("{:?}", path);

                project = Some(match Project::open(&path) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Error opening project: {}", e);
                        continue;
                    }
                })
            }
            Command::New => {
                project = Some(Project::new());
            }
            Command::Save => {
                let Ok(path) = parser.path() else {
                    continue
                };

                println!("{:?}", path);
                
                let Some(unwrap_project) = &project else {
                    println!("No active project");
                    continue
                };

                if let Err(e) = unwrap_project.save(&path) {
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
            if prompt_choice(&format!(
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