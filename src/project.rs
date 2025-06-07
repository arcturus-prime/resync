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
};

#[derive(Default)]
pub struct OpenProjectMenu {
    ip_text: String,
    port_text: String,
}

impl OpenProjectMenu {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        projects: &mut Vec<Project>,
        errors: &mut VecDeque<String>,
        remain_open: &mut bool,
    ) {
        if ui.button("Close").clicked() {
            *remain_open = false
        }

        ui.add_space(20.0);

        //Open project from file
        let open = ui.button("Open").clicked();

        ui.add_space(5.0);

        //Create new project
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
                    errors.push_back(format!("Could not open project: {}", e));
                    return;
                }
            };

            projects.push(project);
            *remain_open = false;
        }

        ui.add_space(15.0);

        let ip_label = ui.label("IP Address:");
        ui.add(egui::TextEdit::singleline(&mut self.ip_text).hint_text("127.0.0.1"))
            .labelled_by(ip_label.id);

        let port_label = ui.label("Port:");
        ui.add(egui::TextEdit::singleline(&mut self.port_text).hint_text("12007"))
            .labelled_by(port_label.id);

        if ui.button("Connect").clicked() {
            let ip = Ipv4Addr::from_str(&self.ip_text).unwrap_or(Ipv4Addr::LOCALHOST);
            let port = u16::from_str(&self.port_text).unwrap_or(12007);

            let client = match Client::connect(SocketAddrV4::new(ip, port)) {
                Ok(client) => client,
                Err(e) => {
                    errors.push_back(format!("Could not connect: {}", e));
                    return;
                }
            };

            let project_name = if self.ip_text.is_empty() {
                String::from("127.0.0.1")
            } else {
                std::mem::take(&mut self.ip_text)
            };

            let project = match Project::create(ProjectKind::Remote(client), project_name) {
                Ok(project) => project,
                Err(e) => {
                    errors.push_back(format!("Could not create project: {}", e));
                    return;
                }
            };

            projects.push(project);
            *remain_open = false
        }
    }
}

pub enum ProjectKind {
    Remote(Client),
    Local(PathBuf),
}

enum Tab {
    Types,
    Functions,
    Globals,
}

pub struct Project {
    pub name: String,

    selected: HashSet<String>,
    current_tab: Tab,

    kind: ProjectKind,
    db: Database,
}

impl Project {
    pub fn create(kind: ProjectKind, name: String) -> Result<Self, DatabaseError> {
        let data = match &kind {
            ProjectKind::Remote(_) => Database::default(),
            ProjectKind::Local(path) => {
                let data = if path.exists() {
                    Database::open(&path)?
                } else {
                    Database::default()
                };

                data
            }
        };

        Ok(Self {
            name,
            kind,
            current_tab: Tab::Types,
            selected: HashSet::new(),
            db: data,
        })
    }

    pub fn save(&self, errors: &mut VecDeque<String>) {
        let result = match &self.kind {
            ProjectKind::Local(path) => self.db.save(&path),
            ProjectKind::Remote(_) => Ok(()),
        };

        let Err(e) = result else { return };
        errors.push_back(format!("Could not save project: {}", e))
    }

    // Get all objects that are selected at the moment in the project listing
    // (used for copying to clipboard)
    pub fn get_selected(&self) -> HashMap<String, Object> {
        let mut data = HashMap::new();

        for name in &self.selected {
            let objects = match self.current_tab {
                Tab::Types => self.db.types_get_net(name),
                Tab::Functions => self.db.functions_get_net(name),
                Tab::Globals => self.db.globals_get_net(name),
            };

            data.extend(objects);
        }

        data
    }

    // Adds objects to the project (used for pasting)
    // This will send messages over the socket if the project is of kind `Remote`
    pub fn add_objects(&mut self, data: HashMap<String, Object>) {
        self.db.push_net(data.clone());

        if let ProjectKind::Remote(client) = &mut self.kind {
            let result = client.tx.send(Message::Push { objects: data });

            if let Err(e) = result {
                log::error!("Cannot send pasted objects to network thread: {}", e);
            }
        }
    }

    // Delets an object by name from the project
    pub fn delete_object(&mut self, name: &str) {
        if let ProjectKind::Remote(_) = &self.kind {
            return;
        }

        match self.current_tab {
            Tab::Types => self.db.types.delete(name),
            Tab::Functions => self.db.functions.delete(name),
            Tab::Globals => self.db.data.delete(name),
        };
    }

    // Render the project UI and handle input
    pub fn render(
        &mut self,
        ui: &mut Ui,
        errors: &mut VecDeque<String>,
        clipboard: &mut HashMap<String, Object>,
    ) {
        if ui.input(|i| i.modifiers.ctrl && i.key_released(egui::Key::S)) {
            self.save(errors);
        }

        if ui.input(|i| i.events.iter().any(|e| matches!(e, egui::Event::Paste(_)))) {
            self.add_objects(clipboard.clone())
        }

        if ui.input(|i| i.events.iter().any(|e| matches!(e, egui::Event::Copy))) {
            *clipboard = self.get_selected()
        }

        if ui.input(|i| i.key_released(egui::Key::Delete)) {
            let contents = std::mem::take(&mut self.selected);

            for name in contents {
                self.delete_object(&name)
            }
        }

        if ui.input(|i| i.key_released(egui::Key::Escape)) {
            self.selected.clear()
        }

        ui.horizontal(|ui| {
            if ui.button("Types").clicked() {
                self.current_tab = Tab::Types
            }

            if ui.button("Functions").clicked() {
                self.current_tab = Tab::Functions
            }

            if ui.button("Globals").clicked() {
                self.current_tab = Tab::Globals
            }
        });

        match self.current_tab {
            Tab::Types => Self::render_main_view(
                &mut self.selected,
                ui,
                self.db.types.len(),
                self.db.types.iter().map(|t| &t.name),
            ),
            Tab::Functions => Self::render_main_view(
                &mut self.selected,
                ui,
                self.db.functions.len(),
                self.db.functions.iter().map(|f| &f.name()),
            ),
            Tab::Globals => Self::render_main_view(
                &mut self.selected,
                ui,
                self.db.data.len(),
                self.db.data.iter().map(|d| &d.name),
            ),
        };
    }

    fn render_main_view<'a, I: Iterator<Item = &'a String>>(
        selected: &mut HashSet<String>,
        ui: &mut Ui,
        len: usize,
        iter: I,
    ) {
        let text_style = ui.text_style_height(&egui::TextStyle::Body);

        ui.columns(2, |ui| {
            egui::ScrollArea::vertical().show_rows(&mut ui[0], text_style, len, |ui, row_range| {
                let names = iter.skip(row_range.start).take(row_range.count());

                for name in names {
                    let is_selected = selected.contains(name);
                    let label = ui.selectable_label(is_selected, name);

                    if label.clicked() && is_selected {
                        selected.remove(name);
                    } else if label.clicked() && !is_selected {
                        selected.insert(name.clone());
                    }
                }
            });
        })
    }

    // Handle incoming network messages
    pub fn handle_network_updates(&mut self) {
        loop {
            let ProjectKind::Remote(client) = &mut self.kind else {
                return;
            };

            let Ok(message) = client.rx.try_recv() else {
                return;
            };

            match message {
                Message::Push { objects } => self.db.push_net(objects),
            }
        }
    }
}
