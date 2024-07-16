use std::path::Path;

use rusqlite::{Connection, Result};
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::ir::Object;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub async fn open(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            if create_dir_all(path).await.is_err() {
                return Err(Error::DatabaseOpen);
            }
        }

        let conn = match Connection::open(path) {
            Ok(conn) => conn,
            Err(_) => return Err(Error::DatabaseOpen),
        };

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub async fn create(&self, name: &str) -> Result<(), Error> {
        let conn = self.conn.lock().await;

        let Ok(mut statment) = conn.prepare_cached(
            "CREATE TABLE IF NOT EXISTS ?1 (
                name TEXT PRIMARY KEY,
                time INTEGER,
                data JSON
            )",
        ) else {
            return Err(Error::DatabaseWrite);
        };

        if statment.execute([name]).is_err() {
            return Err(Error::DatabaseWrite)
        }

        Ok(())
    }

    pub async fn read(&self, table: &str, name: &str) -> Result<Object, Error> {
        let conn = self.conn.lock().await;

        let Ok(mut statment) = conn.prepare_cached("SELECT (name, time, data) FROM ?1 WHERE name = ?2") else {
            return Err(Error::DatabaseRead);
        };

        let Ok(result) = statment.query_row([table, name], |row| {
            let data: String = row.get(2)?;
            let Ok(info) = serde_json::from_str(&data) else {
                return rusqlite::Result::Err(rusqlite::Error::InvalidColumnType(2, "ObjectInfo".to_owned(), rusqlite::types::Type::Blob))
            };

            Ok(Object {
                name: row.get(0)?,
                time: row.get(1)?,
                info
            })
        }) else {
            return Err(Error::DatabaseRead);
        };

        Ok(result)
    }

    pub async fn write(&self, table: &str, data: Object) -> Result<(), Error> {
        let conn = self.conn.lock().await;

        let Ok(mut statment) = conn.prepare_cached(
            "BEGIN tran
            IF EXISTS (SELECT name FROM ?1 WHERE name = ?2)
            BEGIN
                UPDATE ?1 SET time = ?3, data = ?4 WHERE name = ?2;
            END
            ELSE
            BEGIN
                INSERT INTO ?1 (name, time, data) VALUES (?2, ?3, ?4)
            END
            COMMIT tran",
        ) else {
            return Err(Error::DatabaseWrite);
        };

        let Ok(data_buffer) = serde_json::to_string(&data.info) else {
            return Err(Error::Serialization)
        };

        if statment.execute((table, &data.name, data.time, &data_buffer)).is_err() {
            return Err(Error::DatabaseWrite)
        }

        Ok(())
    }
}
