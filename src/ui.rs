use std::{collections::HashSet, path::PathBuf};

use eframe::egui::{self, Ui};

use crate::{ir::Project, net::{Client, Message}};

pub enum ProjectKind {
    Remote(Client),
    Local(PathBuf),
}

pub struct ProjectMenu {
    pub name: String,

    pub selected: HashSet<usize>,

    pub kind: ProjectKind,
    pub project: Project,
}

impl ProjectMenu {
    pub fn new(kind: ProjectKind, name: String, project: Project) -> Self {
        Self {
            name,
            kind,
            selected: HashSet::new(),
            project,
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if let ProjectKind::Remote(client) = &mut self.kind {
            client.update_project(&mut self.project);
        }

        ui.columns(2, |ui| {
            let text_style = ui[0].text_style_height(&egui::TextStyle::Body);

            egui::ScrollArea::vertical().show_rows(
                &mut ui[0],
                text_style,
                self.project.names.len(),
                |ui, row_range| {
                    for i in row_range {
                        let selected = self.selected.contains(&i);
                        let label = ui.selectable_label(selected, &self.project.names[i]);

                        if label.clicked() && selected {
                            self.selected.remove(&i);
                        } else if label.clicked() && !selected {
                            self.selected.insert(i);
                        }
                    }
                },
            );
        });
    }
}
