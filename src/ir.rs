use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    collections::HashMap,
    path::Path,
    fmt::Display,
};

use crate::net::Object;

#[derive(Debug)]
pub enum DatabaseError {
    Io(std::io::Error),
    Serde(serde_json::Error),
    ObjectDoesNotExist,
}

impl From<std::io::Error> for DatabaseError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl<'a> Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Io(e) => e.fmt(f),
            DatabaseError::Serde(e) => e.fmt(f),
            DatabaseError::ObjectDoesNotExist => f.write_str("Object does not exist in database"),
        }
    }
}


#[repr(u8)]
enum Instruction {
    Const24,
    Const16,
    Const8,
}

struct Function {
    name: String,
    code: Vec<Instruction>,
    
}

pub struct Type {

}

pub struct Database {
    objects: HashMap<String, Object>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let objects: HashMap<String, Object> = serde_json::from_slice(project_data.as_slice())?;

        Ok(Database { objects })
    }

    pub fn save(&self, path: &Path) -> Result<(), DatabaseError> {
        let mut file;

        if !path.exists() {
            file = File::create(path)?;
        } else {
            file = OpenOptions::new().write(true).open(path)?;
        }

        let data = serde_json::to_vec(&self.objects)?;
        file.write_all(&data)?;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn name_iter(&self) -> impl Iterator<Item = &String> + '_ {
        self.objects.keys()
    }

    pub fn get(&self, name: &str) -> Result<HashMap<String, Object>, DatabaseError> {
        let mut map = HashMap::new();

        map.insert(name.to_string(), self.objects[name].clone());

        Ok(map)
    }

    pub fn push(&mut self, objects: HashMap<String, Object>) {
        self.objects.extend(objects)
    }

    pub fn delete(&mut self, name: &str) -> Result<(), DatabaseError> {
        self.objects.remove(name);

        Ok(())
    }
}
