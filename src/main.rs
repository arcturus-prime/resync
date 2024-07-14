pub mod error;
pub mod database;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path,
    sync::{mpsc::Receiver, Arc},
};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use clap::{Arg, Command};
use notify::PollWatcher;
use serde::{Deserialize, Serialize};
use tokio::{fs::create_dir_all, sync::Mutex};

use database::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectBody {
    object: serde_json::Value,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRefBody {
    id: String,
}

#[derive(Debug)]
pub struct AppState {
    pub project: Database,
    pub watcher: PollWatcher,
    pub changes: Receiver<ObjectEvent>,
}

pub type AppStateThreadSafe = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    let matches = Command::new("Binal")
        .version("1.0")
        .about("External client for managing Binal repository.")
        .arg(Arg::new("project").required(true))
        .arg(Arg::new("port").long("port").required(false))
        .get_matches();

    let project_name = matches.get_one::<String>("project").unwrap();

    let path = path::Path::new(project_name);

    if !path.exists() {
        create_dir_all(path).await.unwrap();
    }

    let port = matches.get_one::<u16>("port").unwrap_or(&12007);
    let address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let listener = tokio::net::TcpListener::bind(SocketAddr::new(address, *port))
        .await
        .unwrap();

    let project = Database::open(path.to_path_buf()).await.unwrap();
    let (watcher, changes) = project.create_watcher().await.unwrap();

    let state = AppState {
        project,
        changes,
        watcher,
    };

    let app = Router::new()
        .route("/objects", post(post_objects))
        .route("/objects", get(get_objects))
        .route("/objects", delete(delete_objects))
        .route("/changes", get(get_changes))
        .with_state(Arc::new(Mutex::new(state)));

    axum::serve(listener, app).await.unwrap();
}

async fn delete_objects(
    State(state): State<AppStateThreadSafe>,
    Json(message): Json<Vec<ObjectRefBody>>,
) -> StatusCode {
    let project = &state.lock().await.project;

    for object_ref in message {
        project.delete(&object_ref.id).await.unwrap();
    }

    StatusCode::OK
}

async fn post_objects(
    State(state): State<AppStateThreadSafe>,
    Json(messages): Json<Vec<ObjectBody>>,
) -> StatusCode {
    println!("ADD OBJECTS");

    let project = &state.lock().await.project;

    for message in messages {
        project.write(&message.id, message.object).await.unwrap();
    }

    StatusCode::OK
}

async fn get_objects(
    State(state): State<AppStateThreadSafe>,
    Json(message): Json<Vec<ObjectRefBody>>,
) -> (StatusCode, Json<Vec<ObjectBody>>) {
    let project = &state.lock().await.project;
    let mut objects = Vec::new();

    for object_ref in message {
        let object = project.read(&object_ref.id).await.unwrap();

        objects.push(ObjectBody { id: object_ref.id, object });
    }

    (StatusCode::OK, Json(objects))
}

async fn get_changes(
    State(state): State<AppStateThreadSafe>,
) -> (StatusCode, Json<Vec<ObjectEvent>>) {
    let state = &state.lock().await;
    let watcher = &state.watcher;
    let changes = &state.changes;

    watcher.poll().unwrap();

    (StatusCode::OK, Json(changes.iter().collect()))
}