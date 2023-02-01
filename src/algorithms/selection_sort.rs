use super::{Reasons, Sorter};

pub struct SelectionSort {
    x: usize,
    y: usize,
    min: usize,
    needs_switch: bool,
    special: (usize, usize),
    reason: Reasons,
}

impl Sorter for SelectionSort {
    fn new() -> SelectionSort {
        SelectionSort {
            x: 0,
            y: 1,
            min: 0,
            needs_switch: false,
            special: (usize::MAX, usize::MAX),
            reason: Reasons::Comparing,
        }
    }

    fn get_special(&self) -> (usize, usize) {
        self.special
    }

    fn get_reason(&self) -> super::Reasons {
        self.reason
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn run(&mut self, array: &mut Vec<usize>) {
        loop {
            if self.step(array) {
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
        self.special = (self.y, self.min);
        self.reason = Reasons::Comparing;
        if array[self.y] < array[self.min] {
            self.min = self.y;
        }
        self.y += 1;
        if self.y == array.len() {
            self.needs_switch = true;
        }
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        self.special = (self.x, self.min);
        array.swap(self.x, self.min);
        self.reason = Reasons::Switching;
        self.x += 1;
        self.min = self.x;
        self.y = self.x + 1;
        self.needs_switch = false;
    }

    fn reset_state(&mut self) {
        self.x = 0;
        self.y = 1;
        self.min = 0;
        self.special = (usize::MAX, usize::MAX);
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectionSort, Sorter};

    #[test]
    fn run() {
        let mut sorter = SelectionSort::new();
        let mut arr: Vec<usize> = vec![9, 2, 8, 10, 5];

        sorter.run(&mut arr);

        let expected: Vec<usize> = vec![2, 5, 8, 9, 10];

        assert_eq!(arr, expected);
    }

    #[test]
    fn step() {
        let mut sorter = SelectionSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 3];

        // Selects indexes 0 and 1 for comparing
        sorter.step(&mut arr);
        assert_eq!(sorter.get_special(), (1, 0));

        // Selects indexes 1 and 2 for comparing
        sorter.step(&mut arr);
        assert_eq!(sorter.get_special(), (2, 1));

        // Swaps 2 and 5
        sorter.step(&mut arr);
        assert_eq!(vec![2, 5, 3], arr);
        assert_eq!(sorter.get_special(), (0, 1));
    }

    // TODO: Test modify_state

    #[test]
    fn test_switch() {
        let mut sorter = SelectionSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 6];

        sorter.min = 1;
        sorter.switch(&mut arr);
        assert_eq!(arr, vec![2, 5, 6]);
        assert_eq!(sorter.x, 1);
        assert_eq!(sorter.x, sorter.min);
        assert_eq!(sorter.y, 2);
        assert!(!sorter.needs_switch);
    }
}
