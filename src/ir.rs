use bitcode::{Decode, Encode};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::error::Error;

// HELPER OBJECTS

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: String,
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
    Pointer { to_type: String, depth: usize },
    Function { arg_types: Vec<String>, return_type: String },
    Struct { fields: Vec<StructField> },
    Enum { values: Vec<EnumValue> },
    Array { item_type: String },
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Argument {
    pub name: String,
    pub arg_type: String,
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct PointerType {
    to_type: String, depth: usize 
}

// PRIMARY OBJECTS

pub trait DBObject: Sized + for<'a> Decode<'a> + Encode + Serialize + for<'a> Deserialize<'a> {
    type Row;

    async fn read(row: Self::Row) -> Result<Self, Error>;
    async fn write(object: &Self) -> Result<Self::Row, Error>;
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Type<T> {
    name: String,
    size: usize,
    alignment: usize,
    info: T,
}

async fn decode_with_bitcode<'a, T: Decode<'a>>(data: &'a [u8]) -> Result<T, Error> {
    match bitcode::decode(data) {
        Ok(info) => info,
        Err(e) => return Err(Error::Bitcode(e))
    }
}

impl DBObject for Type<PointerType> {
    type Row = (String, usize, usize, String, String);

    async fn read(row: Self::Row) -> Result<Self, Error> {
        Ok(Self {
            name: row.0,
            size: row.1,
            alignment: row.2,
            info: ,
        })
    }

    async fn write(object: Self) -> Result<Self::Row, Error> {
        todo!()
    }
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Function {
    location: usize,
    arguments: Vec<Argument>,
    return_type: String,
}

impl DBObject for Function {
    type Row;

    async fn read(row: &Self::Row) -> Result<Self, Error> {
        todo!()
    }

    async fn write(object: &Self) -> Result<Self::Row, Error> {
        todo!()
    }
}

#[derive(Debug, Decode, Encode, Serialize, Deserialize, Clone)]
pub struct Global {
    location: usize,
    global_type: String,
}

impl DBObject for Global {
    type Row;

    async fn read(row: &Self::Row) -> Result<Self, Error> {
        todo!()
    }

    async fn write(object: &Self) -> Result<Self::Row, Error> {
        todo!()
    }
}