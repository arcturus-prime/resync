use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4},
    path::{Path, PathBuf},
    str::FromStr,
};

use eframe::egui::{self, Ui};
use rfd::FileDialog;

use crate::net::{Client, Message, Object};

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

impl Default for Database {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            names: vec![],
            lookup: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for DatabaseError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl<'a> Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Io(e) => e.fmt(f),
            DatabaseError::Serde(e) => e.fmt(f),
        }
    }
}

struct Database {
    objects: Vec<Object>,
    names: Vec<String>,
    lookup: HashMap<String, usize>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let data: HashMap<String, Object> = serde_json::from_slice(project_data.as_slice())?;

        let mut lookup = HashMap::new();
        let mut objects = Vec::new();
        let mut names = Vec::new();

        for (name, object) in data {
            let index = objects.len();
            objects.push(object);

            names.push(name.clone());
            lookup.insert(name, index);
        }

        Ok(Database {
            objects,
            lookup,
            names,
        })
    }

    pub fn save(&self, path: &Path) -> Result<(), DatabaseError> {
        let mut file;

        if !path.exists() {
            file = File::create(path)?;
        } else {
            file = OpenOptions::new().write(true).open(path)?;
        }

        let mut table = HashMap::new();

        for (name, object) in self.objects.iter().zip(self.names.iter()) {
            table.insert(name, object);
        }

        let data = serde_json::to_vec(&self.objects)?;
        file.write_all(&data)?;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn name_iter(&self) -> impl Iterator<Item = &String> + '_ {
        self.names.iter()
    }

    // TODO: Retreive all dependency types
    pub fn get(&self, name: &str) -> Result<HashMap<String, Object>, DatabaseError> {
        let mut map = HashMap::new();

        let index = self.lookup[name];
        map.insert(name.to_string(), self.objects[index].clone());

        Ok(map)
    }

    pub fn push(&mut self, objects: HashMap<String, Object>) {
        self.objects.extend(objects)
    }

    pub fn delete(&mut self, name: &str) -> Result<(), DatabaseError> {
        self.objects.remove(name);

        Ok(())
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
            data.extend(self.db.get(&name).unwrap());
        }

        data
    }

    // Adds objects to the project (used for pasting)
    // This will send messages over the socket if the project is of kind `Remote`
    pub fn add_objects(&mut self, data: HashMap<String, Object>) {
        self.db.push(data.clone());

        if let ProjectKind::Remote(client) = &mut self.kind {
            let result = client.tx.send(Message::Push { objects: data });

            if let Err(e) = result {
                log::error!("Cannot send pasted objects to network thread: {}", e);
            }
        }
    }

    // Delets an object by name from the project
    // Will send a network message if project is of kind Remote
    pub fn delete_object(&mut self, name: &str) {
        if let Err(e) = self.db.delete(name) {
            log::error!("Could not delete object: {}", e);
            return;
        }
    }

    pub fn render(
        &mut self,
        ui: &mut Ui,
        errors: &mut VecDeque<String>,
        clipboard: &mut HashMap<String, Object>,
    ) {
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

        ui.columns(2, |ui| {
            let text_style = ui[0].text_style_height(&egui::TextStyle::Body);

            egui::ScrollArea::vertical().show_rows(
                &mut ui[0],
                text_style,
                self.db.len(),
                |ui, row_range| {
                    let names = self
                        .db
                        .name_iter()
                        .skip(row_range.start)
                        .take(row_range.count());

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
                Message::Push { objects } => self.db.push(objects),
            }
        }
    }
}
