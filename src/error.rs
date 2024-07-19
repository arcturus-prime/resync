
#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Axum(axum::Error),
    SQLite(rusqlite::Error),
    Io(std::io::Error),
    Timestamp
}