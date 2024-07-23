use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};

use binal_database::{database::{Database, Table}, ir::{Function, Global, Type}};
use binal_database::error::Error;

#[derive(Clone)]
pub struct AppState {
    globals: Table<Global>,
    functions: Table<Function>,
    types: Table<Type>
}

pub async fn create_server(
    address: IpAddr,
    port: u16,
    database: &Database,
) -> Result<(), Error> {
    let socket_addr = SocketAddr::new(address, port);

    let listener = match tokio::net::TcpListener::bind(socket_addr).await {
        Ok(listener) => listener,
        Err(e) => return Err(Error::Io(e)),
    };

    let globals = database.create().await?;
    let functions = database.create().await?;
    let types = database.create().await?;

    let state = AppState { types, globals, functions };

    let app = Router::new().route("/global", put(push_global)).with_state(state);

    if let Err(e) = axum::serve(listener, app).await {
        return Err(Error::Io(e));
    };

    Ok(())
}

async fn push_global(
    State(state): State<AppState>,
    Json(body): Json<Vec<Global>>,
) -> (StatusCode, Json<String>) {
    for global in body {
        if let Err(e) = state.globals.write(global).await {
            println!("ERRROR: {:?}", e);
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(e.to_string()))
        };
    }

    (StatusCode::OK, Json("".to_string()))
}

// async fn pull<T: Object>(
//     State(state): State<AppState>,
//     Json(body): Json<Vec<String>>,
// ) -> (StatusCode, Json<Vec<T>>) {
//     let mut results = Vec::new();

//     for name in body {
//         match state.database.read(&name).await {
//             Ok(result) => results.push(result),
//             Err(e) => {
//                 println!("ERRROR: {:?}", e);
//                 return (StatusCode::UNPROCESSABLE_ENTITY, Json(Vec::new()));
//             }
//         };
//     }

//     (StatusCode::OK, Json(results))
// }

// async fn remove<T: Object>(
//     State(state): State<AppState>,
//     Json(body): Json<Vec<String>>,
// ) -> StatusCode {
//     for name in body {
//         if let Err(e) = state.database.remove::<T>(&name).await {
//             println!("ERRROR: {:?}", e);
//             return StatusCode::UNPROCESSABLE_ENTITY;
//         };
//     }

//     StatusCode::OK
// }

// async fn changes<T: Object>(
//     State(state): State<AppState>,
//     Query(params): Query<usize>,
// ) -> (StatusCode, Json<Vec<(String, T)>>) {
//     let changes = match state.database.changes(params).await {
//         Ok(changes) => changes,
//         Err(e) => {
//             println!("ERROR: {:?}", e);
//             return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()));
//         }
//     };

//     (StatusCode::OK, Json(changes))
// }
