#[derive(Debug)]
pub enum Error {
    InvalidPath,
    FileOpen,
    FileRead,
    FileWrite,
    FileDelete,
    WatcherCreation,
    WatcherPoll,
    Serialization,
    Deserialization,
    SocketFailure,
}