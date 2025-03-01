use std::fmt::Display;


#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Eframe(eframe::Error),
    NoIncoming,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<eframe::Error> for Error {
    fn from(value: eframe::Error) -> Self {
        Self::Eframe(value)
    }
}

impl<'a> Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Serde(e) => e.fmt(f),
            Error::Eframe(e) => e.fmt(f),
            Error::NoIncoming => f.write_str("NoIncoming"),
        }
    }
}
