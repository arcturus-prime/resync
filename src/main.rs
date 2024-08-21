mod ir;
mod app;
mod error;

use std::{path::PathBuf, sync::Arc};

use app::App;
use clap::{value_parser, Arg, Command};

use ir::Project;

fn main() {
    let mut app = App::new();

    app.process();
}
