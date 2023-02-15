pub mod bogo_sort;
pub mod bubble_sort;
mod constants;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod selection_sort;
/// A Sorter is a sorting algorithm split in two stages: the `step` and the `state`.
/// A `step` can be any single step an algorithm takes, such as comparing or switching numbers
/// A `state` controls the variables that the `step` is going to use.
pub trait Sorter {
    fn new() -> Self
    // The Compiler will complain if we don't do this
    where
        Self: Sized;

    /// Returns the indexes currently being compared or about to switch.
    fn special(&self) -> (usize, usize);

    /// Returns the reason the special indexes are special.
    fn reason(&self) -> Reasons;

    /// Loops all states and reset state.
    fn run(&mut self, array: &mut Vec<usize>) {
        loop {
            if self.step(array) {
                break;
            }
        }
        self.reset_state();
    }

    /// Takes a single step in running the algorithm.
    /// Returns true if all states have been covered.
    fn step(&mut self, array: &mut Vec<usize>) -> bool;

    /// Modifying the state is analogous to stepping in a loop.
    /// Returns true if all states have been traversed.
    fn modify_state(&mut self, array: &[usize]) -> bool;

    /// Handles switching positions in an array
    fn switch(&mut self, array: &mut Vec<usize>);

    /// Set the Sorter's state to it's initial state.
    fn reset_state(&mut self);
}

#[derive(PartialEq, Clone, Copy)]
pub enum Reasons {
    Comparing,
    Switching,
    Limits,
}
