pub mod server;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use clap::{value_parser, Arg, Command};
use server::Server;
use tokio::sync::Mutex;

use binal_project::Project;

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

    // let path = matches.get_one::<PathBuf>("project").unwrap();

    let port = *matches.get_one("port").unwrap_or(&12007);

    let mut server = Server::create(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port).await.unwrap();
    let mut project = Project::new();
    let mut transaction = Project::new();

    loop {
        let Some(message) = server.rx.recv().await else {
            break;
        };

        match message {
            server::Message::PushType { name, data } => {
                transaction.types.insert(name, data);
            }
            server::Message::PushGlobal { name, data } => {
                transaction.globals.insert(name, data);
            }
            server::Message::PushFunction { name, data } => {
                transaction.functions.insert(name, data);
            },
            server::Message::DeleteType { name } => todo!(),
            server::Message::DeleteGlobal { name } => todo!(),
            server::Message::DeleteFunction { name } => todo!(),
            server::Message::EndTransaction => {
                project.merge(transaction).unwrap();
                transaction = Project::new();
            }
        };
    }
}
