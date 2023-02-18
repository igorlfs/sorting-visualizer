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

    fn special(&self) -> (usize, usize) {
        self.special
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
        *self = SelectionSort::new();
    }
}

#[cfg(test)]
mod tests {

    use super::SelectionSort;
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
            let mut sorter = SelectionSort::new();
            let mut array = util::gen_random_vector(FLOOR, CEIL, SIZE);

            let mut expected = array.clone();
            expected.sort();

            sorter.run(&mut array);

            assert_eq!(array, expected);
        }
    }
}
