use eframe::egui;

#[derive(PartialEq, Default, Debug)]
enum Enum {
    #[default]
    Bubble,
    Merge,
    Quick,
    Shell,
    Radix,
}

#[derive(Default)]
pub(crate) struct MyEguiApp {
    selected: Enum,
}

impl MyEguiApp {
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Choose an algorithm")
                .selected_text(format!("{:?}", self.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, Enum::Bubble, "Bubble Sort");
                    ui.selectable_value(&mut self.selected, Enum::Merge, "Merge Sort");
                    ui.selectable_value(&mut self.selected, Enum::Quick, "Quick Sort");
                    ui.selectable_value(&mut self.selected, Enum::Shell, "Shell Sort");
                    ui.selectable_value(&mut self.selected, Enum::Radix, "Radix Sort");
                });
        });
    }
}
