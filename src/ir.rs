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

pub struct Function {

}

pub struct Type {

}

pub struct Database {
    objects: Vec<Object>,
    lookup: HashMap<String, usize>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let object_list: Vec<Object> = serde_json::from_slice(project_data.as_slice())?;

        let mut lookup = HashMap::new();
        for (id, object) in object_list.iter().enumerate() {
            lookup.insert(object.name.to_string(), id);
        }

        Ok(Database { objects: object_list, lookup })
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
        self.objects.iter().map(|obj| &obj.name)
    }

    pub fn get(&self, name: &str) -> Result<Object, DatabaseError> {
        let index = match self.lookup.get(name) {
            Some(index) => *index,
            None => return Err(DatabaseError::ObjectDoesNotExist)
        };

        Ok(self.objects[index].clone())
    }

    pub fn push(&mut self, object: Object) {
        if let Some(id) = self.lookup.get(&object.name) {
            self.objects[*id] = object;
        } else {
            self.lookup.insert(object.name.clone(), self.objects.len());

            self.objects.push(object);
        }
    }

    pub fn rename(&mut self, old: &str, new: String) -> Result<(), DatabaseError> {
        let index = match self.lookup.get(old) {
            Some(index) => *index,
            None => return Err(DatabaseError::ObjectDoesNotExist)
        };

        self.objects[index].name = new.clone();
        self.lookup.remove(old);
        self.lookup.insert(new, index);

        Ok(())
    }

    pub fn delete(&mut self, name: &str) -> Result<(), DatabaseError> {
        let index = match self.lookup.get(name) {
            Some(index) => *index,
            None => return Err(DatabaseError::ObjectDoesNotExist)
        };

        let last = self.objects.len() - 1;
        if index != last {
            self.objects.swap(index, last);
            self.objects.pop();
        }

        self.lookup
            .insert(self.objects[index].name.clone(), index);
        self.lookup.remove(name);

        Ok(())
    }
}
