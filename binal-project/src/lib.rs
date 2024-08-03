pub mod ir;

use std::collections::HashMap;

use ir::{Function, Global, Type};

#[derive(Debug)]
pub enum Error {
    TypeNotFound,
}

#[derive(Debug)]
pub struct Project {
    pub types: HashMap<String, Type>,
    pub functions: HashMap<String, Function>,
    pub globals: HashMap<String, Global>,
}

impl Project {
    pub fn merge(&mut self, transaction: Project) -> Result<(), Error> {

        //TODO(AP): Validation of the values is required
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