mod net;
mod project;
mod ir;

use std::collections::VecDeque;

use eframe::egui::{self, CentralPanel, Context, TopBottomPanel, Ui, ViewportBuilder, Window};

use project::{OpenProjectMenu, OpenProjectMenuUpdate, Project, ProjectUpdate};

pub trait Widget<'a> {
    type State;

    fn render(&'a mut self, ui: &mut Ui, state: Self::State);
}

struct App {
    projects: Vec<Project>,
    current: usize,

    errors: VecDeque<String>,

    should_open: bool,
    open_project: OpenProjectMenu,
}

impl Default for App {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            current: 0,

            errors: VecDeque::new(),

            should_open: false,
            open_project: OpenProjectMenu::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
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
            Window::new("Add Project")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    self.open_project.render(
                        ui,
                        OpenProjectMenuUpdate {
                            projects: &mut self.projects,
                            open: &mut self.should_open,
                            errors: &mut self.errors,
                        },
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

        CentralPanel::default().show(ctx, |ui| {
            self.projects[self.current].render(
                ui,
                ProjectUpdate {
                    errors: &mut self.errors,
                },
            )
        });

        if ctx.input(|i| i.modifiers.ctrl && i.key_released(egui::Key::S)) {
            self.projects[self.current].save();
        }
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
