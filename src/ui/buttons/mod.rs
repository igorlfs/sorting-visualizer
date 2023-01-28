use crate::ui;
use ui::*;

pub struct ButtonHandler;

impl ButtonHandler {
    pub(crate) fn handle_step(app: &mut Visualizer) {
        if app.initial_state.is_empty() {
            app.initial_state = app.bundle.numbers_mut().clone();
        }
        if app.bundle.all_default() {
            app.bundle.set_comparing(app.sorter.get_comparing());
            return;
        }
        if app.state != State::Finished {
            if !app.sorter.step(app.bundle.numbers_mut()) {
                if app.sorter.modify_state(app.bundle.numbers().len()) {
                    app.state = State::Finished;
                }
                app.bundle.set_comparing(app.sorter.get_comparing());
            } else {
                app.bundle.set_switching(app.sorter.get_comparing());
            }
        }
        if app.state == State::Finished {
            app.reset();
        }
    }
    pub(crate) fn handle_reset(app: &mut Visualizer) {
        app.reset();
        app.bundle.set_numbers(app.initial_state.clone());
    }
    pub(crate) fn handle_shuffle(app: &mut Visualizer) {
        app.reset();
        app.bundle.set_numbers(util::gen_random_vector(
            constants::FLOOR,
            constants::CEIL,
            constants::VECTOR_SIZE,
        ));
        app.initial_state = app.bundle.numbers_mut().clone();
    }
}

#[cfg(test)]
mod tests {}
