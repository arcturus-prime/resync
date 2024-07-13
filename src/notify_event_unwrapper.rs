use std::path::PathBuf;
use std::sync::mpsc::Sender;

use hex::decode;

use crate::project::*;

pub struct NotifyEventUnwrapper {
    directory: PathBuf,
    tx: Sender<ObjectEvent>,
}

impl NotifyEventUnwrapper {
    pub fn new(directory: PathBuf, tx: Sender<ObjectEvent>) -> Self {
        Self { directory, tx }
    }
}

impl notify::EventHandler for NotifyEventUnwrapper {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        if event.is_err() {
            return;
        }

        let event = event.unwrap();

        for id in event
            .paths
            .iter()
            .map(|path| {
                if path == &self.directory {
                    return None;
                }

                let path_str = path
                    .strip_prefix(&self.directory)
                    .unwrap()
                    .to_str()
                    .unwrap();

                let decoded_path_str = decode(path_str).unwrap();
                Some(String::from_utf8(decoded_path_str).unwrap())
            })
            .collect::<Vec<_>>()
        {
            if id.is_none() {
                continue;
            }

            match event.kind {
                notify::EventKind::Any
                | notify::EventKind::Access(_)
                | notify::EventKind::Other => todo!(),
                notify::EventKind::Create(_) => {
                    let _ = self.tx.send(ObjectEvent::Added(id.unwrap()));
                }
                notify::EventKind::Modify(_) => {
                    let _ = self.tx.send(ObjectEvent::Added(id.unwrap()));
                }
                notify::EventKind::Remove(_) => {
                    let _ = self.tx.send(ObjectEvent::Deleted(id.unwrap()));
                }
            };
        }
    }
}
