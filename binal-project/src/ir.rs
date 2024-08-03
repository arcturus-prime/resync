use std::default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    size: usize,
    alignment: usize,

    #[serde(default)]
    info: TypeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    location: usize,
    arguments: Vec<Argument>,
    return_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Global {
    location: usize,
    global_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub arg_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Pointer {
        to_type: String,
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
    None,
}

impl Default for TypeInfo {
    fn default() -> Self {
        Self::None
    }
}