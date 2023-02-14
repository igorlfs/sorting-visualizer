use crate::ui;
use ui::*;

use super::constants::{CEIL, FLOOR, VECTOR_SIZE};

pub struct ButtonHandler;

impl ButtonHandler {
    /// If not finished, takes a single step within the selected algorithm
    /// Else, resets the app state.
    pub(crate) fn handle_step(app: &mut Visualizer) {
        if app.state != State::Finished && app.sorter.step(&mut app.numbers) {
            app.state = State::Finished;
        }
        if app.state == State::Finished {
            app.reset();
        }
    }

    /// Resets `app` state and sets `numbers` to their initial state.
    pub(crate) fn handle_reset(app: &mut Visualizer) {
        app.reset();
        app.numbers = app.original_numbers.clone();
    }

    /// Resets `app` state, generates new numbers and update the initial state.
    pub(crate) fn handle_shuffle(app: &mut Visualizer) {
        app.reset();
        app.numbers = util::gen_random_vector(FLOOR, CEIL, VECTOR_SIZE);
        app.original_numbers = app.numbers.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::{self, buttons::ButtonHandler};

    #[test]
    fn handle_reset() {
        let mut app = ui::Visualizer::default();
        ButtonHandler::handle_reset(&mut app);
        assert_eq!(app.original_numbers, app.numbers);
    }

    #[test]
    fn handle_shuffle() {
        let mut app = ui::Visualizer::default();
        ButtonHandler::handle_shuffle(&mut app);
        assert_eq!(app.original_numbers, app.numbers);
    }

    #[test]
    fn handle_step() {
        let numbers: Vec<usize> = vec![5, 2, 6];
        let mut app = ui::Visualizer {
            numbers,
            ..Default::default()
        };

        // This test only works the way it does because it's defaulting to BubbleSort

        // The first time the function is run the 2 first elements are set to be compared
        ButtonHandler::handle_step(&mut app);
        assert_eq!(app.sorter.special(), (0, 1));

        // Since 5 > 2,
        // The pair is marked for switching
        ButtonHandler::handle_step(&mut app);
        assert_eq!(app.sorter.special(), (0, 1));

        // After stepping, the new order is [2, 5, 6]
        // Since 2 < 5 we will modify the state: (0, 1) -> (1,2)
        // And we will be comparing 5 and 6 next round
        ButtonHandler::handle_step(&mut app);
        assert_eq!(app.sorter.special(), (1, 2));

        // Since 5 < 6 we will modify the state: (1, 2) -> (0, 1)
        // And we will be comparing 2 and 5 next round
        ButtonHandler::handle_step(&mut app);
        assert_eq!(app.sorter.special(), (0, 1));
    }
}
