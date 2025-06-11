use eframe::egui::ahash::HashSet;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{SocketAddrV4, TcpStream},
    sync::mpsc::{self, Receiver},
};

use crate::ir::{self, IdVec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub r#type: TypeRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub r#type: TypeRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionField {
    pub name: String,
    pub r#type: TypeRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Typedef {
        r#type: TypeRef,
    },
    Function {
        arg_types: Vec<TypeRef>,
        r#type: TypeRef,
    },
    Struct {
        fields: Vec<StructField>,
    },
    Enum {
        values: Vec<EnumValue>,
    },
    Union {
        fields: Vec<UnionField>,
    },
    Array {
        r#type: TypeRef,
        count: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeRef {
    Value { name: String },
    Pointer { depth: u8, name: String },
    Uint { size: u16 },
    Int { size: u16 },
    Float { size: u16 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Object {
    Type {
        size: usize,
        alignment: usize,
        info: TypeInfo,
    },
    Function {
        location: usize,
        arguments: Vec<Argument>,
        return_type: TypeRef,
    },
    Data {
        location: usize,
        r#type: TypeRef,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Message {
    Push { objects: HashMap<String, Object> },
}

pub struct Client {
    pub rx: mpsc::Receiver<Message>,
    pub tx: mpsc::Sender<Message>,

    pub lookup_types: HashMap<String, usize>,
    pub lookup_functions: HashMap<String, usize>,
    pub lookup_data: HashMap<String, usize>,
}

impl Client {
    pub fn connect(socket_addr: SocketAddrV4) -> std::io::Result<Self> {
        let stream = TcpStream::connect(socket_addr)?;

        let mut reader = BufReader::new(stream.try_clone()?);
        let mut buffer = Vec::new();

        let (tx_inside, rx_outside) = mpsc::channel();
        let (tx_outside, rx_inside): (mpsc::Sender<Message>, Receiver<Message>) = mpsc::channel();

        std::thread::spawn(move || loop {
            buffer.clear();

            let size = match reader.read_until(b'\n', &mut buffer) {
                Ok(size) => size,
                Err(e) => {
                    log::error!("Error reading from stream: {}", e);
                    return;
                }
            };

            if size == 0 {
                log::error!("Socket disconnected");
                return;
            }

            let message = match serde_json::from_slice(&buffer) {
                Ok(o) => o,
                Err(e) => {
                    log::error!("Error while deserializing message from server: {}", e);
                    continue;
                }
            };

            tx_inside.send(message).unwrap();
        });

        let mut stream = stream.try_clone()?;

        std::thread::spawn(move || loop {
            if let Ok(message) = rx_inside.recv() {
                let mut buffer = match serde_json::to_vec(&message) {
                    Ok(o) => o,
                    Err(e) => {
                        log::error!("Error while serializing message: {}", e);
                        continue;
                    }
                };

                buffer.push(b'\n');

                if let Err(_) = stream.write_all(&buffer) {
                    log::error!("Socket disconnected");
                    return;
                }
            }
        });

        Ok(Self {
            rx: rx_outside,
            tx: tx_outside,
        })
    }
}

pub fn get_net_type_from_db(db: &ir::Database, id: usize) -> HashMap<String, Object> {
    let mut map = HashMap::new();
    let mut lifted = HashSet::new();
    let mut to_lift = vec![id];

    while !to_lift.is_empty() {
        let id = to_lift.pop().unwrap();

        if lifted.contains(&id) {
            continue;
        }
        lifted.insert(id);

        let r#type = db.types.get(id);

        let lifted_type: Object;
        match r#type.info {
            ir::TypeInfo::Struct(fields) => {
                let mut lifted_fields = Vec::new();
                for field in fields {
                    lifted_fields.push(StructField {
                        name: field.name.clone(),
                        r#type: lift_type_ref(db, &field.r#type),
                        offset: field.offset,
                    });
                }

                lifted_type = Object::Type {
                    info: TypeInfo::Struct {
                        fields: lifted_fields,
                    },
                    size: r#type.size,
                    alignment: r#type.alignment,
                };
            }
            ir::TypeInfo::Array(element_type, count) => {
                let lifted_element_type = lift_type_ref(db, &element_type);

                lifted_type = Object::Type {
                    info: TypeInfo::Array {
                        r#type: lifted_element_type,
                        count,
                    },
                    size: lifted_element_type.size * count,
                };
            }
            ir::TypeInfo::Enum(enum_values) => todo!(),
            ir::TypeInfo::Union(union_members) => todo!(),
            ir::TypeInfo::TypeDef(type_ref) => todo!(),
            ir::TypeInfo::Function(type_refs, type_ref) => todo!(),
        };

        map.insert(r#type.name.clone(), lifted_type);
    }

    map
}
fn lift_type_ref(db: &ir::Database, type_ref: &ir::TypeRef) -> TypeRef {
    match type_ref {
        ir::TypeRef::Int(_) => todo!(),
        ir::TypeRef::Uint(_) => todo!(),
        ir::TypeRef::Float(_) => todo!(),
        ir::TypeRef::Value(_) => todo!(),
        ir::TypeRef::Pointer(_, _) => todo!(),
    }
}

fn reserve_object<T: Default>(
    index_lookup: &mut HashMap<String, usize>,
    objects: &mut IdVec<T>,
    name: &String,
) {
    if index_lookup.get(name).is_none() {
        let id = objects.push(T::default());
        index_lookup.insert(name.clone(), id);
    }
}

pub fn lower_net(db: &mut ir::Database, objects: HashMap<String, Object>) {
    // we need to create stubs for each object to support circular dependencies
    for (name, obj) in &objects {
        match obj {
            Object::Type { .. } => reserve_object(&mut self.type_lookup, &mut self.types, &name),
            Object::Function { .. } => {
                reserve_object(&mut self.function_lookup, &mut self.functions, &name);
            }
            Object::Data { .. } => {
                reserve_object(&mut self.data_lookup, &mut self.data, &name);
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

                db.types.get(index).name = name;
                db.types.get(index).size = size;
                db.types.get(index).alignment = alignment;

                db.types.get(index).info = match info {
                    TypeInfo::Typedef { r#type } => TypeInfo::TypeDef(db.lift_type_ref(&r#type)),
                    TypeInfo::Function { arg_types, r#type } => TypeInfo::Function(
                        arg_types.iter().map(|t| lift_type_ref(db, t)).collect(),
                        lift_type_ref(&r#type),
                    ),
                    TypeInfo::Struct { fields } => TypeInfo::Struct(
                        fields
                            .into_iter()
                            .map(|f| StructMember {
                                name: f.name,
                                offset: f.offset,
                                r#type: db.lift_type_ref(&f.r#type),
                            })
                            .collect(),
                    ),
                    TypeInfo::Enum { values } => TypeInfo::Enum(
                        values
                            .into_iter()
                            .map(|v| EnumValue {
                                name: v.name,
                                value: v.value,
                            })
                            .collect(),
                    ),
                    TypeInfo::Array { r#type, count } => {
                        TypeInfo::Array(db.lift_type_ref(&r#type), count)
                    }
                    TypeInfo::Union { fields } => TypeInfo::Union(
                        fields
                            .into_iter()
                            .map(|f| UnionMember {
                                name: f.name,
                                r#type: db.lift_type_ref(&f.r#type),
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
                let index = db.function_lookup[&name];

                db.functions.get(index).location = location;
                db.functions.get(index).name = name;
                db.functions.get(index).return_type = self.lift_type_ref(&r#type);
                db.functions.get(index).argument_types = arguments
                    .iter()
                    .map(|t| db.lift_type_ref(&t.r#type))
                    .collect();
                db.functions.get(index).argument_names =
                    arguments.iter().map(|t| t.name.clone()).collect();
            }
            Object::Data { r#type, location } => {
                let index = db.data_lookup[&name];

                db.data.get(index).location = location;
                db.data.get(index).name = name;
                db.data.get(index).r#type = self.lift_type_ref(&r#type);
            }
        }
    }
}
