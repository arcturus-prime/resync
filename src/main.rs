mod ir;
mod server;
mod ui;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::Arc,
};

use clap::{value_parser, Arg, Command};
use tokio::sync::{
    mpsc::Sender,
    Mutex,
};

use ir::Project;
use server::Server;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let matches = Command::new("Binal")
        .version("0.1.0")
        .about("External client for managing Binal projects")
        .arg(
            Arg::new("project")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .required(false)
                .value_parser(value_parser!(u16)),
        )
        .get_matches();

    let path = matches.get_one::<PathBuf>("project").unwrap();
    let port = *matches.get_one("port").unwrap_or(&12007);

    let project = Arc::new(Mutex::new(if path.exists() {
        Project::open(path).unwrap()
    } else {
        Project::new()
    })); 

    let mut server = Server::create(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
        .await
        .unwrap();

    server.process(project).await;
}