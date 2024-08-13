use std::net::{IpAddr, SocketAddr};

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufStream},
    net::TcpStream,
    sync::mpsc::{error::SendError, Receiver, Sender},
};

use ir::{Function, Global, Type};

#[derive(Debug)]
pub enum Error {
    Tcp(std::io::Error),
    Json(serde_json::Error),
    Mspc(SendError<Message>)
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Tcp(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<SendError<Message>> for Error {
    fn from(value: SendError<Message>) -> Self {
        Self::Mspc(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Message {
    PushType { id: usize, data: Type },
    PushGlobal { id: usize, data: Global },
    PushFunction { id: usize, data: Function },
    DeleteType { id: usize },
    DeleteGlobal { id: usize },
    DeleteFunction { id: usize },
    EndTransaction,
}

pub struct Server {
    pub rx: Receiver<Message>,
    pub tx: Sender<Message>,
}

impl Server {
    pub async fn create(address: IpAddr, port: u16) -> Result<Self, Error> {
        let (outside_tx, mut inside_rx) = tokio::sync::mpsc::channel(16);
        let (inside_tx, outside_rx) = tokio::sync::mpsc::channel(16);

        let socket_addr = SocketAddr::new(address, port);
        let listener = tokio::net::TcpListener::bind(socket_addr).await?;

        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();

                println!("Accepted connection, starting handler...");

                Self::handler(&inside_tx, &mut inside_rx, stream).await;
            }
        });

        Ok(Self {
            rx: outside_rx,
            tx: outside_tx,
        })
    }

    async fn handler(tx: &Sender<Message>, rx: &mut Receiver<Message>, stream: TcpStream) {
        let mut buffer = String::new();
        let mut stream = BufStream::new(stream);

        loop {
            tokio::select! {
                size = stream.read_line(&mut buffer) => {
                    if let Err(e) = size {
                        println!("Error reading from TCP stream: {}", e);
                        return
                    }

                    let message: Message = match serde_json::from_str(&buffer) {
                        Ok(message) => message,
                        Err(e) => {
                            println!("Failed to deserialize message: {}", e);
                            buffer.clear();
                            continue
                        },
                    };

                    if let Err(e) = tx.send(message).await {
                        println!("Error sending message through MSPC channel: {}", e);
                        buffer.clear();
                        continue;
                    }

                    buffer.clear();
                },
                Some(message) = rx.recv() => {
                    //TODO(AP): Don't make a new vector every time we serialize
                    let data = match serde_json::to_vec(&message) {
                        Ok(data) => data,
                        Err(e) => {
                            println!("Failed to serialize message: {}", e);
                            continue
                        },
                    };
                    if let Err(e) = stream.write(&data).await {
                        println!("Error writing message to TCP stream: {}", e);
                        return
                    }
                }
            }
        }
    }
}
