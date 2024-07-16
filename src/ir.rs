use serde::{Deserialize, Serialize};

// HELPER OBJECTS

#[derive(Debug, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Pointer {
        to: String,
    },
    FnPointer {
        args: Vec<String>,
        ret: String,
    },
    Struct {
        fields: Vec<StructField>,
    },
    Enum {
        values: Vec<EnumValue>,
    },
    Array {
        item: String,
    },
    None,
}

impl Default for TypeInfo {
    fn default() -> Self {
        TypeInfo::None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block(usize, usize);

// PRIMARY OBJECTS

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub size: usize,
    pub alignment: usize,

    #[serde(default)]
    pub info: TypeInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub blocks: Vec<Block>,
    pub arguments: Vec<Argument>,
    pub return_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Global {
    pub location: usize,
    pub r#type: String,
}
