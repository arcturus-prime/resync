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
    pub item_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumType {
    pub values: Vec<EnumValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructType {
    pub fields: Vec<StructField>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionType {
    pub arg_types: Vec<String>,
    pub return_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointerType {
    pub to_type: String,
    pub depth: usize,
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
    pub name: String,
    pub size: usize,
    pub alignment: usize,
    #[bitcode]
    pub info: TypeInfo,
}

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct Function {
    #[id]
    pub name: String,
    pub location: usize,
    #[bitcode]
    pub arguments: Vec<Argument>,
    pub return_type: String,
}

#[derive(Debug, Object, Serialize, Deserialize, Clone)]
pub struct Global {
    #[id]
    pub name: String,
    pub location: usize,
    pub global_type: String,
}
