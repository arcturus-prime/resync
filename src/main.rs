mod error;
mod net;
mod project;
mod ui;

use eframe::egui::{ViewportBuilder, CentralPanel, Context, Ui};

use project::ProjectListing;

pub trait Widget {
    fn render(&mut self, ui: &mut Ui);
}

pub trait UpdateWidget<'a> {
    type State;

    fn render(&'a mut self, ui: &mut Ui, state: Self::State);
}

struct App {
    projects: ProjectListing,
}

impl Default for App {
    fn default() -> Self {
        Self {
            projects: ProjectListing::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.projects.render(ui)
        });
    }
}

fn main() -> Result<(), error::Error> {
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
