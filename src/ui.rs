use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use eframe::egui::{self, Context, Window};
use rfd::FileDialog;

use crate::{
    App, Component,
    net::Client,
    project::{Project, ProjectKind},
};

struct ProjectTabBar {

}

impl Component for ProjectTabBar {
    fn render(&mut self, ctx: &Context, app: &mut App) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| self.handle_top_project_bar(ui));
    }
}

struct OpenProjectMenu {
    ip_text: String,
    port_text: String,
}

impl Component for OpenProjectMenu {
    fn render(&mut self, ctx: &Context, app: &mut App) {
        egui::Window::new("Add Project")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                //Open project from file
                if ui.button("Open").clicked() {
                    let Some(file) = FileDialog::new().pick_file() else {
                        return;
                    };

                    let Some(filename) = file.file_name() else {
                        return;
                    };
                    let filename = filename.to_string_lossy().to_string();

                    let project = match Project::new(ProjectKind::Local(file), filename) {
                        Ok(project) => project,
                        Err(e) => {
                            println!("{e}");
                            return;
                        }
                    };

                    app.components.push(Box::new(project));
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

                    app.components.push(Box::new(project));
                }

                // Open project with connection to client
                ui.add_space(15.0);

                let ip_label = ui.label("IP Address:");
                ui.add(egui::TextEdit::singleline(&mut self.ip_text).hint_text("127.0.0.1"))
                    .labelled_by(ip_label.id);

                let port_label = ui.label("Port:");
                ui.add(egui::TextEdit::singleline(&mut self.port_text).hint_text("12007"))
                    .labelled_by(port_label.id);

                if ui.button("Connect").clicked() {
                    let ip =
                        Ipv4Addr::from_str(&self.ip_text).unwrap_or(Ipv4Addr::new(127, 0, 0, 1));
                    let port = u16::from_str(&self.port_text).unwrap_or(12007);

                    let Ok(client) = Client::connect(SocketAddrV4::new(ip, port)) else {
                        return;
                    };

                    let Ok(project) =
                        Project::new(ProjectKind::Remote(client), self.ip_text.clone())
                    else {
                        return;
                    };

                    app.components.push(Box::new(project));
                }
            });
    }
}

pub fn show_error_message(ctx: &Context, message: String) {
    Window::new("Error")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| ui.label(message));
}
