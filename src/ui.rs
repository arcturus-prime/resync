use std::{collections::HashMap, path::PathBuf};

use eframe::egui::{self, Layout, Ui};

use crate::{ir::{ObjectKind, Project}, net::Client};


pub enum ProjectKind {
    Remote(Client),
    Local(PathBuf),
}

pub struct ProjectMenu {
    pub name: String,

    pub kind: ProjectKind,
    pub project: Project,

    pub tab: ObjectKind,
    pub cursor: usize,
}

fn handle_artifact_list<V>(ui: &mut Ui, start: usize, objects: &HashMap<String, V>) {
    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
        for k in objects.keys().skip(start - (start % 20)).take(20) {
            ui.label(k);
        }
    });
}

impl ProjectMenu {
    pub fn update(&mut self, ui: &mut Ui) {
        if let ProjectKind::Remote(client) = &mut self.kind {
            client.update_project(&mut self.project);
        }

        ui.horizontal_top(|ui| {
            if ui.button("Functions").clicked() {
                self.tab = ObjectKind::Functions
            }

            if ui.button("Types").clicked() {
                self.tab = ObjectKind::Types
            }

            if ui.button("Globals").clicked() {
                self.tab = ObjectKind::Globals
            }
        });

        ui.columns(2, |ui| {
            match self.tab {
                ObjectKind::Functions => handle_artifact_list(
                    &mut ui[0],
                    self.cursor,
                    &self.project.functions,
                ),
                ObjectKind::Types => handle_artifact_list(
                    &mut ui[0],
                    self.cursor,
                    &self.project.types,
                ),
                ObjectKind::Globals => handle_artifact_list(
                    &mut ui[0],
                    self.cursor,
                    &self.project.globals,
                ),
            };
        })
    }
}
