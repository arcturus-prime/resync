mod ir;
mod app;
mod error;

use app::App;

fn main() {
    let mut app = App::new();
    app.process();
}
