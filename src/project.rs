use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use notify::{Config, PollWatcher, RecursiveMode, Watcher};

use crate::error::*;

#[derive(Debug)]
pub enum ObjectEvent {
    Deleted(String),
    Modified(String),
    Added(String),
}

struct EventIdUnwrapper {
    directory: PathBuf,
    tx: Sender<ObjectEvent>,
}

impl EventIdUnwrapper {
    pub fn new(directory: PathBuf, tx: Sender<ObjectEvent>) -> Self {
        Self { directory, tx }
    }
}

impl notify::EventHandler for EventIdUnwrapper {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        if event.is_err() {
            return;
        }

        let event = event.unwrap();

        for id in event
            .paths
            .iter()
            .map(|path| {
                path.strip_prefix(&self.directory)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>()
        {
            match event.kind {
                notify::EventKind::Any => todo!(),
                notify::EventKind::Access(_) => {}
                notify::EventKind::Create(_) => {
                    let _ = self.tx.send(ObjectEvent::Added(id));
                }
                notify::EventKind::Modify(_) => {
                    let _ = self.tx.send(ObjectEvent::Modified(id));
                }
                notify::EventKind::Remove(_) => {
                    let _ = self.tx.send(ObjectEvent::Deleted(id));
                }
                notify::EventKind::Other => todo!(),
            };
        }
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    directory: PathBuf,
}

impl Project {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(Error::FileOpen);
        }

        Ok(Self { directory: path })
    }

    pub fn create_watcher(&self) -> Result<(Receiver<ObjectEvent>, PollWatcher), Error> {
        let (tx, rx) = std::sync::mpsc::channel();

        let handler = EventIdUnwrapper::new(self.directory.clone(), tx);

        let Ok(mut watcher) = PollWatcher::new(
            handler,
            Config::default().with_poll_interval(Duration::from_secs(5)),
        ) else {
            return Err(Error::WatcherCreation);
        };

        if watcher
            .watch(&self.directory, RecursiveMode::Recursive)
            .is_err()
        {
            return Err(Error::WatcherCreation);
        }

        Ok((rx, watcher))
    }

    pub fn read(&self, id: &str) -> Result<serde_json::Value, Error> {
        let path = self.directory.join(URL_SAFE.encode(id));

        let Ok(mut object_file) = File::open(path) else {
            return Err(Error::FileOpen);
        };

        let mut data = Vec::new();
        if object_file.read_to_end(&mut data).is_err() {
            return Err(Error::FileRead);
        };

        let Ok(object) = serde_json::from_slice(data.as_slice()) else {
            return Err(Error::Deserialization);
        };

        Ok(object)
    }

    pub fn write(&self, id: &str, object: serde_json::Value) -> Result<(), Error> {
        let path = self.directory.join(URL_SAFE.encode(id));

        let Ok(mut type_file) = File::create(path) else {
            return Err(Error::FileOpen);
        };

        let Ok(data) = serde_json::to_vec(&object) else {
            return Err(Error::Serialization);
        };

        if type_file.write_all(data.as_slice()).is_err() {
            return Err(Error::FileWrite);
        }

        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), Error> {
        let path = self.directory.join(URL_SAFE.encode(id));

        if remove_file(path).is_err() {
            return Err(Error::FileDelete);
        }

        Ok(())
    }
}
