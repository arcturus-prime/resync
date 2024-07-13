pub mod error;
pub mod project;

use std::{
    fs::create_dir_all,
    io::{BufRead, BufReader, BufWriter},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    path::{self, PathBuf},
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

// TODO(AP): This is good, it doesn't panic. However, we do need to provide some heads-up when any of these checks fail
fn handle_messages(project: Project, stream: TcpStream) {
    let mut data = vec![0; 512 * 64];
    let mut stream = BufReader::new(stream);

    loop {
        data.clear();

        let Ok(bytes) = stream.read_until(b'\n', &mut data) else {
            continue;
        };

        if bytes == 0 {
            continue;
        }

        let Ok(message): Result<Message, _> = serde_json::from_slice(&data) else {
            continue;
        };

        match message {
            Message::Push { path, object } => {
                let path = project.directory.join(&path);
                if project.write(&path, object).is_err() {
                    continue;
                };
            }
            Message::Delete { path } => {
                let path = project.directory.join(&path);
                if project.delete(&path).is_err() {
                    continue;
                };
            }
        }
    }
}

// TODO(AP): This sucks. What am I even looking at?
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
            let path_str = path
                .strip_prefix(&project.directory)
                .unwrap()
                .to_str()
                .unwrap();

            if is_remove {
                if serde_json::to_writer(write_stream, &Message::Delete { path: path_str }).is_err()
                {
                    continue;
                }
            } else {
                let Ok(object) = project.read(&path) else {
                    continue;
                };

                if serde_json::to_writer(
                    write_stream,
                    &Message::Push {
                        path: path_str,
                        object,
                    },
                )
                .is_err()
                {
                    continue;
                }
            }
        }
    }
}

pub fn start(directory: PathBuf, port: u16, address: IpAddr) {
    let listener = TcpListener::bind(SocketAddr::new(address, port)).unwrap();

    loop {
        let stream = listener.accept().unwrap().0;
        let stream_clone = stream.try_clone().unwrap();

        let write_directory = directory.clone();
        let read_directory = directory.clone();

        std::thread::spawn(move || {
            let project = Project::open(&read_directory).unwrap();
            let (rx, _watcher) = project.create_watch().unwrap();

            handle_changes(project, stream, rx)
        });

        std::thread::spawn(move || {
            let project = Project::open(&write_directory).unwrap();

            handle_messages(project, stream_clone)
        });
    }
}

fn main() {
    let matches = Command::new("Binal")
        .version("1.0")
        .about("External client for managing Binal repository.")
        .arg(Arg::new("project").required(true))
        .arg(Arg::new("port").long("port").required(false))
        .get_matches();

    let project_name = matches.get_one::<String>("project").unwrap();

    let path = path::Path::new(project_name);

    if !path.exists() {
        create_dir_all(path).unwrap();
    }

    let port = matches.get_one::<u16>("port").unwrap_or(&12007);
    let address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    start(path.to_path_buf(), *port, address);
}
