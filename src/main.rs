use eframe::epaint::Vec2;

mod algorithms;
mod bundles;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(990., 700.)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(ui::Visualizer::new(cc))),
    );
}
