pub mod ir;
pub mod project;

use std::{
    fs::create_dir_all,
    io::{BufRead, BufReader, BufWriter},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    path::{self, Path, PathBuf},
    sync::mpsc::Receiver,
};

use clap::{Arg, Command};
use serde::{Deserialize, Serialize};

use project::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Message<'a> {
    Push {
        path: &'a str,
        object: serde_json::Value,
    },
    Delete {
        path: &'a str,
    },
}

pub struct ProjectServer {
    pub username: String,
    pub directory: PathBuf,
}

impl ProjectServer {
    pub fn start(&self, port: u16, address: IpAddr) {
        let listener = TcpListener::bind(SocketAddr::new(address, port)).unwrap();

        loop {
            let stream = listener.accept().unwrap().0;

            let write_stream = stream.try_clone().unwrap();
            let read_stream = stream.try_clone().unwrap();

            let write_directory = self.directory.clone();
            let read_directory = self.directory.clone();

            std::thread::spawn(move || {
                let project = Project::open(read_directory).unwrap();
                let (rx, _watcher) = project.create_watch().unwrap();

                Self::handle_changes(project, write_stream, rx);
            });

            std::thread::spawn(move || {
                let project = Project::open(write_directory).unwrap();

                Self::handle_messages(project, read_stream);
            });
        }
    }

    fn handle_messages(project: Project, stream: TcpStream) {
        let mut data = vec![0; 512 * 64];
        let mut stream = BufReader::new(stream);

        loop {
            data.clear();

            let bytes = stream.read_until(b'\n', &mut data).unwrap();
            if bytes == 0 {
                continue;
            }

            let message: Message = serde_json::from_slice(&data).unwrap();

            match message {
                Message::Push { path, object } => {
                    let path = project.directory.join(&path);
                    project.write(&path, object).unwrap();
                }
                Message::Delete { path } => {
                    let path = project.directory.join(&path);
                    project.delete(&path).unwrap();
                }
            }
        }
    }

    fn handle_changes(
        project: Project,
        stream: TcpStream,
        rx: Receiver<notify::Result<notify::Event>>,
    ) {
        for change in rx {
            if change.is_err() {
                continue;
            }

            let is_remove = change.as_ref().unwrap().kind.is_remove();

            for path in change.unwrap().paths {
                let write_stream = BufWriter::new(&stream);
                let path_str = path.strip_prefix(&project.directory).unwrap().to_str().unwrap();

                if is_remove {
                    serde_json::to_writer(write_stream, &Message::Delete { path: path_str }).unwrap();
                } else {
                    let object = project.read(&path).unwrap();

                    serde_json::to_writer(write_stream, &Message::Push { path: path_str, object })
                        .unwrap();
                }
            }
        }
    }
}

fn main() {
    let matches = Command::new("Binal")
        .version("1.0")
        .about("External client for managing Binal repository.")
        .arg(Arg::new("project").required(true))
        .arg(Arg::new("username").required(true))
        .arg(Arg::new("port").long("port").required(false))
        .get_matches();

    let project_name = matches.get_one::<String>("project").unwrap();
    let username = matches.get_one::<String>("username").unwrap();

    let path = path::Path::new(project_name);

    if !path.exists() {
        create_dir_all(path).unwrap();
    }

    let server = ProjectServer {
        username: username.clone(),
        directory: path.canonicalize().unwrap().to_path_buf(),
    };

    let port = matches.get_one::<u16>("port").unwrap_or(&12007);
    let address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    server.start(*port, address);
}
