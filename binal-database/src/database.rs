use std::marker::PhantomData;
use std::time;
use std::{path::Path, str::FromStr};

use rusqlite::Connection;
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::object::{generate_create_query, generate_upsert_query, Object};

pub struct Database {
	connection: Mutex<Connection>,
}

pub struct Table<'a, T: Object> {
	db: &'a Database,
	phantom: PhantomData<T>,
}

impl Database {
	pub async fn open(path: &Path) -> Result<Database, Error> {
		if !path.exists() {
			let Some(path_parent) = path.parent() else {
				return Err(Error::Path("Path does not exist and has no parent directory!"))
			};

			create_dir_all(path_parent).await?;
        }

        let conn = Connection::open(&path)?;

        Ok(Self {
        	connection: Mutex::new(conn)
        })
	}

	pub async fn create<'b, T: Object>(&'b self) -> Result<Table<'b, T>, Error> {
		let conn = self.connection.lock().await;

		conn.execute(&generate_create_query::<T>(), ());

		Ok(Table::<'b, T> {
		    db: &self,
		    phantom: PhantomData,
		})
	}
}

impl<'a, T: Object> Table<'a, T> {
	pub async fn write(&self, object: T) -> Result<(), Error> {
		let conn = self.db.connection.lock().await;

		conn.execute(&generate_upsert_query::<T>(), object.to_row());

		Ok(())
	}

	pub async fn read(&self, id: &T::Index) -> Result<T, Error> {
		todo!()
	}

	pub async fn delete(&self, id: &T::Index) -> Result<(), Error> {
		todo!()
	}

	pub async fn changes(&self, time: usize) -> Result<Vec<T>, Error> {
		todo!()
	}
}