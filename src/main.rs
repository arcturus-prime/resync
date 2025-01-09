mod error;
mod ir;
mod net;

use std::{collections::HashMap, path::PathBuf};

use eframe::egui::{self, Layout, Ui};
use rfd::FileDialog;

use ir::{ObjectKind, Project};
use net::Client;

enum ProjectKind {
    Remote(Client),
    Local(PathBuf)
}

struct ProjectTab {
    kind: ProjectKind,
    project: Project,

    tab: ObjectKind,
    cursor: usize,
}

struct App {
    tabs: Vec<ProjectTab>,
    current: usize,
}

impl Default for App {
    fn default() -> Self {
        Self { tabs: vec![], current: 0 }
    }
}

fn display_artifact_list<V>(ui: &mut Ui, start: usize, objects: &HashMap<String, V>) {
    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
        for k in objects.keys().skip(start - (start % 20)).take(20) {
            ui.label(k); 
        }
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                for (tab, i) in self.tabs.iter().zip(0..) {
                    if ui.button(&tab.project.name).clicked() {
                        self.current = i;
                    }
                }

                if ui.button("+").clicked() {
                    let Some(file) = FileDialog::new().pick_file() else {
                        return
                    };

                    let Ok(project) = Project::open(&file) else {
                        return
                    };

                    self.tabs.push(ProjectTab { kind: ProjectKind::Local(file.to_path_buf()), project, tab: ObjectKind::Functions, cursor: 0 });
                }
            });

            ui.horizontal_top(|ui| {
                let project_tab = &mut self.tabs[self.current];

                if ui.button("Functions").clicked() {
                    project_tab.tab = ObjectKind::Functions
                }

                if ui.button("Types").clicked() {
                    project_tab.tab = ObjectKind::Types
                }

                if ui.button("Globals").clicked() {
                    project_tab.tab = ObjectKind::Globals
                }

            });

            ui.columns(2, |ui| {
                let project_tab = &self.tabs[self.current];

                match project_tab.tab {
                    ObjectKind::Functions => display_artifact_list(&mut ui[0], project_tab.cursor, &project_tab.project.functions),
                    ObjectKind::Types => display_artifact_list(&mut ui[0], project_tab.cursor, &project_tab.project.types),
                    ObjectKind::Globals => display_artifact_list(&mut ui[0], project_tab.cursor, &project_tab.project.globals),
                };
            })
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
