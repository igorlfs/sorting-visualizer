use crate::{bundles::Options, ui};
use ui::*;

pub struct ButtonHandler;

impl ButtonHandler {
    /// Sets the `original_numbers` if necessary.
    /// (A workaround since we can't copy it in the constructor)
    ///
    /// When first run, only selects the first elements and mark them as "will be compared",
    /// Subsequent runs make a comparison between selected elements.
    ///
    /// If the numbers are already ordered, advance in the algorithm by modifying the state.
    /// Also Set the next state's elements as "will be compared".
    ///
    /// Else, mark the current numbers as "will be switched".
    ///
    /// The effect produced is that we can handle switching numbers in a different step
    /// (by giving them a fancy color, for instance), while skipping the switch if not necessary.
    pub(crate) fn handle_step(app: &mut Visualizer) {
        if app.original_numbers.is_empty() {
            app.original_numbers = app.bundle.numbers().to_vec();
        }
        if app.bundle.all_default() {
            app.bundle
                .set(app.sorter.get_comparing(), Options::Comparing);
            return;
        }
        if app.state != State::Finished {
            if !app.sorter.step(app.bundle.numbers_mut()) {
                if app.sorter.modify_state(app.bundle.numbers().len()) {
                    app.state = State::Finished;
                }
                app.bundle
                    .set(app.sorter.get_comparing(), Options::Comparing);
            } else {
                app.bundle
                    .set(app.sorter.get_comparing(), Options::Switching);
            }
        }
        if app.state == State::Finished {
            app.reset();
        }
    }

    /// Resets `app` state and sets numbers to their initial state.
    pub(crate) fn handle_reset(app: &mut Visualizer) {
        app.reset();
        app.bundle.set_numbers(app.original_numbers.clone());
    }

    /// Resets `app` state and generates new numbers.
    pub(crate) fn handle_shuffle(app: &mut Visualizer) {
        app.reset();
        app.bundle.set_numbers(util::gen_random_vector(
            constants::FLOOR,
            constants::CEIL,
            constants::VECTOR_SIZE,
        ));
        app.original_numbers = app.bundle.numbers().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bundles::{Bundle, Options},
        ui::{self, buttons::ButtonHandler},
    };

    #[test]
    fn handle_reset() {
        let mut app = ui::Visualizer::default();
        ButtonHandler::handle_reset(&mut app);
        assert_eq!(app.original_numbers, app.bundle.numbers());
    }

    #[test]
    fn handle_shuffle() {
        let mut app = ui::Visualizer::default();
        ButtonHandler::handle_shuffle(&mut app);
        assert_eq!(app.original_numbers, app.bundle.numbers());
    }

    #[test]
    fn handle_step() {
        let numbers: Vec<u32> = vec![5, 2, 6];
        let options: Vec<Options> = vec![Options::Default; numbers.len()];
        let mut app = ui::Visualizer {
            bundle: Bundle::new(numbers, options),
            ..Default::default()
        };

        // This test only works the way it does because it's defaulting to BubbleSort

        // The first time the function is run,
        // original_numbers is copied and the 2 first elements are set to be compared
        ButtonHandler::handle_step(&mut app);
        assert!(!app.original_numbers.is_empty());
        assert_eq!(app.bundle.indexes(), (0, 1));
        assert_eq!(app.bundle.options()[0], Options::Comparing);
        assert_eq!(app.bundle.options()[1], Options::Comparing);

        // Since 5 > 2,
        // The pair is marked for switching
        ButtonHandler::handle_step(&mut app);
        assert_eq!(app.bundle.indexes(), (0, 1));
        assert_eq!(app.bundle.options()[0], Options::Switching);
        assert_eq!(app.bundle.options()[1], Options::Switching);

        // After stepping, the new order is [2, 5, 6]
        // Since 2 < 5 we will modify the state: (0, 1) -> (1,2)
        // And we will be comparing 5 and 6 next round
        ButtonHandler::handle_step(&mut app);
        assert_eq!(app.bundle.indexes(), (1, 2));
        assert_eq!(app.bundle.options()[1], Options::Comparing);
        assert_eq!(app.bundle.options()[2], Options::Comparing);

        // Since 5 < 6 we will modify the state: (1, 2) -> (0, 1)
        // And we will be comparing 2 and 5 next round
        ButtonHandler::handle_step(&mut app);
        assert_eq!(app.bundle.indexes(), (0, 1));
        assert_eq!(app.bundle.options()[0], Options::Comparing);
        assert_eq!(app.bundle.options()[1], Options::Comparing);

        // We never hit the state of Finished,
        // given that once it's set, it's used to reset the app, setting it to Start once again.
        // However, we can still test if everything has been covered by
        // checking if the bundle is in it's Default state
        ButtonHandler::handle_step(&mut app);
        assert!(app.bundle.all_default());
    }
}
