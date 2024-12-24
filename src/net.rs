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
}
