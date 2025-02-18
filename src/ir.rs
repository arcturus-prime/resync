use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path, string,
    collections::HashMap
};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Argument {
    pub name: String,
    pub arg_type: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: usize,
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
    Int,
    Uint,
    Float,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Object {
    Type {
        size: usize,
        alignment: usize,
        info: TypeInfo,
    },
    Function {
        location: usize,
        arguments: Vec<Argument>,
        return_type: usize,
    },
    Global {
        location: usize,
        global_type: usize,
    },
    Null
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub objects: Vec<Object>,
    pub names: Vec<String>,
}

impl ProjectData {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            names: Vec::new(),
        }
    }
    
    pub fn open(path: &Path) -> Result<Self, Error> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let object_list = serde_json::from_slice(project_data.as_slice())?;

        Ok(object_list)
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let mut transaction;

        if !path.exists() {
            transaction = File::create(path)?;
        } else {
            transaction = OpenOptions::new().write(true).open(path)?;
        }

        let data = serde_json::to_vec_pretty(&self.objects)?;
        transaction.write(&data)?;

        Ok(())
    }
}

pub struct Project {
    pub data: ProjectData,
    pub lookup: HashMap<String, usize>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            data: ProjectData::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, Error> {
        let data = ProjectData::open(path)?;
        let mut lookup = HashMap::new();

        for (i, name) in data.names.iter().enumerate() {
            lookup.insert(name.to_string(), i);
        }

        Ok(Self {
            data,
            lookup
        })
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        self.data.save(path)
    }
}
