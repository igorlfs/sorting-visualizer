use eframe::{
    egui::{self, Sense, Ui},
    epaint::{vec2, Color32, Rect, Stroke, Vec2},
};
use std::{thread, time::Duration};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
mod buttons;
mod constants;
use buttons::ButtonHandler;
mod util;
use crate::algorithms::Sorter;
use crate::algorithms::{
    bubble_sort::BubbleSort, insertion_sort::InsertionSort, selection_sort::SelectionSort,
};

#[derive(PartialEq, Debug, EnumIter, Clone, Copy)]
enum Algorithms {
    Bubble,
    Selection,
    Insertion,
    // Merge,
    // Quick,
    // Heap,
}

const CENTRALIZE_PADDING: f32 = 300.;
const PADDING: f32 = 10.;
const BASE_HEIGHT: usize = 64;
const BASE_WIDTH: f32 = 32.;
const ROUNDING: f32 = 5.;
const STROKE_WIDTH: f32 = 2.;
const NUMBERS_GRID: &str = "numbers";
const STROKE_COLOR: Color32 = Color32::WHITE;
const WAIT_MILLIS: u64 = 120;
const WAIT_TIME: Duration = Duration::from_millis(WAIT_MILLIS);

#[derive(PartialEq, Debug)]
enum State {
    Start,
    Running,
    Finished,
}

pub(crate) struct Visualizer<'a> {
    selected: Algorithms,
    numbers: Vec<usize>,
    original_numbers: Vec<usize>,
    state: State,
    sorter: Box<dyn Sorter + 'a>,
}

impl<'a> Default for Visualizer<'a> {
    fn default() -> Self {
        Self {
            selected: Algorithms::Bubble,
            numbers: util::gen_random_vector(
                constants::FLOOR,
                constants::CEIL,
                constants::VECTOR_SIZE,
            ),
            state: State::Start,
            original_numbers: vec![],
            sorter: Box::new(BubbleSort::new()),
        }
    }
}

impl Visualizer<'_> {
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
        let special: (usize, usize) = self.sorter.get_special();
        ui.horizontal_top(|ui| {
            ui.add_space(PADDING);
            for i in 0..self.numbers.len() {
                let text: String = self.numbers[i].to_string();
                let height: f32 = (BASE_HEIGHT * self.numbers[i]) as f32;
                let size: Vec2 = vec2(BASE_WIDTH, height);
                let color = if (i == special.0 || i == special.1) && self.state != State::Finished {
                    Color32::RED
                } else {
                    Color32::GRAY
                };
                Visualizer::draw_numbers_helper(text, size, color, ui);
            }
            ui.add_space(PADDING);
        });
    }

    fn draw_numbers_helper(text: String, size: Vec2, color: Color32, ui: &mut Ui) {
        egui::Grid::new(NUMBERS_GRID).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(text);
                ui.end_row();

                let rect: Rect = ui.allocate_exact_size(size, Sense::hover()).0;
                ui.painter().rect(
                    rect,
                    ROUNDING,
                    color,
                    Stroke::new(STROKE_WIDTH, STROKE_COLOR),
                );
                ui.end_row();
            });
        });
    }

    /// Create the ComboBox and return true if algorithm selection has been changed.
    fn handle_combox_box(&mut self, ui: &mut Ui) -> bool {
        let previous_selection: Algorithms = self.selected;
        ui.label("Algorithm:");
        egui::ComboBox::from_id_source(0)
            .selected_text(format!("{:?}Sort", self.selected))
            .show_ui(ui, |ui| {
                for option in Algorithms::iter() {
                    ui.selectable_value(&mut self.selected, option, format!("{option:?}Sort"));
                }
            });
        previous_selection != self.selected
    }

    /// Change the algorithm based on the selection and perform a reset.
    fn switch_algorithm(&mut self) {
        self.sorter = match self.selected {
            Algorithms::Bubble => Box::new(BubbleSort::new()),
            Algorithms::Selection => Box::new(SelectionSort::new()),
            Algorithms::Insertion => Box::new(InsertionSort::new()),
        };
        ButtonHandler::handle_reset(self);
    }

    /// Create buttons and handle their events.
    fn handle_buttons(&mut self, ui: &mut Ui) {
        if self.state == State::Running {
            if ui.add(egui::Button::new("Stop")).clicked() {
                self.state = State::Start;
            }
            ui.add_enabled(false, egui::Button::new("Step"));
        } else {
            if ui.add(egui::Button::new("Start")).clicked() {
                self.state = State::Running;
            }
            if ui.add(egui::Button::new("Step")).clicked() {
                ButtonHandler::handle_step(self);
            }
        }
        if ui.add(egui::Button::new("Reset")).clicked() {
            ButtonHandler::handle_reset(self);
        }
        if ui.add(egui::Button::new("Shuffle")).clicked() {
            ButtonHandler::handle_shuffle(self);
        }
    }

    /// If running, take a step and sleep for WAIT_TIME.
    fn handle_running(&mut self) {
        if self.state == State::Running {
            ButtonHandler::handle_step(self);
            thread::sleep(WAIT_TIME);
        }
    }

    /// Set all variables to their initial state.
    fn reset(&mut self) {
        if self.original_numbers.is_empty() {
            self.original_numbers = self.numbers.clone();
        }
        self.state = State::Start;
        self.sorter.reset_state();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::ui::State;

    use super::{Visualizer, WAIT_TIME};

    #[test]
    fn handle_running() {
        let mut app = Visualizer {
            state: State::Running,
            ..Default::default()
        };
        let now = Instant::now();
        app.handle_running();
        assert!(now.elapsed() >= WAIT_TIME);
    }

    #[test]
    fn reset() {
        let mut app = Visualizer::default();
        app.reset();
        assert_eq!(app.original_numbers, app.numbers);
        assert_eq!(app.state, State::Start);
        assert_eq!(app.sorter.get_state(), (0, usize::MAX));
    }
}

impl eframe::App for Visualizer<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Horizontal is used to align the ComboBox with the buttons
            ui.horizontal(|ui| {
                ui.add_space(CENTRALIZE_PADDING);
                if self.handle_combox_box(ui) {
                    self.switch_algorithm();
                }
                self.handle_buttons(ui);
            });

            self.handle_running();

            ui.add_space(PADDING);
            self.draw_numbers(ui);
        });
    }
}
