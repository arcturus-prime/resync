#[derive(Debug)]
pub enum Error {
    Bitcode(bitcode::Error),
    SQLx(sqlx::Error),
    Io(std::io::Error),
    Timestamp(&'static str),
    Path(&'static str),
    None,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::SQLx(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<bitcode::Error> for Error {
    fn from(value: bitcode::Error) -> Self {
        Self::Bitcode(value)
    }
}