use crate::error::Error;
use crate::ir::{Database, Object, ObjectRef};

use tokio::{
    fs::{create_dir_all, File},
    io::BufStream,
};

pub struct FilestoreDatabase {
    stream: BufStream<File>,
}

impl Database for FilestoreDatabase {
    async fn open(path: &std::path::Path) -> Result<Self, Error> {
        if !path.exists() {
            let Some(path_parent) = path.parent() else {
                return Err(Error::Path(
                    "Path does not exist and has no parent directory!",
                ));
            };

            create_dir_all(path_parent).await?;
        }

        let file = File::open(path).await?;
        let stream = BufStream::new(file);

        Ok(Self { stream })
    }

    async fn write<T: Object>(&self, id: ObjectRef, data: T) -> Result<(), Error> {
        todo!()
    }

    async fn read<T: Object>(&self, id: ObjectRef) -> Result<T, Error> {
        todo!()
    }

    async fn remove<T: Object>(&self, id: ObjectRef) -> Result<(), Error> {
        todo!()
    }

    async fn changes<T: Object>(
        &self,
        since: std::time::Duration,
    ) -> Result<Vec<(crate::ir::ObjectRef, T)>, Error> {
        todo!()
    }
}
