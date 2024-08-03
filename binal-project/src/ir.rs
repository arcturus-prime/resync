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
    pub return_type: String,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct Global {
    pub location: usize,
    pub global_type: String,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub arg_type: String,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: String,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
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
