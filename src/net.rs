use serde::{Deserialize, Serialize};
use std::{
    io::{self, BufRead, BufReader, Write},
    net::{SocketAddrV4, TcpStream},
    sync::mpsc::{self, Receiver},
};

use crate::ir::{Object, Project};

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
    }
}

pub struct Client {
    pub rx: mpsc::Receiver<Message>,
    pub tx: mpsc::Sender<Message>,
}

impl Client {
    pub fn connect(socket_addr: SocketAddrV4) -> io::Result<Self> {
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
        let Ok(message) = self.rx.try_recv() else {
            return;
        };

        match message {
            Message::Delete { name } => {
            }
            Message::Rename { old, new } => {

            }
            Message::Push { name, object } => {
            }
            Message::Sync { names, objects } => {
                project.data.names = names;
                project.data.objects = objects;
            }
        }
    }
}
