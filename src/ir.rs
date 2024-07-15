use serde::{Deserialize, Serialize};

pub type ObjectRef = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub r#type: ObjectRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum TypeInfo {
    Pointer {
        to_type: ObjectRef,
    },
    FnPointer {
        arg_types: Vec<ObjectRef>,
        ret_type: ObjectRef,
    },
    Struct {
        fields: Vec<StructField>,
    },
    Enum {
        values: Vec<EnumValue>,
    },
    Array {
        item_type: ObjectRef,
    },
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub r#type: ObjectRef,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Object {
    Type {
        name: String,
        size: usize,
        alignment: usize,
        info: TypeInfo,
    },
    Function {
        name: String,
        blocks: Vec<(usize, usize)>,
        arguments: Vec<Argument>,
        return_type: ObjectRef,
    },
    Global {
        name: String,
        location: usize,
        r#type: ObjectRef,
    }
}