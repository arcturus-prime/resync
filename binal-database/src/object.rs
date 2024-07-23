use rusqlite::{types::FromSql, Params, Row, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Column<A: ToSql + FromSql, B: Serialize + for<'a> Deserialize<'a>> {
    AsIs(A),
    Bitcode(B),
}

pub trait Object: Serialize + for<'a> Deserialize<'a> {
    type Row: Params + for<'a> TryFrom<&'a Row<'a>>;
    type Index: ToSql + FromSql;
    
    const NAME: &'static str;

    const ID_NAME: &'static str;
    const ID_TYPE: &'static str;

    const COLUMN_NAMES: &'static [&'static str];
    const COLUMN_TYPES: &'static [&'static str];

    fn id(&self) -> &Self::Index;

    fn from_row(row: Self::Row) -> Self;
    fn to_row(self) -> Self::Row;
}

#[inline(always)]
pub fn generate_create_query<T: Object>() -> String {
    String::from("deez") 
}

#[inline(always)]
pub fn generate_select_query<T: Object>() -> String {
    String::from("deez") 
}

#[inline(always)]
pub fn generate_upsert_query<T: Object>() -> String {
    String::from("deez") 
}

#[inline(always)]
pub fn generate_remove_query<T: Object>() -> String {
    String::from("deez") 
}