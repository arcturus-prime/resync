#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    FileOpen,
    FileRead,
    FileWrite,
    FileDelete,
    Serialization,
    Deserialization,
    SocketOpen,
    RouterInit,
}