use std::path::Path;
use std::sync::Arc;

use rusqlite::Connection;
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::ir::{Database, Object, ObjectRef};

#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    connection: Arc<Mutex<Connection>>,
}

unsafe impl Sync for SqliteDatabase {}

impl Database for SqliteDatabase {
     async fn open(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            let Some(path_parent) = path.parent() else {
                return Err(Error::Path(
                    "Path does not exist and has no parent directory!",
                ));
            };

            create_dir_all(path_parent).await?;
        }

        let conn = Connection::open(&path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS main (
        	id TEXT NOT NULL,
        	kind TEXT NOT NULL,
            time INTEGER NOT NULL,
        	data BLOB NOT NULL,

            CONSTRAINT name_kind PRIMARY KEY (id, kind)
        )",
            (),
        )?;

        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    async fn write<T: Object>(&self, id: ObjectRef, data: T) -> Result<(), Error> {
        let conn = self.connection.lock().await;

        let data = bitcode::serialize(&data)?;
        conn.execute(
            "INSERT INTO main (id, kind, data) VALUES(?1, ?2, ?3) ON CONFLICT (id, kind) DO UPDATE SET data = ?3",
            (id, T::ID, data),
        )?;

        Ok(())
    }

    async fn read<T: Object>(&self, id: ObjectRef) -> Result<T, Error> {
        let conn = self.connection.lock().await;

        let data: Vec<u8> = conn.query_row(
            "SELECT data FROM main WHERE id = ?1, kind = ?2",
            [id, T::ID],
            |row| row.get(0),
        )?;

        let data = bitcode::deserialize(&data)?;

        Ok(data)
    }

    async fn remove<T: Object>(&self, id: ObjectRef) -> Result<(), Error> {
        let conn = self.connection.lock().await;

        conn.execute("DELETE FROM main WHERE id = ?1, kind = ?2", [id, T::ID])?;

        Ok(())
    }

    async fn changes<T: Object>(&self, since: std::time::Duration) -> Result<Vec<(ObjectRef, T)>, Error> {
        let conn = self.connection.lock().await;

        let mut stat = conn.prepare("SELECT id, data FROM main WHERE time > ?1")?;

        let results = stat.query_map([since.as_secs()], |row| {
            let id = row.get(0)?;
            let data: Vec<u8> = row.get(1)?;
            let data: T = match bitcode::deserialize(&data) {
                Ok(data) => data,
                Err(_) => return Err(rusqlite::Error::InvalidColumnType(3, "Cannot deserialize object".to_string(), rusqlite::types::Type::Blob))
            };

            Ok((id, data))
        })?.map(|result| result.unwrap()).collect();

        Ok(results)
    }
}
