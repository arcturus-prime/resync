pub mod server;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::Arc,
};

use clap::{value_parser, Arg, Command};
use server::{Message, Server};

use binal_project::project::{Project, Transaction};
use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};

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

    let server = Server::create(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
        .await
        .unwrap();

    let project = Arc::new(Mutex::new(if path.exists() {
        Project::open(path.clone()).await.unwrap()
    } else {
        Project::new(path.clone())
    }));

    tokio::spawn(process_network(server.rx, project.clone()));
    process_input(server.tx, project).await;
}

async fn process_input(sender: Sender<Message>, project: Arc<Mutex<Project>>) {
    loop {}
}

async fn process_network(mut receive: Receiver<Message>, project: Arc<Mutex<Project>>) {
    let mut transaction = Transaction::new();
    loop {
        let Some(message) = receive.recv().await else {
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
            }
            server::Message::DeleteType { name } => {
                transaction.types.remove(&name);
            }
            server::Message::DeleteGlobal { name } => {
                transaction.globals.remove(&name);
            }
            server::Message::DeleteFunction { name } => {
                transaction.functions.remove(&name);
            }
            server::Message::EndTransaction => {
                let mut project = project.lock().await;

                if let Err(e) = project.process(transaction).await {
                    println!("Error while processing transaction: {:?}", e);
                } else {
                    project.save().await.unwrap();
                }

                transaction = Transaction::new();
            }
        };
    }
}
