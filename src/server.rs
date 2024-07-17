use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::error::Error;
use crate::{database::Database, ir::Object};

#[derive(Clone)]
pub struct AppState {
    database: Arc<Database>,
}

pub async fn create_server(
    address: IpAddr,
    port: u16,
    database: Arc<Database>,
) -> Result<(), Error> {
    let socket_addr = SocketAddr::new(address, port);

    let listener = match tokio::net::TcpListener::bind(socket_addr).await {
        Ok(listener) => listener,
        Err(e) => return Err(Error::Io(e))
    };

    let app = Router::new()
        .route("/", put(push))
        .route("/", get(pull))
        .route("/", delete(remove))
        .route("/:timestamp", post(changes))
        .with_state(AppState { database });

    if let Err(e) = axum::serve(listener, app).await {
        return Err(Error::Io(e));
    };

    Ok(())
}

async fn push(
    State(state): State<AppState>,
    Json(body): Json<Vec<(String, Object)>>,
) -> StatusCode {
    for pair in body {
        if let Err(e) = state.database.write(&pair.0, &pair.1).await {
            println!("{:?}", e);
            return StatusCode::UNPROCESSABLE_ENTITY;
        };
    }

    StatusCode::OK
}

async fn pull(
    State(state): State<AppState>,
    Json(body): Json<Vec<String>>,
) -> (StatusCode, Json<Vec<Object>>) {
    let mut results = Vec::new();

    for name in body {
        let Ok(result) = state.database.read(&name).await else {
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(Vec::new()));
        };

        results.push(result)
    }

    (StatusCode::OK, Json(results))
}

async fn remove(
    State(state): State<AppState>,
    Json(body): Json<Vec<String>>,
) -> StatusCode {
    for name in body {
        let Ok(_) = state.database.remove(&name).await else {
            return StatusCode::UNPROCESSABLE_ENTITY;
        };
    }

    StatusCode::OK
}

async fn changes(
    State(state): State<AppState>,
    Query(params): Query<usize>,
) -> (StatusCode, Json<Vec<(String, Object)>>) {
    let Ok(changes) = state.database.changes(params).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()));
    };

    (StatusCode::OK, Json(changes))
}
