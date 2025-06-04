use std::{
    collections::HashMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::net::{self, Object};

#[derive(Debug)]
pub enum DatabaseError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for DatabaseError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl<'a> Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Io(e) => e.fmt(f),
            DatabaseError::Serde(e) => e.fmt(f),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct StructMember {
    name: String,
    r#type: TypeRef,
    offset: usize,
}

#[derive(Serialize, Deserialize)]
struct EnumValue {
    name: String,
    value: usize,
}

#[derive(Serialize, Deserialize)]
struct UnionMember {
    name: String,
    r#type: TypeRef,
}

#[derive(Serialize, Deserialize)]
enum TypeRef {
    Int(u16),
    Uint(u16),
    Float(u16),
    Value(usize),
    Pointer(u8, usize),
}

#[derive(Serialize, Deserialize)]
enum TypeInfo {
    Struct(Vec<StructMember>),
    Enum(Vec<EnumValue>),
    Union(Vec<UnionMember>),
    TypeDef(TypeRef),
    Function(Vec<TypeRef>, TypeRef),
    Array(TypeRef, usize),
}

#[derive(Serialize, Deserialize)]
struct Type {
    name: String,
    size: usize,
    alignment: usize,
    info: TypeInfo,
}

impl Default for Type {
    fn default() -> Self {
        Type {
            name: String::new(),
            size: 0,
            alignment: 0,
            info: TypeInfo::TypeDef(TypeRef::Uint(0)),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[repr(u8)]
enum Instruction {
    Const24,
    Const16,
    Const8,
}

#[derive(Serialize, Deserialize)]
struct Function {
    name: String,
    code: Vec<Instruction>,

    return_type: TypeRef,
    argument_names: Vec<String>,
    argument_types: Vec<TypeRef>,
}

impl Default for Function {
    fn default() -> Self {
        Function {
            name: String::new(),
            code: Vec::new(),

            return_type: TypeRef::Uint(0),
            argument_names: Vec::new(),
            argument_types: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Global {
    name: String,
    r#type: TypeRef,
}

impl Default for Global {
    fn default() -> Self {
        Global {
            name: String::new(),
            r#type: TypeRef::Uint(0),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Database {
    functions: Vec<Function>,
    types: Vec<Type>,
    data: Vec<Global>,

    function_lookup: HashMap<String, usize>,
    type_lookup: HashMap<String, usize>,
    data_lookup: HashMap<String, usize>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let db: Database = serde_json::from_slice(project_data.as_slice())?;

        Ok(db)
    }

    pub fn save(&self, path: &Path) -> Result<(), DatabaseError> {
        let mut file;

        if !path.exists() {
            file = File::create(path)?;
        } else {
            file = OpenOptions::new().write(true).open(path)?;
        }

        let data = serde_json::to_vec(&self)?;
        file.write_all(&data)?;

        Ok(())
    }

    pub fn types_len(&self) -> usize {
        self.types.len()
    }

    pub fn types_name(&self) -> impl Iterator<Item = &String> + '_ {
        self.types.iter().map(|t| &t.name)
    }

    pub fn types_get_net(&self, name: &str) -> HashMap<String, Object> {
        let mut map = HashMap::new();

        todo!();

        map
    }

    pub fn types_delete(&mut self, name: &str) {
        todo!();
    }

    pub fn functions_len(&self) -> usize {
        self.functions.len()
    }

    pub fn functions_name(&self) -> impl Iterator<Item = &String> + '_ {
        self.functions.iter().map(|t| &t.name)
    }

    pub fn functions_get_net(&self, name: &str) -> HashMap<String, Object> {
        let mut map = HashMap::new();

        todo!();

        map
    }

    pub fn functions_delete(&mut self, name: &str) {
        todo!();
    }

    pub fn globals_len(&self) -> usize {
        self.data.len()
    }

    pub fn globals_name(&self) -> impl Iterator<Item = &String> + '_ {
        self.data.iter().map(|t| &t.name)
    }

    pub fn globals_get_net(&self, name: &str) -> HashMap<String, Object> {
        let mut map = HashMap::new();

        todo!();

        map
    }

    pub fn globals_delete(&mut self, name: &str) {
        todo!();
    }

    fn lift_type_ref(&self, type_ref: &net::TypeRef) -> TypeRef {
        match type_ref {
            net::TypeRef::Value { name } => {
                let index = self.type_lookup[name];
                TypeRef::Value(index)
            }
            net::TypeRef::Pointer { depth, name } => {
                let index = self.type_lookup[name];
                TypeRef::Pointer(*depth, index)
            }
            net::TypeRef::Uint { size } => TypeRef::Uint(*size),
            net::TypeRef::Int { size } => TypeRef::Int(*size),
            net::TypeRef::Float { size } => TypeRef::Float(*size),
        }
    }

    fn reserve_object<T: Default>(
        index_lookup: &mut HashMap<String, usize>,
        objects: &mut Vec<T>,
        name: &String,
    ) {
        if index_lookup.get(name).is_none() {
            objects.push(T::default());
            index_lookup.insert(name.clone(), objects.len() - 1);
        }
    }

    pub fn push_net(&mut self, objects: HashMap<String, Object>) {
        // we need to create stubs for each object to support circular dependencies
        for (name, obj) in &objects {
            match obj {
                Object::Type { .. } => {
                    Self::reserve_object(&mut self.type_lookup, &mut self.types, &name)
                }
                Object::Function { .. } => {
                    Self::reserve_object(&mut self.function_lookup, &mut self.functions, &name);
                }
                Object::Data { .. } => {
                    Self::reserve_object(&mut self.data_lookup, &mut self.data, &name);
                }
            }
        }

        // now fill out each object
        for (name, obj) in objects {
            match obj {
                Object::Type {
                    info,
                    size,
                    alignment,
                } => {
                    let index = self.type_lookup[&name];

                    self.types[index].name = name;
                    self.types[index].size = size;
                    self.types[index].alignment = alignment;

                    self.types[index].info = match info {
                        net::TypeInfo::Typedef { r#type } => {
                            TypeInfo::TypeDef(self.lift_type_ref(&r#type))
                        }
                        net::TypeInfo::Function { arg_types, r#type } => TypeInfo::Function(
                            arg_types.iter().map(|t| self.lift_type_ref(t)).collect(),
                            self.lift_type_ref(&r#type),
                        ),
                        net::TypeInfo::Struct { fields } => TypeInfo::Struct(
                            fields
                                .into_iter()
                                .map(|f| StructMember {
                                    name: f.name,
                                    offset: f.offset,
                                    r#type: self.lift_type_ref(&f.r#type),
                                })
                                .collect(),
                        ),
                        net::TypeInfo::Enum { values } => TypeInfo::Enum(
                            values
                                .into_iter()
                                .map(|v| EnumValue {
                                    name: v.name,
                                    value: v.value,
                                })
                                .collect(),
                        ),
                        net::TypeInfo::Array { r#type, count } => {
                            TypeInfo::Array(self.lift_type_ref(&r#type), count)
                        }
                        net::TypeInfo::Union { fields } => TypeInfo::Union(
                            fields
                                .into_iter()
                                .map(|f| UnionMember {
                                    name: f.name,
                                    r#type: self.lift_type_ref(&f.r#type),
                                })
                                .collect(),
                        ),
                    };
                }
                Object::Function {
                    arguments,
                    return_type: r#type,
                    location,
                } => {
                    let index = self.function_lookup[&name];

                    self.functions[index].name = name;
                    self.functions[index].return_type = self.lift_type_ref(&r#type);
                    self.functions[index].argument_types = arguments
                        .iter()
                        .map(|t| self.lift_type_ref(&t.r#type))
                        .collect();
                    self.functions[index].argument_names =
                        arguments.iter().map(|t| t.name.clone()).collect();
                }
                Object::Data { r#type, location } => {
                    let index = self.data_lookup[&name];

                    self.data[index].name = name;
                    self.data[index].r#type = self.lift_type_ref(&r#type);
                }
            }
        }
    }
}
