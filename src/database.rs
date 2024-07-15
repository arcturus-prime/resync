use std::path::{Path, PathBuf};

use git2::Repository;
use tokio::fs::{create_dir_all, remove_file, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use crate::error::Error;
use crate::ir::{Object, ObjectRef};

pub struct Database {
    path: PathBuf,
    repo: Mutex<Repository>,
}

impl Database {
    pub async fn open(path: &Path) -> Result<Self, Error> {
        let repo;

        if !path.exists() {
            if create_dir_all(path).await.is_err() {
                return Err(Error::FileWrite);
            }
            repo = match Repository::init(path) {
                Ok(repo) => repo,
                Err(_) => return Err(Error::FileWrite),
            };
        } else {
            repo = match Repository::open(path) {
                Ok(repo) => repo,
                Err(_) => return Err(Error::FileOpen),
            }
        }

        Ok(Self {
            path: path.to_path_buf(),
            repo: Mutex::new(repo),
        })
    }

    pub async fn read(&self, id: ObjectRef) -> Result<Object, Error> {
        let Ok(mut object_file) = File::open(self.path.join(id.to_string())).await else {
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

    pub async fn write(&self, id: ObjectRef, object: Object) -> Result<(), Error> {
        let Ok(mut type_file) = File::create(self.path.join(id.to_string())).await else {
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

    pub async fn delete(&self, id: ObjectRef) -> Result<(), Error> {
        if remove_file(self.path.join(id.to_string())).await.is_err() {
            return Err(Error::FileDelete);
        }

        Ok(())
    }

    pub async fn commit(&self) -> Result<(), Error> {
        Ok(())
    }
}
