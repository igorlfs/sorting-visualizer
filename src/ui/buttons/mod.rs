use crate::algorithms::sorters::Sorter;

use crate::ui;
use ui::*;

pub struct ButtonHandler;

impl ButtonHandler {
    pub(crate) fn handle_start(app: &mut MyEguiApp) {
        app.initial_state = app.bundle.numbers.clone();
        app.sorter.run(&mut app.bundle.numbers);
        app.finished = true;
    }
    pub(crate) fn handle_step(app: &mut MyEguiApp) {
        if app.initial_state.is_empty() {
            app.initial_state = app.bundle.numbers.clone();
        }
        if !app.finished {
            app.bundle.reset_options();
            let (a, b): (usize, usize) = app.sorter.get_state();
            app.bundle.options[a] = Options::Selected;
            app.bundle.options[b] = Options::Selected;
            app.sorter.step(&mut app.bundle.numbers);
            app.finished = app.sorter.modify_state(app.bundle.numbers.len());
        }
    }
    pub(crate) fn handle_reset(app: &mut MyEguiApp) {
        if app.initial_state.is_empty() {
            app.initial_state = app.bundle.numbers.clone()
        } else {
            app.bundle.numbers = app.initial_state.clone();
            app.finished = false;
            app.sorter.reset();
        }
    }
    pub(crate) fn handle_shuffle(app: &mut MyEguiApp) {
        app.bundle.numbers =
            util::gen_random_vector(constants::FLOOR, constants::CEIL, constants::VECTOR_SIZE);
        app.finished = false;
        app.sorter.reset();
        app.initial_state = app.bundle.numbers.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::ui;

    use crate::ui::buttons::ButtonHandler;

    #[test]
    fn handle_start() {
        let mut app = ui::MyEguiApp::default();
        let original_numbers = app.bundle.numbers.clone();
        ButtonHandler::handle_start(&mut app);
        assert_eq!(app.initial_state, original_numbers);
    }
}
