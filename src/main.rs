mod algorithms;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(ui::MyEguiApp::new(cc))),
    );
}
