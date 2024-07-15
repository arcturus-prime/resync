use std::{hash::Hash, path::Path};

use tokio::fs::File;

use crate::error::Error;

pub struct FileMap {
    file: File,
}

impl FileMap {
    pub async fn open(path: &Path) -> Result<Self, Error> {
        return match File::open(path).await {
            Ok(file) => Ok(Self { file }),
            Err(_) => Err(Error::FileOpen),
        };
    }

    pub async fn get<K: Hash, V>(&self, key: K) -> Result<V, Error> {

    }
}
