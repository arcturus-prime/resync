use serde::{Deserialize, Serialize};

pub trait Object<T>: Serialize + for<'a> Deserialize<'a> {
    type Row;
    
    const NAME: &'static str;

    const ID_NAME: &'static str;
    const ID_TYPE: &'static str;

    const COLUMN_NAMES: &'static [&'static str];
    const COLUMN_TYPES: &'static [&'static str];

    fn id(&self) -> &T;

    fn from_row(row: &Self::Row) -> Self;
    fn to_row(&self) -> Self::Row;
}
