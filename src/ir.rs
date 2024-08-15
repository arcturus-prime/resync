use std::{
    collections::HashMap, fs::{create_dir_all, File, OpenOptions}, io::{Read, Write}, path::Path
};

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct Type {
    pub size: usize,
    pub alignment: usize,

    #[serde(default)]
    pub info: TypeInfo,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct Function {
    pub location: usize,
    pub arguments: Vec<Argument>,
    pub return_type: usize,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct Global {
    pub location: usize,
    pub global_type: usize,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub arg_type: usize,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: usize,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Pointer {
        to_type: usize,
        depth: usize,
    },
    Function {
        arg_types: Vec<usize>,
        return_type: usize,
    },
    Struct {
        fields: Vec<StructField>,
    },
    Enum {
        values: Vec<EnumValue>,
    },
    Array {
        item_type: usize,
    },
    None,
}

impl Default for TypeInfo {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Bitcode(bitcode::Error),
    TypeInvalid(String),
    PathInvalid,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<bitcode::Error> for Error {
    fn from(value: bitcode::Error) -> Self {
        Self::Bitcode(value)
    }
}


#[derive(Debug, Encode, Decode)]
pub struct Project {
    pub types: HashMap<usize, Type>,
    pub functions: HashMap<usize, Function>,
    pub globals: HashMap<usize, Global>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            functions: HashMap::new(),
            globals: HashMap::new(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, Error> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let project = bitcode::decode(project_data.as_slice())?;

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

        let data = bitcode::encode(self);
        transaction.write(&data)?;

        Ok(())
    }
}