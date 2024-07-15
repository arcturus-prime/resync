use std::collections::HashMap;
use std::path::{Path, PathBuf};

use git2::Repository;
use tokio::fs::{remove_file, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use crate::ir::Object;
use crate::error::Error;

pub struct Database {
    index: Mutex<HashMap<String, String>>,
    last: Mutex<String>,
    repo: Mutex<Repository>,
}

async fn parse_index(path: &Path) -> Result<HashMap<String, String>, Error> {
    let Ok(mut index_file) = File::open(path).await else {
        return Err(Error::FileOpen)
    };

    let mut index_data = Vec::new();
    if index_file.read_to_end(&mut index_data).await.is_err() {
        return Err(Error::FileRead)
    };

    return Ok(match serde_json::from_slice(index_data.as_slice()) {
        Ok(index) => index,
        Err(_) => return Err(Error::Deserialization),
    })
}

async fn update_index(path: &Path, index: &HashMap<String, String>) -> Result<(), Error> {
    let Ok(mut index_file) = File::open(path).await else {
        return Err(Error::FileOpen)
    };

    let Ok(data) = serde_json::to_vec(index) else {
        return Err(Error::Serialization);
    };

    if index_file.write(&data).await.is_err() {
        return Err(Error::FileWrite)
    }
    
    Ok(())       
}

impl Database {
    pub async fn open(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(Error::FileOpen);
        }

        let index_path = path.join("index");
        let index = if index_path.exists() {
            parse_index(&index_path).await?
        } else {
            HashMap::new()
        };

        let Ok(repo) = Repository::open(path) else {
            return Err(Error::FileOpen)
        };

        Ok(Self { repo: Mutex::new(repo), last: Mutex::new(String::from("0")), index: Mutex::new(index) })
    }

    pub async fn read(&self, id: &str) -> Result<Object, Error> {
        let index = self.index.lock().await;
        let Some(path) = index.get(id) else {
            return Err(Error::FileOpen)
        };

        let Ok(mut object_file) = File::open(self.repo.lock().await.path().join(path)).await else {
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
        let mut index = self.index.lock().await;
        let last = self.last.lock().await;
        let repo = self.repo.lock().await;

        let path = match index.get(id) {
            Some(path) => path,
            None => {
                let path = repo.path().join(last.clone());
                let path_string = path.to_string_lossy().to_string();
                let key = id.to_string();

                index.insert(key, path_string);
                index.get(key).unwrap()
            },
        };

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
        let index = self.index.lock().await;
        let Some(path) = index.get(id) else {
            return Err(Error::FileOpen)
        };

        if remove_file(path).await.is_err() {
            return Err(Error::FileDelete);
        }

        Ok(())
    }

    pub async fn commit(&self) -> Result<(), Error> {
        Ok(())
    }
}
