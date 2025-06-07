use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::net::{self, Object};

#[derive(Debug)]
pub enum DatabaseError {
    Io(std::io::Error),
    Serde(serde_json::Error),
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
        }
    }
}

#[derive(Serialize, Deserialize)]
struct StructMember {
    name: String,
    r#type: TypeRef,
    offset: usize,
}

#[derive(Serialize, Deserialize)]
struct EnumValue {
    name: String,
    value: usize,
}

#[derive(Serialize, Deserialize)]
struct UnionMember {
    name: String,
    r#type: TypeRef,
}

#[derive(Serialize, Deserialize)]
enum TypeRef {
    Int(u16),
    Uint(u16),
    Float(u16),
    Value(usize),
    Pointer(u8, usize),
}

#[derive(Serialize, Deserialize)]
enum TypeInfo {
    Struct(Vec<StructMember>),
    Enum(Vec<EnumValue>),
    Union(Vec<UnionMember>),
    TypeDef(TypeRef),
    Function(Vec<TypeRef>, TypeRef),
    Array(TypeRef, usize),
}

#[derive(Serialize, Deserialize)]
struct Type {
    pub name: String,
    size: usize,
    alignment: usize,
    info: TypeInfo,
}

impl Default for Type {
    fn default() -> Self {
        Type {
            name: String::new(),
            size: 0,
            alignment: 0,
            info: TypeInfo::TypeDef(TypeRef::Uint(0)),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[repr(u8)]
enum Instruction {
    Const24,
    Const16,
    Const8,
}

#[derive(Serialize, Deserialize)]
struct Function {
    pub name: String,
    code: Vec<Instruction>,

    location: usize,

    return_type: TypeRef,
    argument_names: Vec<String>,
    argument_types: Vec<TypeRef>,
}

impl Default for Function {
    fn default() -> Self {
        Function {
            name: String::new(),
            code: Vec::new(),

            location: 0,

            return_type: TypeRef::Uint(0),
            argument_names: Vec::new(),
            argument_types: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub name: String,
    location: usize,
    r#type: TypeRef,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            name: String::new(),
            location: 0,
            r#type: TypeRef::Uint(0),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct IdVec<T> {
    array: Vec<T>,
    reverse_lookup: Vec<usize>,

    lookup: Vec<usize>,
    holes: Vec<usize>,
}

impl<T> IdVec<T> {
    pub fn push(&mut self, item: T) -> usize {
        let id;

        self.array.push(item);

        if let Some(hole) = self.holes.pop() {
            id = hole;
            self.lookup[id] = self.reverse_lookup.len();
            self.reverse_lookup.push(id);
        } else {
            id = self.lookup.len();
            self.lookup.push(self.reverse_lookup.len());
            self.reverse_lookup.push(id);
        }

        id
    }

    pub fn delete(&mut self, id: usize) {
        let index = self.lookup[id];

        self.lookup[*self.reverse_lookup.last().unwrap()] = index;
        self.array.swap_remove(index);
        self.reverse_lookup.swap_remove(index);
        self.holes.push(id);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.array.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.array.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Database {
    pub functions: IdVec<Function>,
    pub types: IdVec<Type>,
    pub data: IdVec<Data>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let db: Database = serde_json::from_slice(project_data.as_slice())?;

        Ok(db)
    }

    pub fn save(&self, path: &Path) -> Result<(), DatabaseError> {
        let mut file;

        if !path.exists() {
            file = File::create(path)?;
        } else {
            file = OpenOptions::new().write(true).open(path)?;
        }

        let data = serde_json::to_vec(&self)?;
        file.write_all(&data)?;

        Ok(())
    }
}
