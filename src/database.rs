use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, Result, Row};
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::ir::Object;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub async fn open(path: &Path) -> Result<Self, Error> {
        if !path.exists() && path.parent().is_some() {
            if let Err(e) = create_dir_all(path.parent().unwrap()).await {
                return Err(Error::Io(e));
            }
        }

        let conn = match Connection::open(path) {
            Ok(conn) => conn,
            Err(e) => return Err(Error::SQLite(e)),
        };

        if let Err(e) = conn.execute(
            "CREATE TABLE IF NOT EXISTS main (
                name TEXT PRIMARY KEY,
                time INTEGER,
                data JSON
            )",
            (),
        ) {
            return Err(Error::SQLite(e));
        }

        let database = Self {
            conn: Mutex::new(conn),
        };

        Ok(database)
    }

    pub async fn read(&self, name: &str) -> Result<Object, Error> {
        let conn = self.conn.lock().await;

        let mut statment =
            match conn.prepare_cached("SELECT name, time, data FROM main WHERE name = ?1") {
                Ok(statement) => statement,
                Err(e) => return Err(Error::SQLite(e)),
            };

        let callback = |row: &Row| -> rusqlite::Result<Object> {
            let data: String = row.get(2)?;
            let json = serde_json::from_str(&data);

            let result = match json {
                Ok(data) => data,
                Err(e) => {
                    return Result::Err(rusqlite::Error::InvalidColumnType(
                        2,
                        e.to_string(),
                        rusqlite::types::Type::Blob,
                    ))
                }
            };

            Ok(result)
        };

        let result = match statment.query_row([name], callback) {
            Ok(result) => result,
            Err(e) => return Err(Error::SQLite(e)),
        };

        Ok(result)
    }

    pub async fn write(&self, name: &str, data: &Object) -> Result<(), Error> {
        let conn = self.conn.lock().await;

        let mut statment = match conn.prepare_cached(
            "INSERT INTO main(name, time, data) VALUES(?1, ?2, ?3)
            ON CONFLICT (name) DO UPDATE SET time = ?2, data = ?3",
        ) {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        let data_buffer = match serde_json::to_string(data) {
            Ok(buffer) => buffer,
            Err(e) => return Err(Error::Serde(e)),
        };

        let now = SystemTime::now();
        let Ok(timestamp) = now.duration_since(UNIX_EPOCH) else {
            return Err(Error::Timestamp);
        };

        if let Err(e) = statment.execute((name, timestamp.as_secs(), data_buffer)) {
            return Err(Error::SQLite(e));
        }

        Ok(())
    }

    pub async fn remove(&self, name: &str) -> Result<(), Error> {
        let conn = self.conn.lock().await;

        let mut statment = match conn.prepare_cached("DELETE FROM main WHERE name = ?1") {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        if let Err(e) = statment.execute([name]) {
            return Err(Error::SQLite(e));
        };

        Ok(())
    }

    pub async fn changes(&self, timestamp: usize) -> Result<Vec<(String, Object)>, Error> {
        let conn = self.conn.lock().await;

        let mut statment = match conn.prepare_cached("SELECT name, data FROM main WHERE time > ?1")
        {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        let callback = |row: &Row| -> rusqlite::Result<(String, Object)> {
            let data: String = row.get(1)?;
            let name: String = row.get(0)?;

            let object = match serde_json::from_str(&data) {
                Ok(object) => object,
                Err(e) => {
                    return Result::Err(rusqlite::Error::InvalidColumnType(
                        2,
                        e.to_string(),
                        rusqlite::types::Type::Blob,
                    ))
                }
            };

            Ok((name, object))
        };

        let result = match statment.query_map([timestamp], callback) {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        Ok(result.into_iter().filter_map(|item| item.ok()).collect())
    }
}
