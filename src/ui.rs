use std::{
    collections::HashSet,
    path::PathBuf,
};

use eframe::egui::{self, Ui};
use rusqlite::Connection;

use crate::{error::Error, net::{Client, Message}};

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS Types(
     id INT PRIMARY KEY AUTOINCREMENT,
     name TEXT,
     data TEXT,
);

CREATE TABLE IF NOT EXISTS Globals(
     id INT PRIMARY KEY AUTOINCREMENT,
     name TEXT,
     data TEXT,
);

CREATE TABLE IF NOT EXISTS Functions(
     id INT PRIMARY KEY AUTOINCREMENT,
     name TEXT,
     data TEXT,
);";

#[derive(Clone)]
pub enum Tab {
    Types,
    Globals,
    Functions
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let _ = match self {
            Self::Types => write!(f, "Types"),
            Self::Globals => write!(f, "Globals"),
            Self::Functions => write!(f, "Functions"),
        };

        Ok(())
    }
}

impl Copy for Tab {}

pub struct ClipboardEntry {
    pub project: usize,
    pub tab: Tab,
    pub ids: HashSet<usize>,
}

pub enum ProjectKind {
    Remote(Client),
    Local(PathBuf),
}

pub struct Project {
    pub name: String,
    pub selected: HashSet<usize>,
    pub tab: Tab,

    pub kind: ProjectKind,
    pub data: Connection,
}

impl Project {
    pub fn new(kind: ProjectKind, name: String) -> Result<Self, Error> {
        let data = match &kind {
            ProjectKind::Remote(_) => Connection::open_in_memory()?,
            ProjectKind::Local(path) => Connection::open(path)?
        };

        data.execute(SCHEMA, ())?;

        Ok(Self {
            name,
            selected: HashSet::new(),
            tab: Tab::Functions,
            kind,
            data,
        })
    }

    fn update_with_message(&mut self, message: Message) -> Result<(), Error> {
            match message {
                Message::Delete { name } => {}
                Message::Rename { old, new } => {}
                Message::Push { names, objects } => {
                    let tx = self.data.transaction()?;
                    {
                        let mut stmt = tx.prepare(&format!("INSERT INTO {} VALUES(, ?1, ?2) ON CONFLICT name DO UPDATE SET data = ?2;", self.tab))?;

                        for pair in names.iter().zip(objects.iter().map(|o| serde_json::to_string(o).expect("Failed to serialize incoming object"))) {
                            stmt.execute(pair)?;
                        }
                    }

                    tx.commit()?;
                }
            };

            Ok(())
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if let ProjectKind::Remote(client) = &mut self.kind {
            if let Ok(message) = client.rx.try_recv() {
                let _ = self.update_with_message(message);
            }
        }

        let _ = ui.columns(2, |ui| -> Result<(), Error> {
            let text_style = ui[0].text_style_height(&egui::TextStyle::Body);

            let num = self.data.query_row(&format!("SELECT COUNT(*) FROM {};", self.tab), [], |r| r.get(0))?;
            let mut stat = self.data.prepare(&format!("SELECT id, name FROM {} WHERE id > ? AND id < ?;", self.tab))?;
            
            egui::ScrollArea::vertical().show_rows(
                &mut ui[0],
                text_style,
                num,
                |ui, row_range| -> Result<(), Error> {
                    let mut rows = stat.query((row_range.start, row_range.end))?;

                    while let Some(row) = rows.next()? {
                        let id = row.get(0)?;
                        let name: String = row.get(1)?;

                        let selected = self.selected.contains(&id);
                        let label = ui.selectable_label(selected, &name);

                        if label.clicked() && selected {
                            self.selected.remove(&id);
                        } else if label.clicked() && !selected {
                            self.selected.insert(id);
                        }
                    }

                    Ok(())
                },
            );

            Ok(())
        });
    }

    pub fn copy_from_others(&self, projects: &[Project], clipboard: &[ClipboardEntry]) -> Result<(), Error> {
        for entry in clipboard {
            let mut stmt = projects[entry.project].data.prepare(&format!("SELECT name, data FROM {} WHERE id = ?", entry.tab))?; 

            for id in &entry.ids {
                let _ = stmt.query_row([id], |r| {
                    let name: String = r.get(0)?;
                    let data: String = r.get(1)?;

                    self.data.execute(&format!("INSERT INTO {} VALUES(, ?2, ?3) ON CONFLICT(name) DO UPDATE SET data = ?3", entry.tab), (name, data))
                });
            }
        }

        Ok(())
    }
}
