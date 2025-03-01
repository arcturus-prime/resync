use serde::{Deserialize, Serialize};

use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddrV4, TcpStream, TcpListener},
    sync::{Arc, Mutex},
};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub value: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Argument {
    pub name: String,
    pub arg_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructField {
    pub name: String,
    pub offset: usize,
    pub field_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum TypeInfo {
    Pointer {
        to_type: String,
        depth: usize,
    },
    Function {
        arg_types: Vec<String>,
        return_type: String,
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
    Int,
    Uint,
    Float,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        return_type: String,
    },
    Global {
        location: usize,
        global_type: String,
    },
    Null
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Message {
    Delete {
        name: String,
    },
    Rename {
        old: String,
        new: String,
    },
    Push {
        name: String,
        object: Object,
    },
    Sync {
        names: Vec<String>,
        objects: Vec<Object>,
    },
}

pub struct Client {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    
    buffer: Vec<u8>,
}

impl Client {
    pub fn read(&mut self) -> Result<Message, Error> {
        let _ = self.reader.read_until(b'\n', &mut self.buffer)?;
        let message = serde_json::from_slice(&self.buffer)?;
    
        Ok(message)
    }

    pub fn write(&mut self, message: Message) -> Result<(), Error> {
        let mut buffer = serde_json::to_vec(&message)?;
        
        buffer.push(b'\n');

        let _ = self.writer.write(&buffer)?;
    
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        Ok(self.writer.flush()?)
    }

    pub fn new(stream: TcpStream) -> Result<Self, Error> {
        let temp = stream.try_clone()?;

        Ok(Self {
            reader: BufReader::new(stream),
            writer: BufWriter::new(temp),

            buffer: Vec::new()
        })
    }
}

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn listen(socket_addr: SocketAddrV4) -> Result<Self, Error> {
        let listener = TcpListener::bind(socket_addr)?;
        listener.set_nonblocking(true);

        Ok(Self {
            listener
        })
    }

    pub fn accept(&self) -> Result<Client, Error> {
        let Ok(stream) = self.listener.accept() else {
            return Err(Error::NoIncoming)
        };

        Client::new(stream.0)
    }
}
