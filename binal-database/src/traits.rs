use serde::{Deserialize, Serialize};

pub trait Object<T>: Serialize + for<'a> Deserialize<'a> {
    const NAME: &'static str;

    const COLUMN_NAMES: &'static [&'static str];
    const COLUMN_TYPES: &'static [&'static str];

    fn id(&self) -> &T;
}
