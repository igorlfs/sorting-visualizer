use super::{Reasons, Sorter};

pub struct InsertionSort {
    x: usize,
    y: usize,
    current: usize,
    needs_switch: bool,
    reason: Reasons,
    switched: bool,
}

impl Sorter for InsertionSort {
    fn new() -> InsertionSort {
        InsertionSort {
            x: 0,
            y: 1,
            current: 1,
            needs_switch: false,
            reason: Reasons::Comparing,
            switched: false,
        }
    }

    fn get_special(&self) -> (usize, usize) {
        if self.current != 1 {
            return (self.x, self.y);
        }
        (usize::MAX, usize::MAX)
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
        if self.current == array.len() && !self.switched {
            return true;
        }
        self.reason = Reasons::Comparing;
        if self.switched && self.y > 1 {
            self.x -= 1;
            self.y -= 1;
        } else {
            self.x = self.current - 1;
            self.y = self.current;
            self.current += 1;
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
        self.x = 0;
        self.y = 1;
        self.current = 1;
    }
}
