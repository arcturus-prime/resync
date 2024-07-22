use std::marker::PhantomData;
use std::time;
use std::{path::Path, str::FromStr};

use sqlx::{Encode, IntoArguments, Sqlite, SqlitePool};
use tokio::fs::create_dir_all;

use crate::error::Error;
use crate::traits::Object;

pub struct Database {
	pool: SqlitePool,
}

pub struct Table<'a, I, T: Object<I>> {
	db: &'a Database,
	phantom: PhantomData<T>,
	phantom_2: PhantomData<I>,
}

impl Database {
	pub async fn open(path: &Path) -> Result<Database, Error> {
		if !path.exists() {
			let Some(path_parent) = path.parent() else {
				return Err(Error::Path("Path does not exist and has no parent directory!"))
			};

			create_dir_all(path_parent).await?;
        }

        let Some(path_string) = path.to_str() else {
        	return Err(Error::Path("Path could not be converted to a string!"))
        };

        let mut db_string = String::from_str("sqlite://").unwrap();
        db_string.push_str(path_string);
        db_string.push_str("?mode=\"rwc\"");

        let pool = SqlitePool::connect(&db_string).await?;

        Ok(Self {
        	pool
        })
	}

	pub async fn create<'b, I, T: Object<I>>(&'b self) -> Result<Table<'b, I, T>, Error> {
		let mut conn = self.pool.acquire().await?;

		sqlx::query(&format!("CREATE TABLE IF NOT EXISTS {} (
		    id TEXT NOT NULL PRIMARY KEY,
		    time BIGINT NOT NULL,
		    data DATA,
		);", T::NAME)).execute(&mut *conn).await?;

		Ok(Table::<'b, I, T> {
		    db: &self,
		    phantom: PhantomData,
    		phantom_2: PhantomData,
		})
	}
}

impl<'a, I: Sync + sqlx::Type<Sqlite> + for<'b> sqlx::Encode<'b, Sqlite>, T: Object<I>> Table<'a, I, T> {
	pub async fn write(&self, object: &T) -> Result<(), Error> {
		let mut conn = self.db.pool.acquire().await?;

		let query_string = format!("INSERT INTO {} (id, time, data) VALUES(?1, ?2, ?3) ON CONFLICT (id) DO UPDATE SET time = ?2, data = ?3;", T::NAME);
		let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
		let data = bitcode::serialize(object).unwrap();

		sqlx::query(&query_string).bind(object.id()).bind(time.as_secs() as u32).bind(data).execute(&mut *conn);

		Ok(())
	}

	// pub async fn read(&self, id: &I) -> Result<T, Error> {

	// }

	// pub async fn delete(&self, id: &I) -> Result<(), Error> {
		
	// }

	// pub async fn changes(&self, time: usize) -> Result<Vec<T>, Error> {
		
	// }
}