use super::{Reasons, Sorter};
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct BogoSort{
    reason: Reasons,
    needs_shuffle: bool,
    x: usize,
    y: usize
}

impl Sorter for BogoSort {
    fn new() -> BogoSort {
        BogoSort {
            reason: Reasons::Comparing,
            needs_shuffle: false,
            x: 0,
            y: 1
        }
    }

    fn get_reason(&self) -> super::Reasons {
        self.reason
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_special(&self) -> (usize, usize) {
        (self.x, self.y)
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
        if self.y == array.len() {
            return true;
        }
        self.needs_shuffle = array[self.y] < array[self.x];
        if !self.needs_shuffle {
            self.x += 1;
            self.y += 1;
        }
        self.reason = Reasons::Comparing;
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.shuffle(&mut thread_rng()); 
        self.x = 0;
        self.y = 1;
        self.needs_shuffle = false;
        self.reason = Reasons::Switching;
    }

    fn reset_state(&mut self) {
        self.needs_shuffle = false; 
    }
}

