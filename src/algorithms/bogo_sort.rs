use super::{Reasons, Sorter};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct BogoSort {
    reason: Reasons,
    needs_shuffle: bool,
    shuffled: bool,
    curr: usize,
    x: usize,
    y: usize,
}

impl Sorter for BogoSort {
    fn new() -> BogoSort {
        BogoSort {
            reason: Reasons::Comparing,
            needs_shuffle: false,
            shuffled: false,
            curr: 1,
            x: 0,
            y: 1,
        }
    }

    fn get_reason(&self) -> super::Reasons {
        self.reason
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_special(&self) -> (usize, usize) {
        if self.shuffled {
            return (usize::MAX, usize::MAX);
        }
        if self.curr != 1 {
            return (self.x, self.y);
        }
        (usize::MAX, usize::MAX)
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.needs_shuffle {
            self.switch(array);
        } else {
            return self.modify_state(array);
        }
        false
    }

    fn modify_state(&mut self, array: &[usize]) -> bool {
        if self.curr == array.len() {
            return true;
        }
        if self.y < array.len() - 1 && !self.shuffled {
            self.x = self.curr - 1;
            self.y = self.curr;
            self.curr += 1;
        }
        if self.shuffled {
            self.x = 0;
            self.y = 1;
            self.curr = 2;
        }
        self.needs_shuffle = array[self.y] < array[self.x];
        self.reason = Reasons::Comparing;
        self.shuffled = false;
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.shuffle(&mut thread_rng());
        self.shuffled = true;
        self.needs_shuffle = false;
        self.curr = 2;
    }

    fn reset_state(&mut self) {
        self.needs_shuffle = false;
        self.shuffled = false;
        self.x = 0;
        self.y = 1;
        self.curr = 1;
    }
}
