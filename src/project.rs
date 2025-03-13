use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use eframe::egui::{self, Ui};
use serde::{Serialize, Deserialize};

use crate::{error::Error, net::{Client, Message, Object}};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub objects: Vec<Object>,
    pub names: Vec<String>,
}

impl ProjectData {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            names: Vec::new(),
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
        file.write(&data)?;

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

                for (id, name) in data.names.iter().enumerate() {
                    lookup.insert(name.to_string(), id);
                }

                data
            },
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

    pub fn update(&mut self) {
        let ProjectKind::Remote(client) = &mut self.kind else {
            return
        };

        let Ok(message) = client.rx.try_recv() else {
            return
        };

        //TODO: Implement the rest of the network protocol
        match message {
            Message::Delete { name } => {}
            Message::Rename { old, new } => {}
            Message::Push { mut names, mut objects } => {
                self.data.names.append(&mut names);
                self.data.objects.append(&mut objects);
            }
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.columns(2, |ui| {
            let text_style = ui[0].text_style_height(&egui::TextStyle::Body);

            egui::ScrollArea::vertical().show_rows(
                &mut ui[0],
                text_style,
                self.data.names.len(),
                |ui, row_range| {
                    for i in row_range {
                        let selected = self.selected.contains(&i);
                        let label = ui.selectable_label(selected, &self.data.names[i]);

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
    // Get all objects that are selected at the moment in the project listing
    // (used for copying to clipboard)
    pub fn get_selected(&self) -> ProjectData {
        let mut data = ProjectData::new();

        for id in &self.selected {
            data.objects.push(self.data.objects[*id].clone());
            data.names.push(self.data.names[*id].clone());
        }

        data
    }

    // Adds objects to the project (used for pasting)
    //
    // This will send messages over the socket if the project is of kind `Remote`
    pub fn add_objects(&mut self, data: ProjectData) {
        for i in 0..data.objects.len() {
            let name = data.names[i].clone();
            let object = data.objects[i].clone();

            if let Some(id) = self.lookup.get(&name) {
                self.data.names[*id] = name;
                self.data.objects[*id] = object;
            } else {
                self.lookup.insert(name.clone(), self.data.objects.len());

                self.data.objects.push(object);
                self.data.names.push(name);
            }
        }

        if let ProjectKind::Remote(client) = &mut self.kind {
            let _ = client.tx.send(Message::Push {
                names: data.names,
                objects: data.objects
            });
        }
    }
}
