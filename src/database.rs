use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

use base32::{Alphabet, decode, encode};
use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tokio::fs::{remove_file, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::error::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum ObjectEvent {
    Deleted(String),
    Added(String),
}

pub struct NotifyEventUnwrapper {
    directory: PathBuf,
    tx: Sender<ObjectEvent>,
}

impl NotifyEventUnwrapper {
    pub fn new(directory: PathBuf, tx: Sender<ObjectEvent>) -> Self {
        Self { directory, tx }
    }
}

impl notify::EventHandler for NotifyEventUnwrapper {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        let Ok(event) = event else {
            return;
        };

        for path in event.paths {
            let path_str = path
                .strip_prefix(&self.directory)
                .unwrap()
                .to_str()
                .unwrap();

            
            let decoded_path_str = decode(Alphabet::Crockford, path_str).unwrap();
            let decoded_path = String::from_utf8(decoded_path_str).unwrap();

            self.tx.send(match event.kind {
                notify::EventKind::Any
                | notify::EventKind::Other
                | notify::EventKind::Access(_) => todo!(),
                notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                    ObjectEvent::Added(decoded_path)
                }
                notify::EventKind::Remove(_) => ObjectEvent::Deleted(decoded_path),
            }).unwrap();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    directory: PathBuf,
}

impl Database {
    pub async fn open(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(Error::FileOpen);
        }

        Ok(Self { directory: path })
    }

    pub async fn create_watcher(&self) -> Result<(PollWatcher, Receiver<ObjectEvent>), Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        let handler = NotifyEventUnwrapper::new(self.directory.clone(), tx);

        let Ok(mut watcher) = PollWatcher::new(handler, Config::default().with_manual_polling())
        else {
            return Err(Error::WatcherCreation);
        };

        if watcher
            .watch(&self.directory, RecursiveMode::Recursive)
            .is_err()
        {
            return Err(Error::WatcherCreation);
        }

        Ok((watcher, rx))
    }

    pub async fn read(&self, id: &str) -> Result<serde_json::Value, Error> {
        let path = self.directory.join(&encode(Alphabet::Crockford, id.as_bytes())[0..260]);

        let Ok(mut object_file) = File::open(path).await else {
            return Err(Error::FileOpen);
        };

        let mut data = Vec::new();
        if object_file.read_to_end(&mut data).await.is_err() {
            return Err(Error::FileRead);
        };

        let Ok(object) = serde_json::from_slice(data.as_slice()) else {
            return Err(Error::Deserialization);
        };

        Ok(object)
    }

    pub async fn write(&self, id: &str, object: serde_json::Value) -> Result<(), Error> {
        let path = self.directory.join(&encode(Alphabet::Crockford, id.as_bytes())[0..260]);

        let Ok(mut type_file) = File::create(&path).await else {
            return Err(Error::FileOpen);
        };

        let Ok(data) = serde_json::to_vec(&object) else {
            return Err(Error::Serialization);
        };

        if type_file.write_all(data.as_slice()).await.is_err() {
            return Err(Error::FileWrite);
        }

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = self.directory.join(&encode(Alphabet::Crockford, id.as_bytes())[0..260]);

        if remove_file(path).await.is_err() {
            return Err(Error::FileDelete);
        }

        Ok(())
    }
}
