use serde::{Deserialize, Serialize};

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
pub enum TypeInfo {
    Pointer {
        to_type: String,
    },
    FnPointer {
        arg_types: Vec<String>,
        ret_type: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    Type {
        size: usize,
        alignment: usize,
        info: TypeInfo,
    },
    Function {
        location: usize,
        size: usize,
        args: Vec<Argument>,
        ret_type: String,
    },
    Global {
        location: usize,
        r#type: String,
    }
}