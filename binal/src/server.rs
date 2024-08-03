use std::net::{IpAddr, SocketAddr};

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufStream},
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

use binal_project::ir::{Function, Global, Type};

#[derive(Debug)]
pub enum Error {
    Tcp(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Tcp(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Message {
    PushType { name: String, data: Type },
    PushGlobal { name: String, data: Global },
    PushFunction { name: String, data: Function },
    DeleteType { name: String },
    DeleteGlobal { name: String },
    DeleteFunction { name: String },
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

                let Err(e) = Self::handler(&inside_tx, &mut inside_rx, stream).await else {
                    panic!("Handler returned without error!")
                };

                //TODO(AP): Handle errors
            }
        });

        Ok(Self {
            rx: outside_rx,
            tx: outside_tx,
        })
    }

    //TODO(AP): Error handling please
    async fn handler(tx: &Sender<Message>, rx: &mut Receiver<Message>, stream: TcpStream) -> Result<(), Error> {
        let mut buffer = String::new();
        let mut stream = BufStream::new(stream);

        loop {
            tokio::select! {
                _ = stream.read_line(&mut buffer) => {
                    let message: Message = serde_json::from_str(&buffer).unwrap();
                    tx.send(message).await.unwrap();
                    buffer.clear();
                },
                Some(message) = rx.recv() => {
                    //TODO(AP): Don't make a new vector every time we serialize
                    let data = serde_json::to_vec(&message).unwrap();
                    stream.write(&data).await.unwrap();
                }
            }
        }
    }
}
