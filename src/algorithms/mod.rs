/// A Sorter is a sorting algorithm split in two stages: the `step` and the `state`.
/// A `step` consists in comparing two elements and swap them if necessary.
/// A `state` controls the variables that the "step" is going to use.
pub trait Sorter {
    fn new() -> Self
    // The Compiler will complain if we don't do this
    where
        Self: Sized;

    /// Modifying the state is analogous to stepping in a loop.
    /// Returns true if all states have been traversed.
    fn modify_state(&mut self, len: usize) -> bool;

    /// Returns the indexes currently being compared.
    fn get_comparing(&self) -> (usize, usize);

    fn get_state(&self) -> (usize, usize);

    /// Runs the algorithm all at once.
    fn run(&mut self, array: &mut Vec<u32>);

    /// Takes a single step in running the algorithm.
    /// Returns true if the step swaps numbers.
    fn step(&self, array: &mut Vec<u32>) -> bool;

    /// Set the Sorter's state to it's initial state.
    fn reset_state(&mut self);
}

pub struct BubbleSort {
    x: usize,
    y: usize,
}

impl Sorter for BubbleSort {
    fn new() -> BubbleSort {
        BubbleSort { x: 0, y: 0 }
    }

    fn modify_state(&mut self, len: usize) -> bool {
        if self.y < len - 2 - self.x {
            self.y += 1;
        } else {
            self.x += 1;
            self.y = 0;
        }
        if self.x == len - 1 {
            return true;
        }
        false
    }

    fn run(&mut self, array: &mut Vec<u32>) {
        let len: usize = array.len();
        loop {
            self.step(array);
            if self.modify_state(len) {
                break;
            }
        }
        self.reset_state();
    }

    fn step(&self, array: &mut Vec<u32>) -> bool {
        if array[self.y] > array[self.y + 1] {
            array.swap(self.y, self.y + 1);
            return true;
        }
        false
    }

    fn get_comparing(&self) -> (usize, usize) {
        (self.y, self.y + 1)
    }

    fn reset_state(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::BubbleSort;
    use crate::algorithms::Sorter;

    #[test]
    fn run() {
        let mut sorter = BubbleSort::new();
        let mut arr: Vec<u32> = vec![5, 2, 3, 4, 1];

        sorter.run(&mut arr);

        let expected: Vec<u32> = vec![1, 2, 3, 4, 5];

        assert_eq!(arr, expected);

        // We must reset after running
        assert_eq!((sorter.x, sorter.y), (0, 0))
    }

    /// In BubbleSort, we first compare (0,1), (1,2) .. (n,n - 1)
    /// And the go back with (0,1), .. (n,n - 2) and so on.
    #[test]
    fn modify_state() {
        let mut sorter = BubbleSort::new();
        let len: usize = 4;

        // Comparing should be (1,2)
        sorter.modify_state(len);
        assert_eq!(sorter.y, 1);

        // Comparing should be (2,3)
        sorter.modify_state(len);
        assert_eq!(sorter.y, 2);

        // Comparing should be (0,1)
        sorter.modify_state(len);
        assert_eq!(sorter.y, 0);

        // Comparing should be (1,2)
        sorter.modify_state(len);
        assert_eq!(sorter.y, 1);

        // Comparing should be (0,1)
        sorter.modify_state(len);
        assert_eq!(sorter.y, 0);
    }

    #[test]
    fn reset_state() {
        let mut sorter = BubbleSort::new();

        sorter.reset_state();

        assert_eq!((sorter.x, sorter.y), (0, 0));
    }

    #[test]
    fn get_comparing() {
        let sorter = BubbleSort::new();
        assert_eq!((0, 1), sorter.get_comparing());
    }

    #[test]
    fn step() {
        let sorter = BubbleSort::new();
        let mut arr: Vec<u32> = vec![5, 2];

        // Swap 2 and 5
        assert!(sorter.step(&mut arr));
        // Correct position
        assert!(!sorter.step(&mut arr));
    }
}
