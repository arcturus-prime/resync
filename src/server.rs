use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use notify::PollWatcher;
use serde::{Deserialize, Serialize};

use crate::error::*;
use crate::project::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Message {
    Push {
        id: String,
        object: serde_json::Value,
    },
    Delete {
        id: String,
    },
}

fn message_loop(project: Project, stream: TcpStream, error_stream: Sender<Error>) {
    let mut reader = BufReader::new(stream);
    let mut data = vec![0; 512 * 64];

    loop {
        data.clear();

        println!("NEW MESSAGE ATTEMPT");

        let bytes = match reader.read_until(0x0A, &mut data) {
            Ok(bytes) => bytes,
            Err(_) => {
                let _ = error_stream.send(Error::SocketClosed);
                return;
            }
        };

        if bytes == 0 {
            let _ = error_stream.send(Error::SocketClosed);
            return;
        }

        println!("NEW MESSAGE");

        let Ok(message): Result<Message, _> = serde_json::from_slice(data.as_slice()) else {
            let _ = error_stream.send(Error::Deserialization);
            continue;
        };

        if let Err(e) = match message {
            Message::Push { id, object } => project.write(&id, object),
            Message::Delete { id } => project.delete(&id),
        } {
            let _ = error_stream.send(e);
            continue;
        }
    }
}

fn changes_loop(
    project: Project,
    mut stream: TcpStream,
    changes: Receiver<ObjectEvent>,
    error_stream: Sender<Error>,
) {
    let mut data = vec![0; 512 * 64];

    for event in changes {
        println!("NEW CHANGE");

        data.clear();
        let mut writer = BufWriter::new(&mut data);

        if let Err(_e) = match event {
            ObjectEvent::Deleted(id) => {
                serde_json::to_writer(&mut writer, &Message::Delete { id: id })
            }
            ObjectEvent::Added(id) => {
                let object = match project.read(&id) {
                    Ok(object) => object,
                    Err(e) => {
                        let _ = error_stream.send(e);
                        continue;
                    }
                };
                serde_json::to_writer(&mut writer, &Message::Push { id: id, object })
            }
        } {
            let _ = error_stream.send(Error::Serialization);
        }

        if let Err(_e) = writer.write(&[0x0A]) {
            let _ = error_stream.send(Error::Serialization);
        }

        if let Err(_e) = stream.write(writer.buffer()) {
            let _ = error_stream.send(Error::SocketClosed);

            return;
        };
    }
}

pub struct Server {
    project: Project,
    stream: TcpStream,
}

impl Server {
    pub fn open(project: Project, stream: TcpStream) -> Result<Self, Error> {
        Ok(Self { project, stream })
    }

    pub fn spawn(self) -> Result<(PollWatcher, Receiver<Error>), Error> {
        let project_clone = self.project.clone();
        let Ok(stream_clone) = self.stream.try_clone() else {
            return Err(Error::SocketClosed);
        };

        let (tx, rx) = std::sync::mpsc::channel();
        let tx_clone = tx.clone();

        let Ok((changes, watcher)) = self.project.create_watcher() else {
            return Err(Error::WatcherCreation);
        };

        std::thread::spawn(move || message_loop(project_clone, stream_clone, tx_clone));
        std::thread::spawn(move || changes_loop(self.project, self.stream, changes, tx));

        Ok((watcher, rx))
    }
}
