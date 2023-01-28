use crate::algorithms::Sorter;

use crate::ui;
use ui::*;

pub struct ButtonHandler;

impl ButtonHandler {
    pub(crate) fn handle_start(app: &mut Visualizer) {
        app.initial_state = app.bundle.numbers_mut().clone();
        app.sorter.run(app.bundle.numbers_mut());
        app.finished = true;
        app.bundle.reset_options();
    }
    pub(crate) fn handle_step(app: &mut Visualizer) {
        if app.initial_state.is_empty() {
            app.initial_state = app.bundle.numbers_mut().clone();
        }
        if !app.finished {
            app.bundle.set_selected(&app.sorter);
            app.sorter.step(app.bundle.numbers_mut());
            app.finished = app.sorter.modify_state(app.bundle.numbers().len());
        } else {
            app.bundle.reset_options();
        }
    }
    pub(crate) fn handle_reset(app: &mut Visualizer) {
        app.bundle.reset_options();
        if app.initial_state.is_empty() {
            app.initial_state = app.bundle.numbers_mut().clone()
        } else {
            app.bundle.set_numbers(app.initial_state.clone());
            app.finished = false;
            app.sorter.reset();
        }
    }
    pub(crate) fn handle_shuffle(app: &mut Visualizer) {
        app.bundle.set_numbers(util::gen_random_vector(
            constants::FLOOR,
            constants::CEIL,
            constants::VECTOR_SIZE,
        ));
        app.finished = false;
        app.sorter.reset();
        app.bundle.reset_options();
        app.initial_state = app.bundle.numbers_mut().clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::ui;

    use crate::ui::buttons::ButtonHandler;

    #[test]
    fn handle_start() {
        let mut app = ui::Visualizer::default();
        let original_numbers = app.bundle.numbers_mut().clone();
        ButtonHandler::handle_start(&mut app);
        assert_eq!(app.initial_state, original_numbers);
    }
}
