use super::{Reasons, Sorter};

pub struct HeapSort {
    index: usize,
    special: (usize, usize),
    swapped: bool,
    root: usize,
    start: usize,
    reason: Reasons,
}

impl HeapSort {
    /// Take a single step in a heapification.
    /// Returns false when a swap happens

    // Thanks for Pavankumar for the code that inspired our heapify: https://chercher.tech/rust/heap-sort-rust
    fn sift_down(&mut self, array: &mut [usize], end_index: usize) -> bool {
        let mut child = self.root * 2 + 1;

        if child > end_index {
            return true;
        } else {
            if child < end_index && array[child] < array[child + 1] {
                child += 1;
            }

            if array[self.root] < array[child] {
                self.swap(array, self.root, child, Reasons::Comparing);
            } else {
                return true;
            }
        }

        false
    }

    /// Swaps (a,b) in array, mark them as special, and update reason
    fn swap(&mut self, array: &mut [usize], a: usize, b: usize, reason: Reasons) {
        array.swap(a, b);
        self.reason = reason;
        self.special = (a, b);
        self.root = b;
    }
}

impl Sorter for HeapSort {
    fn new() -> Self {
        HeapSort {
            index: usize::MAX,
            special: (usize::MAX, usize::MAX),
            swapped: false,
            root: usize::MAX,
            start: usize::MAX,
            reason: Reasons::Comparing,
        }
    }

    fn special(&self) -> (usize, usize) {
        self.special
    }

    fn reason(&self) -> super::Reasons {
        self.reason
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        let len = array.len();

        // "Start" tracks initial heap construction
        if self.start == usize::MAX {
            self.start = len / 2;
            // The root is essentially a modifiable start
            // That's used inside sift_down()
            self.root = self.start;
        }

        // "Index" tracks heapification after initial construction
        // We only initialize it after the heap is constructed
        if self.index == usize::MAX {
            // We "ignore" when a heapification doesn't switch anything
            // by using a while instead of an if
            while self.sift_down(array, len - 1) {
                if self.start == 0 {
                    self.index = len - 1;
                    // The last step after initialization requires a switch,
                    // otherwise the step will do nothing visible by user
                    self.switch(array);
                    return false;
                }
                self.start -= 1;
                self.root = self.start;
            }
            return false;
        }
        // We finally can sort using "index"
        // We use different "reasons" for swaps and heapification
        self.switch(array);
        // This function has no semantics in this algorithm
        self.modify_state(array)
    }

    fn modify_state(&mut self, _array: &[usize]) -> bool {
        self.index == 0
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        if self.swapped {
            let end = self.index - 1;
            if self.sift_down(array, end) {
                self.index -= 1;
                self.swapped = false;
            }
        }

        // We don't use an else because as soon as we set swapped to false we want to swap
        if !self.swapped {
            self.swap(array, self.index, 0, Reasons::Switching);
            self.swapped = true;
        }
    }

    fn reset_state(&mut self) {
        *self = HeapSort::new();
    }
}

#[cfg(test)]
mod tests {
    use super::HeapSort;
    use crate::{
        algorithms::{
            constants::{CEIL, FLOOR, REPETITIONS, SIZE},
            Sorter,
        },
        util,
    };

    #[test]
    fn run() {
        for _ in 0..REPETITIONS {
            let mut sorter = HeapSort::new();
            let mut array = util::gen_random_vector(FLOOR, CEIL, SIZE);

            let mut expected = array.clone();
            expected.sort();

            sorter.run(&mut array);

            assert_eq!(array, expected);
        }
    }
}
