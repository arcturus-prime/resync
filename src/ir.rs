use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

// HELPER OBJECTS

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub r#type: String,
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Pointer { to: String, depth: usize },
    Function { args: Vec<String>, ret: String },
    Struct { fields: Vec<StructField> },
    Enum { values: Vec<EnumValue> },
    Array { item: String },
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Argument {
    pub name: String,
    pub r#type: String,
}

// PRIMARY OBJECTS

pub trait Object: Sized + for<'a> Decode<'a> + Encode + Serialize + for<'a> Deserialize<'a> {
    const KIND: &'static str;
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Type {
    size: usize,
    alignment: usize,
    info: TypeInfo,
}

impl Object for Type {
    const KIND: &'static str = "type";
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Function {
    location: usize,
    arguments: Vec<Argument>,
    return_type: String,
}

impl Object for Function {
    const KIND: &'static str = "function";
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Global {
    location: usize,
    type_: String,
}

impl Object for Global {
    const KIND: &'static str = "global";
}