use std::{
    collections::{HashMap, HashSet},
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4},
    path::{Path, PathBuf},
    str::FromStr,
};

use eframe::egui::{self, Ui};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    net::{Client, Message, Object},
    UpdateWidget, Widget,
};

pub struct ProjectListing {
    projects: Vec<Project>,
    current: usize,

    clipboard: ProjectData,

    should_open: bool,
    open_project: OpenProjectMenu,
}

impl Widget for ProjectListing {
    fn render(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                for (tab, i) in self.projects.iter().zip(0..) {
                    if ui.button(&tab.name).clicked() {
                        self.current = i;
                    }
                }

                if ui.button("+").clicked() {
                    self.should_open = true;
                }
            });
        });

        if self.should_open {
            egui::Window::new("Add Project")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    self.open_project
                        .render(ui, (&mut self.projects, &mut self.should_open));
                });
        }

        if self.projects.is_empty() {
            return;
        }

        self.projects[self.current].handle_network_updates();
        self.projects[self.current].render(ui);

        if ui.input(|i| i.events.iter().any(|ev| matches!(ev, egui::Event::Copy))) {
            self.clipboard = self.projects[self.current].get_selected();
        }

        if ui.input(|i| {
            i.events
                .iter()
                .any(|ev| matches!(ev, egui::Event::Paste(_)))
        }) {
            self.projects[self.current].add_objects(self.clipboard.clone());
        }

        if ui.input(|i| i.modifiers.ctrl && i.key_released(egui::Key::S)) {
            self.projects[self.current].save();
        }
    }
}

impl Default for ProjectListing {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            current: 0,

            clipboard: ProjectData::new(),

            should_open: false,
            open_project: OpenProjectMenu::default(),
        }
    }
}

struct OpenProjectMenu {
    ip_text: String,
    port_text: String,
}

impl<'a> UpdateWidget<'a> for OpenProjectMenu {
    type State = (&'a mut Vec<Project>, &'a mut bool);

    fn render(&mut self, ui: &mut Ui, state: Self::State) {
        if ui.button("Close").clicked() {
            *state.1 = false
        }

        ui.add_space(20.0);

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

            state.0.push(project);
            *state.1 = false;
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

            state.0.push(project);
            *state.1 = false;
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
            let ip = Ipv4Addr::from_str(&self.ip_text).unwrap_or(Ipv4Addr::new(127, 0, 0, 1));
            let port = u16::from_str(&self.port_text).unwrap_or(12007);

            let Ok(client) = Client::connect(SocketAddrV4::new(ip, port)) else {
                return;
            };

            let Ok(project) = Project::new(ProjectKind::Remote(client), self.ip_text.clone())
            else {
                return;
            };

            state.0.push(project);
            *state.1 = false
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub objects: Vec<Object>,
}

impl ProjectData {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, Error> {
        let mut project_file = File::open(&path)?;
        let mut project_data = Vec::<u8>::new();

        project_file.read_to_end(&mut project_data)?;
        let object_list = serde_json::from_slice(project_data.as_slice())?;

        Ok(object_list)
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let mut file;

        if !path.exists() {
            file = File::create(path)?;
        } else {
            file = OpenOptions::new().write(true).open(path)?;
        }

        let data = serde_json::to_vec(&self)?;
        file.write_all(&data)?;

        Ok(())
    }
}

pub enum ProjectKind {
    Remote(Client),
    Local(PathBuf),
}

pub struct Project {
    pub name: String,

    selected: HashSet<usize>,
    lookup: HashMap<String, usize>,

    kind: ProjectKind,
    data: ProjectData,
}

impl Project {
    pub fn new(kind: ProjectKind, name: String) -> Result<Self, Error> {
        let mut lookup = HashMap::new();

        let data = match &kind {
            ProjectKind::Remote(_) => ProjectData::new(),
            ProjectKind::Local(path) => {
                let data = if path.exists() {
                    ProjectData::open(&path)?
                } else {
                    ProjectData::new()
                };

                for (id, object) in data.objects.iter().enumerate() {
                    lookup.insert(object.name.to_string(), id);
                }

                data
            }
        };

        Ok(Self {
            name,
            kind,
            selected: HashSet::new(),
            lookup,
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

    // call to handle any incoming network requests and update the project
    pub fn handle_network_updates(&mut self) {
        let ProjectKind::Remote(client) = &mut self.kind else {
            return;
        };

        let Ok(message) = client.rx.try_recv() else {
            return;
        };

        match message {
            Message::Delete { name } => {
                let index = match self.lookup.get(&name) {
                    Some(index) => *index,
                    None => {
                        log::error!(
                            "Received delete message for object that does not exist in project: {}",
                            name
                        );
                        return;
                    }
                };

                let last = self.data.objects.len() - 1;
                if index != last {
                    self.data.objects.swap(index, last);
                    self.data.objects.pop();
                }

                self.lookup
                    .insert(self.data.objects[index].name.clone(), index);
                self.lookup.remove(&name);
            }
            Message::Rename { old, new } => {
                let index = match self.lookup.get(&old) {
                    Some(index) => *index,
                    None => {
                        log::error!(
                            "Received rename message for object that does not exist in project: {}",
                            old
                        );
                        return;
                    }
                };

                self.data.objects[index].name = new.clone();
                self.lookup.remove(&old);
                self.lookup.insert(new, index);
            }
            Message::Push { mut objects } => {
                self.data.objects.append(&mut objects);
            }
        }
    }

    // Get all objects that are selected at the moment in the project listing
    // (used for copying to clipboard)
    pub fn get_selected(&self) -> ProjectData {
        let mut data = ProjectData::new();

        for id in &self.selected {
            data.objects.push(self.data.objects[*id].clone());
        }

        data
    }

    // Adds objects to the project (used for pasting)
    // This will send messages over the socket if the project is of kind `Remote`
    pub fn add_objects(&mut self, data: ProjectData) {
        for i in 0..data.objects.len() {
            let object = data.objects[i].clone();

            if let Some(id) = self.lookup.get(&object.name) {
                self.data.objects[*id] = object;
            } else {
                self.lookup
                    .insert(object.name.clone(), self.data.objects.len());

                self.data.objects.push(object);
            }
        }

        if let ProjectKind::Remote(client) = &mut self.kind {
            let _ = client.tx.send(Message::Push {
                objects: data.objects,
            });
        }
    }
}

impl Widget for Project {
    fn render(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_released(egui::Key::Escape)) {
            self.selected.clear()
        }

        ui.columns(2, |ui| {
            let text_style = ui[0].text_style_height(&egui::TextStyle::Body);

            egui::ScrollArea::vertical().show_rows(
                &mut ui[0],
                text_style,
                self.data.objects.len(),
                |ui, row_range| {
                    for i in row_range {
                        let selected = self.selected.contains(&i);
                        let label = ui.selectable_label(selected, &self.data.objects[i].name);

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
