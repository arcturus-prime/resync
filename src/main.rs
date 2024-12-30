mod error;
mod ir;
mod net;

use std::net::{Ipv4Addr, SocketAddr};

use eframe::egui::{self, Ui};

use ir::{ObjectKind, Project};
use net::Client;

struct App {
    projects: Vec<Project>,
    client: Client,

    current_project: usize,
    current_tab: ObjectKind,
    current_cursor: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
        });
    }
}
                                  

fn main() -> Result<(), error::Error> {
    env_logger::init();

    let client = Client::connect(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 12007))?;
    let app = App { projects: vec![Project::new()], client, current_cursor: 0, current_tab: ObjectKind::Functions, current_project: 0 };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(app))),
    )?;

    Ok(())
}
