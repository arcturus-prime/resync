use std::marker::PhantomData;
use std::sync::Arc;
use std::time;
use std::{path::Path, str::FromStr};

use rusqlite::Connection;
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::object::{generate_create_query, generate_remove_query, generate_select_query, generate_upsert_query, Object};

#[derive(Debug, Clone)]
pub struct Database {
	connection: Arc<Mutex<Connection>>,
}

unsafe impl Sync for Database {}

#[derive(Debug, Clone)]
pub struct Table<T: Object> {
	db: Arc<Mutex<Connection>>,
	phantom: PhantomData<T>,
}

unsafe impl<T: Object> Sync for Table<T> {}

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
        	connection: Arc::new(Mutex::new(conn))
        })
	}

	pub async fn create<T: Object>(&self) -> Result<Table<T>, Error> {
		let conn = self.connection.lock().await;

		conn.execute(&generate_create_query::<T>(), ())?;

		Ok(Table::<T> {
		    db: self.connection.clone(),
		    phantom: PhantomData,
		})
	}
}

impl<T: Object> Table<T> {
	pub async fn write(&self, object: T) -> Result<(), Error> {
		let conn = self.db.lock().await;

		conn.execute(&generate_upsert_query::<T>(), object.to_row())?;

		Ok(())
	}

	pub async fn read(&self, id: &T::Index) -> Result<T, Error> {
		let conn = self.db.lock().await;

		let Ok(result) = conn.query_row(&generate_select_query::<T>(), [id], |row| {
			let row: T::Row = match row.try_into() {
			    Ok(row) => row,
			    Err(_) => return Err(rusqlite::Error::InvalidParameterCount(0, 0)),
			};
			Ok(T::from_row(row))
		}) else {
			return Err(Error::InternalMacro)
		};

		Ok(result)
	}

	pub async fn delete(&self, id: &T::Index) -> Result<(), Error> {
		let conn = self.db.lock().await;

		conn.execute(&generate_remove_query::<T>(), [id])?;

		Ok(())
	}

	pub async fn changes(&self, time: usize) -> Result<Vec<T>, Error> {
		todo!()
	}
}