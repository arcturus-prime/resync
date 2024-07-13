#[derive(Debug, Eq, PartialEq)]
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
    SocketClosed,
    SocketWrite,
    SocketRead,
}