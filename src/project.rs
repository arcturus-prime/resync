use std::{
    collections::{HashMap, HashSet, VecDeque},
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
    str::FromStr,
};

use eframe::egui::{self, Ui};
use rfd::FileDialog;

use crate::{
    ir::{Database, DatabaseError},
    net::{Client, Message, Object},
    Widget,
};

pub struct OpenProjectMenuUpdate<'a> {
    pub projects: &'a mut Vec<Project>,
    pub open: &'a mut bool,
    pub errors: &'a mut VecDeque<String>,
}

pub struct OpenProjectMenu {
    ip_text: String,
    port_text: String,
}

impl<'a> Widget<'a> for OpenProjectMenu {
    type State = OpenProjectMenuUpdate<'a>;

    fn render(&mut self, ui: &mut Ui, state: Self::State) {
        if ui.button("Close").clicked() {
            *state.open = false
        }

        ui.add_space(20.0);

        //Open project from file
        let open = ui.button("Open").clicked();

        //Create new project
        ui.add_space(5.0);

        let new = ui.button("New").clicked();

        if open || new {
            let file = if new {
                FileDialog::new().save_file()
            } else {
                FileDialog::new().pick_file()
            };

            let Some(file) = file else { return };

            let filename = file.file_name().unwrap().to_string_lossy().to_string();

            let project = match Project::create(ProjectKind::Local(file), filename) {
                Ok(project) => project,
                Err(e) => {
                    state
                        .errors
                        .push_back(format!("Could not open project: {}", e));
                    return;
                }
            };

            state.projects.push(project);
            *state.open = false;
        }

        ui.add_space(15.0);

        let ip_label = ui.label("IP Address:");
        ui.add(egui::TextEdit::singleline(&mut self.ip_text).hint_text("127.0.0.1"))
            .labelled_by(ip_label.id);

        let port_label = ui.label("Port:");
        ui.add(egui::TextEdit::singleline(&mut self.port_text).hint_text("12007"))
            .labelled_by(port_label.id);

        if ui.button("Connect").clicked() {
            let ip = Ipv4Addr::from_str(&self.ip_text).unwrap_or(Ipv4Addr::new(127, 0, 0, 1));
            let port = u16::from_str(&self.port_text).unwrap_or(12007);

            let client = match Client::connect(SocketAddrV4::new(ip, port)) {
                Ok(client) => client,
                Err(e) => {
                    state.errors.push_back(format!("Could not connect: {}", e));
                    return;
                }
            };

            let project = match Project::create(ProjectKind::Remote(client), self.ip_text.clone()) {
                Ok(project) => project,
                Err(e) => {
                    state
                        .errors
                        .push_back(format!("Could not create project: {}", e));
                    return;
                }
            };

            state.projects.push(project);
            *state.open = false
        }
    }
}

impl Default for OpenProjectMenu {
    fn default() -> Self {
        Self {
            ip_text: String::new(),
            port_text: String::new(),
        }
    }
}

pub enum ProjectKind {
    Remote(Client),
    Local(PathBuf),
}

pub struct Project {
    pub name: String,

    selected: HashSet<String>,

    kind: ProjectKind,
    data: Database,
}

impl Project {
    pub fn create(kind: ProjectKind, name: String) -> Result<Self, DatabaseError> {
        let data = match &kind {
            ProjectKind::Remote(_) => Database::new(),
            ProjectKind::Local(path) => {
                let data = if path.exists() {
                    Database::open(&path)?
                } else {
                    Database::new()
                };


                data
            }
        };

        Ok(Self {
            name,
            kind,
            selected: HashSet::new(),
            data,
        })
    }

    pub fn save(&self) {
        //TODO: Handle this error using GUI
        let result = match &self.kind {
            ProjectKind::Local(path) => self.data.save(&path),
            ProjectKind::Remote(_) => Ok(()),
        };
    }

    // Get all objects that are selected at the moment in the project listing
    // (used for copying to clipboard)
    pub fn get_selected(&self) -> Vec<Object> {
        let mut data = Vec::new();

        for name in &self.selected {
            data.push(self.data.get(&name).unwrap());
        }

        data
    }

    // Adds objects to the project (used for pasting)
    // This will send messages over the socket if the project is of kind `Remote`
    pub fn add_objects(&mut self, data: Vec<Object>) {

        if let ProjectKind::Remote(client) = &mut self.kind {
            let _ = client.tx.send(Message::Push { objects: data });
        }
    }
}

pub struct ProjectUpdate<'a> {
    pub errors: &'a mut VecDeque<String>,
}

impl<'a> Widget<'a> for Project {
    type State = ProjectUpdate<'a>;

    fn render(&mut self, ui: &mut Ui, state: Self::State) {
        if ui.input(|i| i.key_released(egui::Key::Escape)) {
            self.selected.clear()
        }

        ui.columns(2, |ui| {
            let text_style = ui[0].text_style_height(&egui::TextStyle::Body);

            egui::ScrollArea::vertical().show_rows(
                &mut ui[0],
                text_style,
                self.data.len(),
                |ui, row_range| {
                    let names = self.data.name_iter().skip(row_range.start).take(row_range.count());
                    
                    for name in names {
                        let selected = self.selected.contains(name);
                        let label = ui.selectable_label(selected, name);

                        if label.clicked() && selected {
                            self.selected.remove(name);
                        } else if label.clicked() && !selected {
                            self.selected.insert(name.clone());
                        }
                    }
                },
            );
        });

        let ProjectKind::Remote(client) = &mut self.kind else {
            return;
        };

        let Ok(message) = client.rx.try_recv() else {
            return;
        };

        match message {
            Message::Delete { name } => {
                if let Err(e) = self.data.delete(&name) {
                    state.errors.push_back(format!("Delete message failed: {}", e));
                }
            }
            Message::Rename { old, new } => {
                if let Err(e) = self.data.rename(&old, new) {
                    state.errors.push_back(format!("Rename message failed: {}", e));
                }
            }
            Message::Push { objects } => {
                for object in objects {
                    self.data.push(object)
                }
            }
        }
    }
}
