use std::{net::{IpAddr, SocketAddr}, time::Duration};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::delete,
    Json, Router,
};

use binal_database::{
    error::Error, ir::{Database, Function, Global, Object, ObjectRef, Type}, sqlite::SqliteDatabase
};

#[derive(Clone)]
pub struct AppState {
    pub database: SqliteDatabase,
}

pub async fn create_server(address: IpAddr, port: u16, database: SqliteDatabase) -> Result<(), Error> {
    let socket_addr = SocketAddr::new(address, port);
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;

    let state = AppState { database };

    let app = Router::new()
        .route("/type", delete(remove::<Type>).post(pull::<Type>).put(push::<Type>).get(changes::<Type>))
        .route("/function", delete(remove::<Function>).post(pull::<Function>).put(push::<Function>).get(changes::<Function>))
        .route("/global", delete(remove::<Global>).post(pull::<Global>).put(push::<Global>).get(changes::<Global>))
        .with_state(state);

    if let Err(e) = axum::serve(listener, app).await {
        return Err(Error::Io(e));
    };

    Ok(())
}

async fn changes<T: Object>(State(state): State<AppState>, Query(time): Query<Duration>) -> (StatusCode, Json<Vec<(ObjectRef, T)>>) {
    let results = match state.database.changes(time).await {
        Ok(results) => results,
        Err(e) => {
            println!("ERRROR: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
        },
    };

    (StatusCode::OK, Json(results))
}

async fn push<T: Object>(
    State(state): State<AppState>,
    Json(body): Json<Vec<(ObjectRef, T)>>,
) -> StatusCode {
    for (id, object) in body {
        if let Err(e) = state.database.write(id, object).await {
            println!("ERRROR: {:?}", e);
            return StatusCode::UNPROCESSABLE_ENTITY;
        };
    }

    StatusCode::OK
}

async fn pull<T: Object>(
    State(state): State<AppState>,
    Json(body): Json<Vec<ObjectRef>>,
) -> (StatusCode, Json<Vec<T>>) {
    let mut results = Vec::new();

    for id in body {
        match state.database.read(id).await {
            Ok(result) => results.push(result),
            Err(e) => {
                println!("ERRROR: {:?}", e);
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Vec::new()));
            }
        };
    }

    (StatusCode::OK, Json(results))
}

async fn remove<T: Object>(State(state): State<AppState>, Json(body): Json<Vec<ObjectRef>>) -> StatusCode {
    for id in body {
        if let Err(e) = state.database.remove::<T>(id).await {
            println!("ERRROR: {:?}", e);
            return StatusCode::UNPROCESSABLE_ENTITY;
        };
    }

    StatusCode::OK
}
