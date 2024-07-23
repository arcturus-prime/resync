mod server;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::Path,
    sync::Arc,
};

use clap::{value_parser, Arg, Command};

use database::Database;
use server::create_server;

#[tokio::main]
async fn main() {
    let matches = Command::new("Binal")
        .version("1.0")
        .about("External client for managing Binal projects")
        .arg(Arg::new("project").required(true))
        .arg(Arg::new("port").long("port").required(false).value_parser(value_parser!(u16)))
        .get_matches();

    let project_name = matches.get_one::<String>("project").unwrap();

    let path = Path::new(project_name).to_path_buf();
    let database = Arc::new(Database::open(&path).await.unwrap());

    let port = *matches.get_one("port").unwrap_or(&12007);
    let address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    create_server(address, port, database).await.unwrap();
}
