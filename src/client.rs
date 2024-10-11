use serde::{Deserialize, Serialize};
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::ir::{Function, Global, Project, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    PushFunction(String, Function),
    PushGlobal(String, Global),
    PushType(String, Type),
}

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn update_project(&mut self, project: &mut Project) {
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(&self.stream);

        if let Err(e) = reader.read_until(b'\n', &mut buffer) {
            println!("Error reading message from stream: {}", e)
        };

        let message = match serde_json::from_slice(&buffer) {
            Ok(o) => o,
            Err(e) => {
                println!("Error while decoding message from server: {}", e);
                return;
            }
        };

        self.process_message(message, project);
    }

    fn process_message(&mut self, message: Message, project: &mut Project) {
        match message {
            Message::PushFunction(name, function) => {
                project.functions.insert(name, function);
            }
            Message::PushGlobal(name, global) => {
                project.globals.insert(name, global);
            }
            Message::PushType(name, type_) => {
                project.types.insert(name, type_);
            }
        }
    }

    pub fn send_message(&mut self, message: Message) {
        let mut buffer = match serde_json::to_vec(&message) {
            Ok(o) => o,
            Err(e) => {
                println!("Error while serializing message: {}", e);
                return
            },
        };

        buffer.push(b'\n');

        if let Err(e) = self.stream.write_all(&buffer) {
            println!("Error while sending message: {}", e);
        }
    }

    pub fn connect(socket_addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(socket_addr)?,
        })
    }
}
