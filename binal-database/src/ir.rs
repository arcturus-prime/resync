use serde::{Deserialize, Serialize};
use binal_database_macros::Object;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Argument {
    pub name: String,
    pub arg_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArrayType {
    item_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumType {
    values: Vec<EnumValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructType {
    fields: Vec<StructField>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionType {
    arg_types: Vec<String>,
    return_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointerType {
    to_type: String,
    depth: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeInfo {
    Pointer(PointerType),
    Function(FunctionType),
    Struct(StructType),
    Enum(EnumType),
    Array(ArrayType),
}

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct Type {
    #[id]
    name: String,
    size: usize,
    alignment: usize,
    info: TypeInfo,
}

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct Function {
    #[id]
    name: String,
    location: usize,
    arguments: Vec<Argument>,
    return_type: String,
}

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct Global {
    #[id]
    name: String,
    location: usize,
    global_type: String,
}
