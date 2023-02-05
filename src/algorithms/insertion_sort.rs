use super::{Reasons, Sorter};

pub struct InsertionSort {
    x: usize,
    y: usize,
    needs_switch: bool,
    reason: Reasons,
}

impl Sorter for InsertionSort {
    fn new() -> InsertionSort {
        InsertionSort {
            x: 0,
            y: 1,
            needs_switch: false,
            reason: Reasons::Comparing,
        }
    }

    fn get_special(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_reason(&self) -> super::Reasons {
        self.reason
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
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
        if self.y == array.len() {
            return true;
        }

        self.needs_switch = array[self.y] < array[self.x] && self.y > 0;
        if !self.needs_switch {
            self.y += 1;
            self.x += 1;
        }
        self.reason = Reasons::Comparing;
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        self.reason = Reasons::Switching;
        array.swap(self.y, self.x);
        self.needs_switch = false;
        if self.y != 1 {
            self.x -= 1;
            self.y -= 1;
        }
    }

    fn reset_state(&mut self) {
        self.x = 0;
        self.y = 1;
    }
}
