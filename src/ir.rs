use std::{
    fs::{File, OpenOptions}, io::{Read, Write}, path::Path
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
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    objects: Vec<Object>,
    names: Vec<String>,
}

impl Project {
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
        let project = serde_json::from_slice(project_data.as_slice())?;

        Ok(project)
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

    pub fn get_obj_mut(&mut self, id: usize) -> &mut Object {
        &mut self.objects[id]
    }

    pub fn get_obj(&self, id: usize) -> &Object {
        &self.objects[id]
    }

    pub fn get_name_mut(&mut self, id: usize) -> &mut String {
        &mut self.names[id]
    }

    pub fn get_name(&self, id: usize) -> &String {
        &self.names[id]
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}