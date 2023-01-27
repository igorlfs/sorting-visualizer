use eframe::{
    egui::{self, Sense, Ui},
    epaint::{vec2, Color32, Stroke, Vec2},
};
mod util;
use crate::algorithms::sorters;
use crate::algorithms::sorters::Sorter;

#[derive(PartialEq, Debug)]
enum Enum {
    Bubble,
    Merge,
    Quick,
    Shell,
    Radix,
}

const PADDING: f32 = 10.;
const BASE_HEIGHT: u32 = 64;
const BASE_WIDTH: f32 = 32.;
const ROUNDING: f32 = 5.;
const STROKE_WIDTH: f32 = 2.;
const NUMBERS_GRID: &str = "numbers";
const STROKE_COLOR: Color32 = Color32::WHITE;

pub(crate) struct MyEguiApp {
    selected: Enum,
    numbers: Vec<u32>,
    initial_state: Vec<u32>,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            selected: Enum::Bubble,
            numbers: util::gen_random_vector(1, 11, 20),
            initial_state: vec![],
        }
    }
}

impl MyEguiApp {
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    /// Draws rectangles representing the numbers, whose height is proportional to the number.
    /// Use the number as a centralized label.
    fn draw_numbers(&self, ui: &mut Ui) {
        ui.horizontal_top(|ui| {
            ui.add_space(PADDING);
            for i in 0..self.numbers.len() {
                let height: f32 = (BASE_HEIGHT * self.numbers[i]) as f32;
                let size: Vec2 = vec2(BASE_WIDTH, height);
                egui::Grid::new(NUMBERS_GRID).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        let (rect, _response) = ui.allocate_at_least(size, Sense::hover());
                        ui.painter().rect(
                            rect,
                            ROUNDING,
                            Color32::from_gray(64),
                            Stroke::new(STROKE_WIDTH, STROKE_COLOR),
                        );
                        ui.end_row();
                        let text: String = self.numbers[i].to_string();
                        ui.label(text);
                        ui.end_row();
                    });
                });
            }
            ui.add_space(PADDING);
        });
    }

    /// Create buttons and handle their events.
    fn handle_buttons(&mut self, ui: &mut Ui) {
        if ui.add(egui::Button::new("Start")).clicked() {
            self.initial_state = self.numbers.clone();
            sorters::BubbleSort::run(&mut self.numbers);
        }
        if ui.add(egui::Button::new("Step")).clicked() {}
        if ui.add(egui::Button::new("Shuffle")).clicked() {
            self.numbers = util::gen_random_vector(1, 11, 20);
            self.initial_state = self.numbers.clone();
        }
        if ui.add(egui::Button::new("Reset")).clicked() {
            self.numbers = self.initial_state.clone();
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Horizontal is used to align the ComboBox with the buttons
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Choose an algorithm")
                    .selected_text(format!("{:?}", self.selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected, Enum::Bubble, "Bubble Sort");
                        ui.selectable_value(&mut self.selected, Enum::Merge, "Merge Sort");
                        ui.selectable_value(&mut self.selected, Enum::Quick, "Quick Sort");
                        ui.selectable_value(&mut self.selected, Enum::Shell, "Shell Sort");
                        ui.selectable_value(&mut self.selected, Enum::Radix, "Radix Sort");
                    });
                self.handle_buttons(ui);
            });

            ui.add_space(PADDING);
            self.draw_numbers(ui);
        });
    }
}
