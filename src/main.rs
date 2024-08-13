mod ir;
mod server;
mod app;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::Arc,
};

use clap::{value_parser, Arg, Command};
use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};

use server::{Message, Server};
use ir::{Project, Transaction};

pub struct State {
    pub project: Project,
    pub working_dir: PathBuf,
}

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
        Project::open(&path).unwrap()
    } else {
        Project::new()
    }));

    tokio::spawn(process_network(server.rx, project.clone()));
    process_input(server.tx, project).await;
}

async fn process_input(sender: Sender<Message>, project: Arc<Mutex<Project>>) {
    loop {}
}

async fn process_network(mut receive: Receiver<Message>, project: Arc<Mutex<Project>>) {
    let mut transaction = Project::new();
    loop {
        let Some(message) = receive.recv().await else {
            break;
        };

        match message {
            Message::PushType { id, data } => {
                transaction.types.insert(id, data);
            }
            Message::PushGlobal { id, data } => {
                transaction.globals.insert(id, data);
            }
            Message::PushFunction { id, data } => {
                transaction.functions.insert(id, data);
            }
            Message::DeleteType { id } => {
                transaction.types.remove(&id);
            }
            Message::DeleteGlobal { id } => {
                transaction.globals.remove(&id);
            }
            Message::DeleteFunction { id } => {
                transaction.functions.remove(&id);
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
