use super::{Reasons, Sorter};

pub struct QuickSort {
    x: usize,
    y: usize,
    pivot: usize,
    needs_switch: bool,
    reason: Reasons,
}

impl Sorter for QuickSort {
    fn new() -> QuickSort {
        QuickSort {
            x: 0,
            y: usize::MAX,
            pivot: 2,
            needs_switch: false,
            reason: Reasons::Comparing,
        }
    }

    fn special(&self) -> (usize, usize) {
       
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
       false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
    }

    fn reset_state(&mut self) {
        *self = QuickSort::new();
    }
}

