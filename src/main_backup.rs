mod app;
mod assets;
mod project;
mod render;
mod timeline;
mod ui;

fn main() -> Result<(), eframe::Error> {
    app::launch()
}
