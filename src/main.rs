mod error;
mod net;
mod ui;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use eframe::egui::{self, Ui};
use net::Client;
use rfd::FileDialog;

use ui::{Project, ProjectKind, ClipboardEntry};

#[derive(PartialEq)]
enum Focus {
    Open,
    View,
}

struct App {
    tabs: Vec<Project>,
    current: usize,

    clipboard: Vec<ClipboardEntry>,

    ip_text: String,
    port_text: String,

    focus: Focus,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tabs: vec![],
            current: 0,

            clipboard: Vec::new(),

            ip_text: String::new(),
            port_text: String::new(),

            focus: Focus::View,
        }
    }
}

impl App {
    fn handle_add_project_window(&mut self, ui: &mut Ui) {
        //Open project from file
        if ui.button("Open").clicked() {
            let Some(file) = FileDialog::new().pick_file() else {
                return;
            };

            let Some(filename) = file.file_name() else {
                return;
            };
            let filename = filename.to_string_lossy().to_string();

            let Ok(project) = Project::new(ProjectKind::Local(file), filename) else {
                return;
            };

            self.tabs.push(project);
            self.focus = Focus::View;
        }

        //Create new project
        ui.add_space(5.0);

        if ui.button("New").clicked() {
            let Some(file) = FileDialog::new().save_file() else {
                return;
            };

            let Some(filename) = file.file_name() else {
                return;
            };
            let filename = filename.to_string_lossy().to_string();
            
            let Ok(project) = Project::new(ProjectKind::Local(file), filename) else {
                return;
            };

            self.tabs.push(project);
            self.focus = Focus::View;
        }

        // Open project with connection to client
        ui.add_space(15.0);

        let ip_label = ui.label("IP Address:");
        ui.text_edit_singleline(&mut self.ip_text)
            .labelled_by(ip_label.id);

        let port_label = ui.label("Port:");
        ui.text_edit_singleline(&mut self.port_text)
            .labelled_by(port_label.id);

        if ui.button("Connect").clicked() {
            let ip = Ipv4Addr::from_str(&self.ip_text).unwrap();
            let port = u16::from_str(&self.port_text).unwrap();

            let Ok(client) = Client::connect(SocketAddrV4::new(ip, port)) else {
                return;
            };

            let Ok(project) = Project::new(ProjectKind::Remote(client), self.ip_text.clone())
            else {
                return;
            };

            self.tabs.push(project);
            self.focus = Focus::View;
        }
    }

    fn handle_top_project_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for (tab, i) in self.tabs.iter().zip(0..) {
                if ui.button(&tab.name).clicked() {
                    self.current = i;
                }
            }

            if ui.button("+").clicked() {
                self.focus = Focus::Open
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| self.handle_top_project_bar(ui));

        if self.focus == Focus::Open {
            egui::Window::new("Add Project")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| self.handle_add_project_window(ui));
        }

        if self.tabs.len() == 0 {
            return;
        }

        if ctx.input(|i| i.key_down(egui::Key::Copy)) {
            self.clipboard.push(ClipboardEntry { project: self.current, ids: self.tabs[self.current].selected.clone(), tab: self.tabs[self.current].tab});
        }

        if ctx.input(|i| i.key_down(egui::Key::Paste)) {
            let _ = self.tabs[self.current].copy_from_others(&self.tabs, &self.clipboard);
            self.clipboard.clear();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
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

    eframe::run_native("resync", native_options, Box::new(|_cc| Ok(Box::new(app))))?;

    Ok(())
}
