use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};

use crate::database::Database;
use crate::error::Error;
use crate::ir::Object;

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

    let Ok(listener) = tokio::net::TcpListener::bind(socket_addr).await else {
        return Err(Error::SocketOpen);
    };

    database.create("global");
    database.create("function");
    database.create("type");

    let app = Router::new()
        .route("/object", post(push_object))
        .route("/object", get(pull_objects))
        .route("/object", delete(delete_objects))
        .route("/changes", post(get_changes))
        .with_state(AppState { database });

    if axum::serve(listener, app).await.is_err() {
        return Err(Error::RouterInit);
    };

    Ok(())
}

async fn push_object(
    State(state): State<AppState>,
    Json(body): Json<Vec<Object>>,
) -> StatusCode {
    for object in body {
        state.database.write().await.unwrap();
    }

    StatusCode::OK
}

async fn delete_objects(State(state): State<AppState>, Json(body): Json<Vec<String>>) -> StatusCode {
    for name in body {
        state.database.delete(id).await.unwrap();
    }

    StatusCode::OK
}

async fn pull_objects(
    State(state): State<AppState>,
    Json(body): Json<Vec<String>>,
) -> (StatusCode, Json<Vec<Object>>) {
    let mut results = Vec::new();

    for id in body {
        let object = state.database.read(id).await.unwrap();

        results.push((id, object))
    }

    (StatusCode::OK, Json(results))
}

async fn get_changes(State(state): State<AppState>) -> (StatusCode, Json<Vec<()>>) {
    (StatusCode::OK, Json(Vec::new()))
}
