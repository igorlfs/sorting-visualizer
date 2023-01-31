/// A Sorter is a sorting algorithm split in two stages: the `step` and the `state`.
/// A `step` can be any single step an algorithm takes, such as comparing or switching numbers
/// A `state` controls the variables that the `step` is going to use.
pub trait Sorter {
    fn new() -> Self
    // The Compiler will complain if we don't do this
    where
        Self: Sized;

    /// Returns the indexes currently being compared.
    fn get_comparing(&self) -> (usize, usize);

    /// Returns the indexes that will be switching.
    fn get_switching(&self) -> (usize, usize);

    /// Return the loop's variables
    fn get_state(&self) -> (usize, usize);

    /// Runs the algorithm all at once.
    fn run(&mut self, array: &mut Vec<usize>);

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

pub struct BubbleSort {
    x: usize,
    y: usize,
    needs_switch: bool,
}

impl BubbleSort {}

impl Sorter for BubbleSort {
    fn new() -> BubbleSort {
        BubbleSort {
            x: 0,
            y: usize::MAX,
            needs_switch: false,
        }
    }
    fn get_comparing(&self) -> (usize, usize) {
        if self.y != usize::MAX {
            (self.y, self.y + 1)
        } else {
            (usize::MAX, usize::MAX)
        }
    }

    fn get_switching(&self) -> (usize, usize) {
        self.get_comparing()
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn run(&mut self, array: &mut Vec<usize>) {
        loop {
            if self.needs_switch {
                self.switch(array);
            } else if self.modify_state(array) {
                break;
            }
        }
        self.reset_state();
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.needs_switch {
            self.switch(array);
        } else {
            return self.modify_state(array);
        }
        false
    }

    fn modify_state(&mut self, array: &[usize]) -> bool {
        if self.x == array.len() - 1 {
            return true;
        }
        if self.y < array.len() - 1 - self.x {
            self.y = if self.y == usize::MAX { 0 } else { self.y + 1 };
        } else {
            self.x += 1;
            self.y = 0;
        }
        self.needs_switch = array[self.y] > array[self.y + 1];
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.swap(self.y, self.y + 1);
        self.needs_switch = false;
    }

    fn reset_state(&mut self) {
        self.x = 0;
        self.y = usize::MAX;
    }
}

#[cfg(test)]
mod tests_bubble {
    use crate::algorithms::BubbleSort;
    use crate::algorithms::Sorter;

    #[test]
    fn get_comparing() {
        let mut sorter = BubbleSort::new();
        assert_eq!(sorter.get_comparing(), (usize::MAX, usize::MAX));
        sorter.y = 0;
        assert_eq!(sorter.get_comparing(), (0, 1));
    }

    // In BubbleSort, the numbers being compared are the same as the ones switching, so we don't test that

    #[test]
    fn get_state() {
        let sorter = BubbleSort::new();
        assert_eq!(sorter.get_state(), (0, usize::MAX));
    }

    #[test]
    fn run() {
        let mut sorter = BubbleSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 6];

        sorter.run(&mut arr);

        let expected: Vec<usize> = vec![2, 5, 6];

        assert_eq!(arr, expected);

        // We must reset after running
        assert_eq!((sorter.x, sorter.y), (0, usize::MAX))
    }

    #[test]
    fn step() {
        let mut sorter = BubbleSort::new();
        let mut arr: Vec<usize> = vec![5, 2];

        // Selects indexes 0 and 1 for comparing
        sorter.step(&mut arr);
        assert_eq!(vec![5, 2], arr);
        assert_eq!(sorter.get_comparing(), (0, 1));

        // Swaps 2 and 5
        sorter.step(&mut arr);
        assert_eq!(vec![2, 5], arr);
        assert_eq!(sorter.get_comparing(), (0, 1));
    }

    // TODO: Test modify_state

    #[test]
    fn test_switch() {
        let mut sorter = BubbleSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 6];

        sorter.y = 0;
        sorter.switch(&mut arr);
        assert_eq!(arr, vec![2, 5, 6]);
        assert!(!sorter.needs_switch);
    }

    #[test]
    fn reset_state() {
        let mut sorter = BubbleSort::new();

        sorter.reset_state();

        assert_eq!((sorter.x, sorter.y), (0, usize::MAX));
    }
}
