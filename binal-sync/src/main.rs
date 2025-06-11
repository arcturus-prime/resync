mod net;
mod project;

use std::collections::{HashMap, VecDeque};

use eframe::egui::{self, CentralPanel, Context, TopBottomPanel, ViewportBuilder, Window};

use net::Object;
use project::{OpenProjectMenu, Project};

struct App {
    projects: Vec<Project>,
    current: usize,

    errors: VecDeque<String>,
    clipboard: HashMap<String, Object>,

    should_open: bool,
    open_project: OpenProjectMenu,
}

impl Default for App {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            current: 0,

            errors: VecDeque::new(),
            clipboard: HashMap::new(),

            should_open: false,
            open_project: OpenProjectMenu::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let spacing = ui.spacing().item_spacing.x;

                let mut remove: Option<usize> = None;

                for i in 0..self.projects.len() {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    if ui.button(&self.projects[i].name).clicked() {
                        self.current = i;
                    }

                    ui.spacing_mut().item_spacing.x = spacing;

                    if ui.button("X").clicked() {
                        remove = Some(i);
                    }
                }

                if let Some(i) = remove {
                    self.projects.remove(i);
                    self.current = if i == 0 { 0 } else { i - 1 };
                }

                if ui.button("+").clicked() {
                    self.should_open = true;
                }
            });
        });

        if self.should_open {
            Window::new("Add Project")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    self.open_project.render(
                        ui,
                        &mut self.projects,
                        &mut self.errors,
                        &mut self.should_open,
                    );
                });
        }

        if !self.errors.is_empty() {
            Window::new("Error")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(self.errors.front().unwrap());
                    ui.add_space(20.0);

                    if ui.button("Close").clicked() {
                        self.errors.pop_front();
                    }
                });
        }

        if self.projects.is_empty() {
            return;
        }

        for project in &mut self.projects {
            project.handle_network_updates()
        }

        CentralPanel::default().show(ctx, |ui| {
            self.projects[self.current].render(ui, &mut self.errors, &mut self.clipboard)
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init();

    let app = App::default();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native("binal", native_options, Box::new(|_cc| Ok(Box::new(app))))?;

    Ok(())
}
