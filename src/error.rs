#[derive(Debug)]
pub enum Error {
    InvalidPath,
    FileOpen,
    FileRead,
    FileWrite,
    FileDelete,
    WatcherCreation,
    Serialization,
    Deserialization,
}
