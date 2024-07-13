use std::fs::{create_dir_all, remove_file, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::time::Duration;

use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use path_absolutize::*;

#[derive(Debug)]
pub enum Error {
    InvalidPath,
    FileOpen,
    FileRead,
    FileWrite,
    FileDelete,
    WatcherCreation,
    Serialization,
    Deserialization,
}

pub struct Project {
    pub directory: PathBuf,
}

impl Project {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(Error::FileOpen)
        }

        Ok(Self { directory: path })
    }

    pub fn create_watch(&self) -> Result<(Receiver<notify::Result<notify::Event>>, PollWatcher), Error> {
        let (tx, rx) = std::sync::mpsc::channel();

        let Ok(mut watcher) = PollWatcher::new(tx, Config::default().with_poll_interval(Duration::from_secs(2))) else {
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

    pub fn validate_path(&self, path: &Path) -> Result<PathBuf, Error> {
        let Ok(path) = path.absolutize() else {
            return Err(Error::InvalidPath)
        };

        if path.strip_prefix(&self.directory).is_err() {
            return Err(Error::InvalidPath)
        }

        Ok(path.to_path_buf())
    }

    pub fn read(&self, path: &Path) -> Result<serde_json::Value, Error> {
        self.validate_path(path)?;

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

    pub fn write(&self, path: &Path, object: serde_json::Value) -> Result<(), Error> {
        self.validate_path(path)?;

        if !path.exists() && create_dir_all(path.parent().unwrap()).is_err() {
            return Err(Error::FileOpen);
        }

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

    pub fn delete(&self, path: &Path) -> Result<(), Error> {
        self.validate_path(path)?;

        if remove_file(path).is_err() {
            return Err(Error::FileDelete)
        }

        Ok(())
    }
}
