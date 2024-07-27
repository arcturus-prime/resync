use std::{path::Path, time::Duration};

use serde::{Deserialize, Serialize};

use crate::error::Error;

pub trait Database: Sized + Sync {
    fn open(path: &Path) -> impl std::future::Future<Output = Result<Self, Error>> + Send;

    fn write<T: Object>(
        &self,
        id: ObjectRef,
        data: T,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    fn read<T: Object>(
        &self,
        id: ObjectRef,
    ) -> impl std::future::Future<Output = Result<T, Error>> + Send;

    fn changes<T: Object>(
        &self,
        since: Duration,
    ) -> impl std::future::Future<Output = Result<Vec<(ObjectRef, T)>, Error>> + Send;

    fn remove<T: Object>(
        &self,
        id: ObjectRef,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

pub trait Object: Serialize + for<'a> Deserialize<'a> + Send {
    const ID: usize;
}

pub type ObjectRef = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub arg_type: ObjectRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: ObjectRef,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum TypeInfo {
    Pointer {
        to_type: ObjectRef,
        depth: usize,
    },
    Function {
        arg_types: Vec<ObjectRef>,
        return_type: ObjectRef,
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
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    name: String,
    size: usize,
    alignment: usize,
    info: TypeInfo,
}

impl Object for Type {
    const ID: usize = 0;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    name: String,
    location: usize,
    arguments: Vec<Argument>,
    return_type: ObjectRef,
}

impl Object for Function {
    const ID: usize = 1;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Global {
    name: String,
    location: usize,
    global_type: ObjectRef,
}

impl Object for Global {
    const ID: usize = 2;
}
