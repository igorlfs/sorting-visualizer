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

    fn get_special(&self) -> (usize, usize) {
        (self.x, self.y)
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
            self.switch(array)
        } else {
            return self.modify_state(array);
        }
        false
    }

    fn modify_state(&mut self, array: &[usize]) -> bool {
        if self.curr == array.len() + 1 {
            return true;
        }
        self.reason = Reasons::Comparing;
        if self.switched && self.y > 1{
            self.x -= 1;
            self.y -= 1;
        } else{
            self.x  = self.curr - 1;
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
        self.x = 0;
        self.y = 1;
    }
}
