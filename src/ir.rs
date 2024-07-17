use serde::{Deserialize, Serialize};

// HELPER OBJECTS

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Pointer { to: String },
    FnPointer { args: Vec<String>, ret: String },
    Struct { fields: Vec<StructField> },
    Enum { values: Vec<EnumValue> },
    Array { item: String },
    None,
}

impl Default for TypeInfo {
    fn default() -> Self {
        TypeInfo::None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Argument {
    pub name: String,
    pub r#type: String,
}

// PRIMARY OBJECTS

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Type {
    size: usize,
    alignment: usize,

    #[serde(default)]
    info: TypeInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    blocks: Vec<(usize, usize)>,
    arguments: Vec<Argument>,
    return_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Global {
    location: usize,
    r#type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Object {
    Type {
        size: usize,
        alignment: usize,

        #[serde(default)]
        info: TypeInfo,
    },
    Function {
        blocks: Vec<(usize, usize)>,
        arguments: Vec<Argument>,
        return_type: String,
    },
    Global {
        location: usize,
        r#type: String,
    }
}