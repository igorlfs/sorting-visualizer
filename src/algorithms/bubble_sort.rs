use super::{Reasons, Sorter};

pub struct BubbleSort {
    x: usize,
    y: usize,
    needs_switch: bool,
    reason: Reasons,
}

impl Sorter for BubbleSort {
    fn new() -> BubbleSort {
        BubbleSort {
            x: 0,
            y: usize::MAX,
            needs_switch: false,
            reason: Reasons::Comparing,
        }
    }

    fn special(&self) -> (usize, usize) {
        if self.y != usize::MAX {
            (self.y, self.y + 1)
        } else {
            (usize::MAX, usize::MAX)
        }
    }

    fn reason(&self) -> super::Reasons {
        self.reason
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
        self.reason = Reasons::Comparing;
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.swap(self.y, self.y + 1);
        self.reason = Reasons::Switching;
        self.needs_switch = false;
    }

    fn reset_state(&mut self) {
        *self = BubbleSort::new();
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::bubble_sort::BubbleSort;
    use crate::algorithms::Sorter;

    #[test]
    fn run() {
        let mut sorter = BubbleSort::new();
        let mut arr = vec![5, 2, 6];

        sorter.run(&mut arr);

        let expected: Vec<usize> = vec![2, 5, 6];

        assert_eq!(arr, expected);

        // We must reset after running
        assert_eq!((sorter.x, sorter.y), (0, usize::MAX))
    }

    #[test]
    fn step() {
        let mut sorter = BubbleSort::new();
        let mut arr = vec![5, 2];

        // Selects indexes 0 and 1 for comparing
        sorter.step(&mut arr);
        assert_eq!(vec![5, 2], arr);
        assert_eq!(sorter.special(), (0, 1));

        // Swaps 2 and 5
        sorter.step(&mut arr);
        assert_eq!(vec![2, 5], arr);
        assert_eq!(sorter.special(), (0, 1));
    }

    #[test]
    fn test_switch() {
        let mut sorter = BubbleSort::new();
        let mut arr = vec![5, 2, 6];

        sorter.y = 0;
        sorter.switch(&mut arr);
        assert_eq!(arr, vec![2, 5, 6]);
        assert!(!sorter.needs_switch);
    }
}
