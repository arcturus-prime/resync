use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::Arc,
};

use clap::{value_parser, Arg, Command};
use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};

use binal_server::{Message, Server};
use binal_project::{Project, Transaction};

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
            Message::PushType { name, data } => {
                transaction.types.insert(name, data);
            }
            Message::PushGlobal { name, data } => {
                transaction.globals.insert(name, data);
            }
            Message::PushFunction { name, data } => {
                transaction.functions.insert(name, data);
            }
            Message::DeleteType { name } => {
                transaction.types.remove(&name);
            }
            Message::DeleteGlobal { name } => {
                transaction.globals.remove(&name);
            }
            Message::DeleteFunction { name } => {
                transaction.functions.remove(&name);
            }
            Message::EndTransaction => {
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
