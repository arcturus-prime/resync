mod database;
mod error;
mod ir;
mod server;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::Path,
    sync::Arc,
};

use clap::{Arg, Command};
use tokio::fs::create_dir_all;

use database::Database;
use server::create_server;

#[tokio::main]
async fn main() {
    let matches = Command::new("Binal")
        .version("1.0")
        .about("External client for managing Binal projects `1Z.")
        .arg(Arg::new("project").required(true))
        .arg(Arg::new("port").long("port").required(false))
        .get_matches();

    let project_name = matches.get_one::<String>("project").unwrap();

    let path = Path::new(project_name).to_path_buf();

    if !path.exists() {
        create_dir_all(&path).await.unwrap();
    }

    let database = Database::open(path).await.unwrap();

    let port = *matches.get_one::<u16>("port").unwrap_or(&12007);
    let address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    create_server(address, port, Arc::new(database))
        .await
        .unwrap();
}
