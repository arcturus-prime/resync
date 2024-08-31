use std::sync::Mutex;

use crate::ir::Project;

pub struct Context {
    pub project: Mutex<Project>,
    pub merge_project: Mutex<Option<Project>>,
    
}

