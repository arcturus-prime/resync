use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use eframe::egui::{self, Ui};
use serde::{Serialize, Deserialize};

use crate::{error::Error, ir::Object, net::{Client, Message}};

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
        let mut transaction;

        if !path.exists() {
            transaction = File::create(path)?;
        } else {
            transaction = OpenOptions::new().write(true).open(path)?;
        }

        let data = serde_json::to_vec_pretty(&self.objects)?;
        transaction.write(&data)?;

        Ok(())
    }
}

pub enum ProjectKind {
    Remote(Client),
    Local(PathBuf),
}

pub struct Project {
    pub name: String,

    pub selected: HashSet<usize>,

    lookup: HashMap<String, usize>,

    kind: ProjectKind,
    data: ProjectData,
}

impl Project {
    pub fn new(kind: ProjectKind, name: String) -> Result<Self, Error> {
        let mut lookup = HashMap::new();

        let data = match &kind {
            ProjectKind::Remote(client) => ProjectData::new(),
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

    pub fn update(&mut self, ui: &mut Ui) {
        if let ProjectKind::Remote(client) = &mut self.kind {
            let Ok(message) = client.rx.try_recv() else {
                return;
            };

            match message {
                Message::Delete { name } => {}
                Message::Rename { old, new } => {}
                Message::Push { name, object } => {}
                Message::Sync { names, objects } => {
                    self.data.names = names;
                    self.data.objects = objects;
                }
            }
        }

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

    pub fn add_object(&mut self, name: String, object: Object) {
        self.data.objects.push(object);
        self.data.names.push(name);
    }

    pub fn get_object(&self, id: usize) -> (&String, &Object) {
        (&self.data.names[id], &self.data.objects[id])
    }
}
