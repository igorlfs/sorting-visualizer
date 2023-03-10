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
    use super::BubbleSort;
    use crate::{
        algorithms::{
            Sorter, {CEIL, FLOOR, REPETITIONS, SIZE},
        },
        util,
    };

    #[test]
    fn run() {
        for _ in 0..REPETITIONS {
            let mut sorter = BubbleSort::new();
            let mut array = util::gen_random_vector(FLOOR, CEIL, SIZE);

            let mut expected = array.clone();
            expected.sort();

            sorter.run(&mut array);

            assert_eq!(array, expected);
        }
    }
}
