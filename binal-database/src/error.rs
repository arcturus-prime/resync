#[derive(Debug)]
pub enum Error {
    Bitcode(bitcode::Error),
    Rusqlite(rusqlite::Error),
    Io(std::io::Error),
    Timestamp(&'static str),
    Path(&'static str),
    None,
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self::Rusqlite(value)
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