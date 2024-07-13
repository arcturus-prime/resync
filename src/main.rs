pub mod error;
pub mod project;
pub mod server;

use std::{
    fs::create_dir_all,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    path::{self},
};

use clap::{Arg, Command};

use error::*;
use project::*;
use server::*;

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

    let listener = TcpListener::bind(SocketAddr::new(address, *port)).unwrap();

    loop {
        let (stream, _addr) = listener.accept().unwrap();

        let project = Project::open(path.to_path_buf()).unwrap();
        let server = Server::open(project, stream).unwrap();

        let (_watcher, errors) = server.spawn().unwrap();

        for error in errors {
            if error == Error::SocketClosed {
                break;
            }

            println!("Error occured: {:?}", error);
        }
    }
}
