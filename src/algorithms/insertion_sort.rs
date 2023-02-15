use super::{Reasons, Sorter};

pub struct InsertionSort {
    x: usize,
    y: usize,
    curr: usize,
    needs_switch: bool,
    reason: Reasons,
    switched: bool,
}

impl Sorter for InsertionSort {
    fn new() -> InsertionSort {
        InsertionSort {
            x: 0,
            y: 1,
            curr: 1,
            needs_switch: false,
            reason: Reasons::Comparing,
            switched: false,
        }
    }

    fn special(&self) -> (usize, usize) {
        if self.curr != 1 {
            return (self.x, self.y);
        }
        (usize::MAX, usize::MAX)
    }

    fn reason(&self) -> super::Reasons {
        self.reason
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.needs_switch {
            self.switch(array)
        } else {
            return self.modify_state(array);
        }
        false
    }

    fn modify_state(&mut self, array: &[usize]) -> bool {
        if self.curr == array.len() && !self.switched {
            return true;
        }
        self.reason = Reasons::Comparing;
        if self.switched && self.y > 1 {
            self.x -= 1;
            self.y -= 1;
        } else {
            self.x = self.curr - 1;
            self.y = self.curr;
            self.curr += 1;
        }
        self.switched = false;
        self.needs_switch = self.y < array.len() && array[self.y] < array[self.x] && self.y > 0;
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.swap(self.y, self.x);
        self.reason = Reasons::Switching;
        self.needs_switch = false;
        self.switched = true;
    }

    fn reset_state(&mut self) {
        *self = InsertionSort::new();
    }
}

#[cfg(test)]
mod tests {
    use super::InsertionSort;
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
            let mut sorter = InsertionSort::new();
            let mut array = util::gen_random_vector(FLOOR, CEIL, SIZE);

            let mut expected = array.clone();
            expected.sort();

            sorter.run(&mut array);

            assert_eq!(array, expected);
        }
    }
}
