mod error;
mod ir;
mod net;
mod ui;

use std::{net::{Ipv4Addr, SocketAddrV4}, str::FromStr};

use eframe::egui;
use net::Client;
use rfd::FileDialog;

use ir::{ObjectKind, Project};
use ui::{ProjectKind, ProjectMenu};


#[derive(PartialEq)]
enum Focus {
    Open,
    View,
}

struct App {
    tabs: Vec<ProjectMenu>,
    current: usize,

    ip_text: String,
    port_text: String,

    focus: Focus,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tabs: vec![],
            current: 0,

            ip_text: String::new(),
            port_text: String::new(),

            focus: Focus::View,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            for (tab, i) in self.tabs.iter().zip(0..) {
                if ui.button(&tab.name).clicked() {
                    self.current = i;
                }
            }

            if ui.button("+").clicked() {
                self.focus = Focus::Open
            }
        });

        if self.focus == Focus::Open {
            egui::Window::new("Open Project")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false )
                .show(ctx, |ui| {
                    if ui.button("Open with file").clicked() {
                        self.focus = Focus::View;

                        let Some(file) = FileDialog::new().pick_file() else {
                            return;
                        };

                        let Ok(project) = Project::open(&file) else {
                            return;
                        };

                        self.tabs.push(ProjectMenu {
                            name: file.to_string_lossy().to_string(),
                            kind: ProjectKind::Local(file),
                            project,
                            tab: ObjectKind::Functions,
                            cursor: 0,
                        });
                    }

                    ui.add_space(20.0);

                    let ip_label = ui.label("IP Address:");
                    ui.text_edit_singleline(&mut self.ip_text).labelled_by(ip_label.id);
                
                    let port_label = ui.label("Port:");
                    ui.text_edit_singleline(&mut self.port_text).labelled_by(port_label.id);


                    if ui.button("Open with network").clicked() {
                        let ip = Ipv4Addr::from_str(&self.ip_text).unwrap();
                        let port = u16::from_str(&self.port_text).unwrap();

                        let Ok(client) = Client::connect(SocketAddrV4::new(ip, port)) else {
                            return
                        };

                        self.tabs.push(ProjectMenu {
                            name: self.ip_text.clone(),
                            kind: ProjectKind::Remote(client),
                            project: Project::new(),
                            tab: ObjectKind::Functions,
                            cursor: 0,
                        });
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.tabs.len() == 0 {
                return;
            }
            
            self.tabs[self.current].update(ui);
        });
    }
}

fn main() -> Result<(), error::Error> {
    env_logger::init();

    let app = App::default();

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
