
#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Bitcode(bitcode::Error),
    Axum(axum::Error),
    SQLite(rusqlite::Error),
    Io(std::io::Error),
    Timestamp
}