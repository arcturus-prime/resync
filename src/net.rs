use serde::{Deserialize, Serialize};
use std::{
    io::{self, BufRead, BufReader, Write},
    net::{SocketAddr, TcpStream},
    sync::mpsc::{self, Receiver},
};

use crate::ir::{Function, Global, Project, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    PushFunction(String, Function),
    PushGlobal(String, Global),
    PushType(String, Type),

    RenameFunction(String),
    RenameGlobal(String),
    RenameType(String),

    DeleteFunction(String),
    DeleteGlobal(String),
    DeleteType(String),
}

pub struct Client {
    pub rx: mpsc::Receiver<Message>,
    pub tx: mpsc::Sender<Message>,
}

macro_rules! rename_object {
    ( $object_type:literal, $map:expr, $name:expr ) => {{
        let Some(a) = $map.remove(&$name) else {
            log::warn!(
                "Plugin tried to rename a {} that does not exist on client: {}",
                $object_type,
                $name
            );
            continue;
        };

        $map.insert($name, a);
    }};
}

macro_rules! delete_object {
    ( $object_type:literal, $map:expr, $name:expr ) => {{
        if $map.remove(&$name).is_none() {
            log::warn!(
                "Plugin tried to delete a {} that does not exist on client: {}",
                $object_type,
                $name
            );
            continue;
        }
    }};
}

impl Client {
    pub fn connect(socket_addr: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(socket_addr)?;

        let mut reader = BufReader::new(stream.try_clone()?);
        let mut buffer = Vec::new();

        let (tx_inside, rx_outside) = mpsc::channel();
        let (tx_outside, rx_inside): (mpsc::Sender<Message>, Receiver<Message>) = mpsc::channel();

        std::thread::spawn(move || loop {
            if let Err(e) = reader.read_until(b'\n', &mut buffer) {
                log::error!("Error reading from stream: {}", e);
                continue;
            };

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
                        return;
                    }
                };

                buffer.push(b'\n');

                if let Err(e) = stream.write_all(&buffer) {
                    log::error!("Error while sending message: {}", e);
                }
            }
        });

        Ok(Self {
            rx: rx_outside,
            tx: tx_outside,
        })
    }
    pub fn update_project(&mut self, project: &mut Project) {
        loop {
            let Ok(data) = self.rx.try_recv() else {
                return;
            };

            match data {
                Message::PushFunction(name, function) => {
                    project.functions.insert(name, function);
                }
                Message::PushGlobal(name, global) => {
                    project.globals.insert(name, global);
                }
                Message::PushType(name, type_) => {
                    project.types.insert(name, type_);
                }
                Message::RenameFunction(name) => {
                    rename_object!("function", project.functions, name)
                }
                Message::RenameGlobal(name) => rename_object!("global", project.globals, name),
                Message::RenameType(name) => rename_object!("type", project.types, name),
                Message::DeleteFunction(name) => {
                    delete_object!("function", project.functions, name)
                }
                Message::DeleteGlobal(name) => delete_object!("global", project.globals, name),
                Message::DeleteType(name) => delete_object!("type", project.types, name),
            }
        }
    }
}
