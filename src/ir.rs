use std::{
    cell::Cell, collections::HashMap, fmt::Display, fs::{create_dir_all, File, OpenOptions}, io::{Read, Write}, path::Path
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub size: usize,
    pub alignment: usize,
    pub info: TypeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    pub location: usize,
    pub arguments: Vec<Argument>,
    pub return_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Global {
    pub location: usize,
    pub global_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Argument {
    pub name: String,
    pub arg_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Pointer {
        to_type: usize,
        depth: usize,
    },
    Function {
        arg_types: Vec<String>,
        return_type: String,
    },
    Struct {
        fields: Vec<StructField>,
    },
    Enum {
        values: Vec<EnumValue>,
    },
    Array {
        item_type: String,
    },
    Int,
    Uint,
    Float,
}

pub enum ObjectKind {
    Functions,
    Types,
    Globals,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,

    pub types: HashMap<String, Type>,
    pub functions: HashMap<String, Function>,
    pub globals: HashMap<String, Global>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            name,
            types: HashMap::new(),
            functions: HashMap::new(),
            globals: HashMap::new(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, Error> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let project = serde_json::from_slice(project_data.as_slice())?;

        Ok(project)
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let mut transaction;

        if !path.exists() && path.parent().is_some() {
            create_dir_all(path.parent().unwrap())?;
            transaction = File::create(path)?;
        } else {
            transaction = OpenOptions::new().write(true).open(path)?;
        }

        let data = serde_json::to_vec_pretty(self)?;
        transaction.write(&data)?;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.functions.is_empty() && self.globals.is_empty() && self.types.is_empty()
    }
}