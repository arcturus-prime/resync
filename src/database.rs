use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, Result, Row};
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::ir::{Object, F32, F64, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};

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
                name TEXT NOT NULL,
                kind TEXT NOT NULL,
                time INTEGER NOT NULL,
                data DATA,

                CONSTRAINT name_kind PRIMARY KEY (name, kind)
            );",
            (),
        ) {
            return Err(Error::SQLite(e));
        }

        let database = Self {
            conn: Mutex::new(conn),
        };

        Ok(database)
    }

    pub async fn read<T: Object>(&self, name: &str) -> Result<T, Error> {
        let conn = self.conn.lock().await;

        let mut statment = match conn
            .prepare_cached("SELECT name, time, data FROM main WHERE name = ?1 AND kind = ?2")
        {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        let callback = |row: &Row| -> rusqlite::Result<T> {
            let data: Vec<u8> = row.get(2)?;

            let result = match bitcode::decode(&data) {
                Ok(data) => data,
                Err(e) => {
                    return Result::Err(rusqlite::Error::InvalidColumnType(
                        3,
                        e.to_string(),
                        rusqlite::types::Type::Blob,
                    ))
                }
            };

            Ok(result)
        };

        let result = match statment.query_row([name, T::KIND], callback) {
            Ok(result) => result,
            Err(e) => return Err(Error::SQLite(e)),
        };

        Ok(result)
    }

    pub async fn write<T: Object>(&self, name: &str, data: &T) -> Result<(), Error> {
        let conn = self.conn.lock().await;

        let mut statment = match conn.prepare_cached(
            "INSERT INTO main (name, kind, time, data) VALUES(?1, ?2, ?3, ?4) ON CONFLICT (name, kind) DO UPDATE SET time = ?3, data = ?4;",
        ) {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        let data_buffer = bitcode::encode(data);

        let now = SystemTime::now();
        let Ok(timestamp) = now.duration_since(UNIX_EPOCH) else {
            return Err(Error::Timestamp);
        };

        if let Err(e) = statment.execute((name, T::KIND, timestamp.as_secs(), data_buffer)) {
            return Err(Error::SQLite(e));
        }

        Ok(())
    }

    pub async fn remove<T: Object>(&self, name: &str) -> Result<(), Error> {
        let conn = self.conn.lock().await;

        let mut statment = match conn.prepare_cached("DELETE FROM main WHERE name = ?1, kind = ?2")
        {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        if let Err(e) = statment.execute([name, T::KIND]) {
            return Err(Error::SQLite(e));
        };

        Ok(())
    }

    pub async fn changes<T: Object>(&self, timestamp: usize) -> Result<Vec<(String, T)>, Error> {
        let conn = self.conn.lock().await;

        let mut statment =
            match conn.prepare_cached("SELECT name, data FROM main WHERE time > ?1, kind = ?2") {
                Ok(statement) => statement,
                Err(e) => return Err(Error::SQLite(e)),
            };

        let callback = |row: &Row| -> rusqlite::Result<(String, T)> {
            let data: Vec<u8> = row.get(1)?;
            let name: String = row.get(0)?;

            let object = match bitcode::decode(&data) {
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

        let result = match statment.query_map((timestamp, T::KIND), callback) {
            Ok(statement) => statement,
            Err(e) => return Err(Error::SQLite(e)),
        };

        Ok(result.into_iter().filter_map(|item| item.ok()).collect())
    }
}
