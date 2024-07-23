#[derive(Debug)]
pub enum Error {
    Bitcode(bitcode::Error),
    Rusqlite(rusqlite::Error),
    Io(std::io::Error),
    Timestamp(&'static str),
    Path(&'static str),
    InternalMacro,
    None,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Bitcode(_) => todo!(),
            Error::Rusqlite(_) => todo!(),
            Error::Io(_) => todo!(),
            Error::Timestamp(_) => todo!(),
            Error::Path(_) => todo!(),
            Error::InternalMacro => todo!(),
            Error::None => todo!(),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
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