use super::{Reasons, Sorter};
const VECTOR_SIZE: usize = 20; 

pub struct QuickSort {
    x: usize,
    y: usize,
    pivot_ptr: usize,
    needs_switch: bool,
    reason: Reasons,
    special: (usize, usize),
    partition_stack: Vec<(usize, usize)>,
    moving_left_ptr: bool,
}

impl Sorter for QuickSort {
    fn new() -> QuickSort {
        QuickSort {
            x: 0,
            y: VECTOR_SIZE - 1,
            pivot_ptr: (VECTOR_SIZE / 2) - 1,
            needs_switch: false,
            reason: Reasons::Comparing,
            special: (usize::MAX, usize::MAX),
            partition_stack: vec![],
            moving_left_ptr: true,
         }
    }

    fn special(&self) -> (usize, usize) {
        (self.x, self.y)
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
        if self.partition_stack.len() == 0 {
            return true;
        }

        if self.moving_left_ptr {
            if array[self.x] < array[self.pivot_ptr] {
                self.x += 1;
            } else {
                self.moving_left_ptr = false;
            }
        } else {
            if array[self.y] >= array[self.pivot_ptr] {
                self.y -= 1;
            } else {
                self.moving_left_ptr = true;
                self.needs_switch = true;
            }
        }

        false
   }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.swap(self.x, self.y);
    }

    fn reset_state(&mut self) {
        *self = QuickSort::new();
    }
}
