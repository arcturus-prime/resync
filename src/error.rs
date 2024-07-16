#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Serialization,
    Deserialization,
    DatabaseOpen,
    SocketOpen,
    RouterInit,
    DatabaseWrite,
    DatabaseRead,
}