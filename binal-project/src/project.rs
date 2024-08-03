use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use bitcode::{Decode, Encode};
use lazy_static::lazy_static;
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::ir::{Function, Global, Type};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Bitcode(bitcode::Error),
    TypeInvalid(String),
    PathInvalid,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<bitcode::Error> for Error {
    fn from(value: bitcode::Error) -> Self {
        Self::Bitcode(value)
    }
}

lazy_static! {
    static ref PRIMITIVE_TYPES: HashSet<&'static str> = {
        HashSet::from_iter(
            vec![
                "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
                "void",
            ]
            .iter()
            .map(|s| *s),
        )
    };
}

#[derive(Debug, Encode, Decode)]
pub struct Transaction {
    pub types: HashMap<String, Type>,
    pub functions: HashMap<String, Function>,
    pub globals: HashMap<String, Global>,
}

impl Transaction {
    #[inline]
    fn check_child_type(&self, parent: &String, child: &String) -> Result<(), Error> {
        if self.types.get(child).is_none() && !PRIMITIVE_TYPES.contains(child.as_str()) {
            Err(Error::TypeInvalid(format!(
                "'{}' has invalid reference to type '{}'",
                parent, child
            )))
        } else {
            Ok(())
        }
    }

    fn validate_type<'a>(&self, pair: (&'a String, &'a Type)) -> Result<(), Error> {
        if PRIMITIVE_TYPES.contains(pair.0.as_str()) {
            return Err(Error::TypeInvalid(format!(
                "{} cannot be registered as it is a reserved type",
                pair.0
            )));
        }

        match &pair.1.info {
            crate::ir::TypeInfo::Pointer { to_type, .. } => self.check_child_type(pair.0, to_type),
            crate::ir::TypeInfo::Function {
                arg_types,
                return_type,
            } => {
                for arg in arg_types {
                    self.check_child_type(pair.0, arg)?
                }

                self.check_child_type(pair.0, return_type)
            }
            crate::ir::TypeInfo::Struct { fields } => {
                for field in fields {
                    self.check_child_type(pair.0, &field.field_type)?
                }

                Ok(())
            }
            crate::ir::TypeInfo::Enum { .. } => Ok(()),
            crate::ir::TypeInfo::Array { item_type } => self.check_child_type(pair.0, item_type),
            crate::ir::TypeInfo::None => Ok(()),
        }
    }

    pub fn validate_function(&self, pair: (&String, &Function)) -> Result<(), Error> {
        for arg in &pair.1.arguments {
            self.check_child_type(pair.0, &arg.arg_type)?;
        }

        self.check_child_type(pair.0, &pair.1.return_type)
    }

    pub fn validate_global(&self, pair: (&String, &Global)) -> Result<(), Error> {
        self.check_child_type(pair.0, &pair.1.global_type)
    }

    pub async fn merge(&mut self, transaction: Transaction) -> Result<(), Error> {
        for pair in &transaction.types {
            self.validate_type(pair)
                .or(transaction.validate_type(pair))?;
        }

        for pair in &transaction.functions {
            self.validate_function(pair)
                .or(transaction.validate_function(pair))?;
        }

        for pair in &transaction.globals {
            self.validate_global(pair)
                .or(transaction.validate_global(pair))?;
        }

        self.types.extend(transaction.types);
        self.globals.extend(transaction.globals);
        self.functions.extend(transaction.functions);

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            functions: HashMap::new(),
            globals: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Project {
    current: Transaction,
    path: PathBuf,
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        Self {
            current: Transaction::new(),
            path,
        }
    }

    pub async fn open(path: PathBuf) -> Result<Self, Error> {
        let mut project_file = File::open(&path).await?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data).await?;
        let project = Self {
            current: bitcode::decode(project_data.as_slice())?,
            path,
        };

        Ok(project)
    }

    pub async fn save(&self) -> Result<(), Error> {
        let mut transaction;

        if !self.path.exists() && self.path.parent().is_some() {
            create_dir_all(&self.path.parent().unwrap()).await?;
            transaction = File::create(&self.path).await?;
        } else {
            transaction = OpenOptions::new().write(true).open(&self.path).await?;
        }

        let data = bitcode::encode(&self.current);
        transaction.write(&data).await?;
        transaction.shutdown().await?;

        Ok(())
    }

    pub async fn process(&mut self, transaction: Transaction) -> Result<(), Error> {
        self.current.merge(transaction).await
    }
}
