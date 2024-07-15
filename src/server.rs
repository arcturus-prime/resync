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
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::error::Error;
use crate::ir::Object;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBody {
    id: String,
    object: Object,
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

    let app = Router::new()
        .route("/objects", post(post_objects))
        .route("/objects", get(get_objects))
        .route("/objects", delete(delete_objects))
        .route("/changes", get(get_changes))
        .with_state(database);

    if axum::serve(listener, app).await.is_err() {
        return Err(Error::RouterInit);
    };

    Ok(())
}

async fn delete_objects(
    State(database): State<Arc<Database>>,
    Json(messages): Json<Vec<String>>,
) -> StatusCode {
    for id in messages {
        database.delete(&id).await.unwrap();
    }

    StatusCode::OK
}

async fn post_objects(
    State(database): State<Arc<Database>>,
    Json(messages): Json<Vec<ObjectBody>>,
) -> StatusCode {
    for message in messages {
        database.write(&message.id, message.object).await.unwrap();
    }

    StatusCode::OK
}

async fn get_objects(
    State(database): State<Arc<Database>>,
    Json(messages): Json<Vec<String>>,
) -> (StatusCode, Json<Vec<Object>>) {
    let mut objects = Vec::new();

    for id in messages {
        let object = database.read(&id).await.unwrap();

        objects.push(object);
    }

    (StatusCode::OK, Json(objects))
}

async fn get_changes(State(database): State<Arc<Database>>) -> (StatusCode, Json<Vec<()>>) {
    (StatusCode::OK, Json(Vec::new()))
}
