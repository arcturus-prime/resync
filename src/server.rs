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

use crate::{database::Database, ir::Object};
use crate::{
    error::Error,
    ir::{Function, Global, Type},
};

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
        Err(e) => return Err(Error::Io(e)),
    };

    let app = Router::new()
        .route("/type", put(push::<Type>))
        .route("/type", post(pull::<Type>))
        .route("/type", delete(remove::<Type>))
        .route("/type/:timestamp", get(changes::<Type>))
        .route("/function", put(push::<Function>))
        .route("/function", post(pull::<Function>))
        .route("/function", delete(remove::<Function>))
        .route("/function/:timestamp", get(changes::<Function>))
        .route("/global", put(push::<Global>))
        .route("/global", post(pull::<Global>))
        .route("/global", delete(remove::<Global>))
        .route("/global/:timestamp", get(changes::<Global>))
        .with_state(AppState { database });

    if let Err(e) = axum::serve(listener, app).await {
        return Err(Error::Io(e));
    };

    Ok(())
}

async fn push<T: Object>(
    State(state): State<AppState>,
    Json(body): Json<Vec<(String, T)>>,
) -> StatusCode {
    for pair in body {
        if let Err(e) = state.database.write(&pair.0, &pair.1).await {
            println!("ERRROR: {:?}", e);
            return StatusCode::UNPROCESSABLE_ENTITY;
        };
    }

    StatusCode::OK
}

async fn pull<T: Object>(
    State(state): State<AppState>,
    Json(body): Json<Vec<String>>,
) -> (StatusCode, Json<Vec<T>>) {
    let mut results = Vec::new();

    for name in body {
        let result = match state.database.read(&name).await {
            Ok(result) => result,
            Err(e) => {
                println!("ERRROR: {:?}", e);
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Vec::new()));
            }
        };

        results.push(result)
    }

    (StatusCode::OK, Json(results))
}

async fn remove<T: Object>(
    State(state): State<AppState>,
    Json(body): Json<Vec<String>>,
) -> StatusCode {
    for name in body {
        if let Err(e) = state.database.remove::<T>(&name).await {
            println!("ERRROR: {:?}", e);
            return StatusCode::UNPROCESSABLE_ENTITY;
        };
    }

    StatusCode::OK
}

async fn changes<T: Object>(
    State(state): State<AppState>,
    Query(params): Query<usize>,
) -> (StatusCode, Json<Vec<(String, T)>>) {
    let changes = match state.database.changes(params).await {
        Ok(changes) => changes,
        Err(e) => {
            println!("ERROR: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()));
        }
    };

    (StatusCode::OK, Json(changes))
}
