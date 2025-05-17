use std::{
    str::FromStr,
};

use eframe::egui::{self, Context, Window};

use crate::{
    net::Client,
    project::{Project, ProjectKind},
};


pub fn show_error_message(ctx: &Context, message: String) {
    Window::new("Error")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| ui.label(message));
}
