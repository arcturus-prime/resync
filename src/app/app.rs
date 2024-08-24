use crate::ir::Project;

use super::merge::MergeConflictMenu;

pub struct App {
    pub project: Project,
    pub merge_menu: MergeConflictMenu
}
 