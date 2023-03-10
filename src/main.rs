use eframe::{epaint::Vec2, run_native, NativeOptions};

mod algorithms;
mod ui;
mod util;

fn main() {
    let native_options = NativeOptions {
        initial_window_size: Some(Vec2::new(990., 710.)),
        ..Default::default()
    };
    run_native(
        "Visualizer",
        native_options,
        Box::new(|cc| Box::new(ui::Visualizer::new(cc))),
    )
    .unwrap();
}
