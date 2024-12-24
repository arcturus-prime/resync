mod error;
mod ir;
mod net;

use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use eframe::egui;

use ir::Project;
use net::{Client, Message};

struct App {
    plugin_project: Project,
    plugin_comm: Client,

    file_projects: Vec<Project>,
}

macro_rules! rename_object {
    ( $object_type:literal, $map:expr, $name:expr ) => {{
        let Some(a) = $map.remove(&$name) else {
            log::warn!(
                "Plugin tried to rename a {} that does not exist on client: {}",
                $object_type,
                $name
            );
            continue;
        };

        $map.insert($name, a);
    }};
}

macro_rules! delete_object {
    ( $object_type:literal, $map:expr, $name:expr ) => {{
        if $map.remove(&$name).is_none() {
            log::warn!(
                "Plugin tried to delete a {} that does not exist on client: {}",
                $object_type,
                $name
            );
            continue;
        }
    }};
}

impl App {
    pub fn process_network_updates(&mut self) {
        loop {
            let Ok(data) = self.plugin_comm.rx.try_recv() else {
                return;
            };

            match data {
                Message::PushFunction(name, function) => {
                    self.plugin_project.functions.insert(name, function);
                }
                Message::PushGlobal(name, global) => {
                    self.plugin_project.globals.insert(name, global);
                }
                Message::PushType(name, type_) => {
                    self.plugin_project.types.insert(name, type_);
                }
                Message::RenameFunction(name) => {
                    rename_object!("function", self.plugin_project.functions, name)
                }
                Message::RenameGlobal(name) => rename_object!("global", self.plugin_project.globals, name),
                Message::RenameType(name) => rename_object!("type", self.plugin_project.types, name),
                Message::DeleteFunction(name) => {
                    delete_object!("function", self.plugin_project.functions, name)
                }
                Message::DeleteGlobal(name) => delete_object!("global", self.plugin_project.globals, name),
                Message::DeleteType(name) => delete_object!("type", self.plugin_project.types, name),
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.process_network_updates();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {

            })
        });
    }
}

fn main() -> Result<(), error::Error> {
    env_logger::init();

    let client = Client::connect(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 12007))?;
    let app = App { plugin_project: Project::new(), plugin_comm: client, file_projects: Vec::new() };

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
