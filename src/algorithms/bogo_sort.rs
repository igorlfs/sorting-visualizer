use super::{Reasons, Sorter};
use rand::thread_rng;
use rand::seq::SliceRandom;

fn is_sorted(array : &[usize]) -> bool {
    array.windows(2).all(|w| w[0] <= w[1])
}

pub struct BogoSort{
    reason: Reasons,
    needs_shuffle: bool
}

impl Sorter for BogoSort {
    fn new() -> BogoSort {
        BogoSort {
            reason: Reasons::Comparing,
            needs_shuffle: false,
        }
    }

    fn get_special(&self) -> (usize, usize) {
        if self.y != usize::MAX {
            (self.y, self.y + 1)
        } else {
            (usize::MAX, usize::MAX)
        }
    }

    fn get_reason(&self) -> super::Reasons {
        self.reason
    }

    fn get_state(&self) -> (usize, usize) {
        self.curr_array
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
        if is_sorted(array) {
            return true;
        }
        self.needs_shuffle = true;
        self.reason = Reasons::Comparing;
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.shuffle(&mut thread_rng()); 
        self.needs_shuffle = false;
        self.reason = Reasons::Switching;
    }

    fn reset_state(&mut self) {
        self.x = 0;
        self.y = usize::MAX;
    }
}

