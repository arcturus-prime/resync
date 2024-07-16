use std::path::Path;

use rusqlite::{Connection, Result};
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::ir::{Function, Global, Type};

pub struct Database {
    conn: Mutex<Connection>,
}

async fn init_sqlite(conn: &Connection) {
    conn.execute(
        "
        PRAGMA foreign_keys = ON;

        CREATE TABLE global (
            name TEXT PRIMARY KEY,
            location INTEGER,
            FOREIGN KEY(type) REFERENCES type(name)
        );

        CREATE TABLE function (
            name TEXT PRIMARY KEY,
            FOREIGN KEY(return_type) REFERENCES type(name), 
            blocks JSON,
            arguments JSON,
        );

        CREATE TABLE type (
            name TEXT PRIMARY KEY,
            size INTEGER,
            alignment INTEGER,
            info JSON
        );
    ",
        (),
    );
}

impl Database {
    pub async fn open(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            if create_dir_all(path).await.is_err() {
                return Err(Error::DatabaseOpen);
            }
        }

        let mut conn = match Connection::open(path) {
            Ok(conn) => conn,
            Err(_) => return Err(Error::DatabaseOpen),
        };

        if !path.exists() {
            init_sqlite(&mut conn);
        }

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

impl Database {
    pub async fn read_global(&self, name: &String) -> Result<Global, Error> {
        let conn = self.conn.lock().await;

        let Ok(mut statement) = conn.prepare("SELECT location, type FROM global WHERE name = ?1") else {
            return Err(Error::DatabaseRead)
        };

        let Ok(global) = statement.query_row([name], |row|
            Ok(Global {
                location: row.get(0)?,
                r#type: row.get(1)?
            })
        ) else {
            return Err(Error::DatabaseRead)
        };

        Ok(global)
    }

    pub async fn write_global(&self, name: &String, global: &Global) -> Result<(), Error> {
        let conn = self.conn.lock().await;

        if conn.execute(
            "INSERT INTO global (name, location, type) VALUES (?1, ?2, ?3)",
            (name, &global.location, &global.r#type),
        ).is_err() {
            return Err(Error::DatabaseWrite)
        }

        Ok(())
    }

    pub async fn delete_global(&self, name: String) -> Result<(), Error> {
        todo!()
    }

    pub async fn read_function(&self, name: String) -> Result<Function, Error> {
        todo!()
    }

    pub async fn write_function(&self, name: String, function: Function) -> Result<(), Error> {
        todo!()
    }

    pub async fn delete_function(&self, name: String) -> Result<(), Error> {
        todo!()
    }

    pub async fn read_type(&self, name: String) -> Result<Type, Error> {
        todo!()
    }

    pub async fn write_type(&self, name: String, type_: Type) -> Result<(), Error> {
        todo!()
    }

    pub async fn delete_type(&self, name: String) -> Result<(), Error> {
        todo!()
    }
}