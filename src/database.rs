use std::path::PathBuf;

use git2::Repository;
use tokio::fs::{remove_file, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use crate::ir::Object;
use crate::filemap::FileMap;
use crate::error::Error;

pub struct Database {
    repo: Mutex<Repository>,
}

impl Database {
    async fn get_path(&self, id: &str) -> Result<PathBuf, Error> {
        let database = self.repo.lock().await;
        let index_path = database.path().join("index");

        let index = FileMap::open(&index_path).await?;



        todo!();
    }

    pub async fn open(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(Error::FileOpen);
        }

        let Ok(repo) = Repository::open(path) else {
            return Err(Error::FileOpen)
        };

        Ok(Self { repo: Mutex::new(repo) })
    }

    pub async fn read(&self, id: &str) -> Result<Object, Error> {
        let path = self.get_path(id).await?;

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

    pub async fn write(&self, id: &str, object: Object) -> Result<(), Error> {
        let path = self.get_path(id).await?;

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
        let path = self.get_path(id).await?;

        if remove_file(path).await.is_err() {
            return Err(Error::FileDelete);
        }

        Ok(())
    }

    pub async fn commit(&self) -> Result<(), Error> {
        Ok(())
    }
}
