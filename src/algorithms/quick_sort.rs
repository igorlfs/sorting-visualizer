use super::{Reasons, Sorter};
const VECTOR_SIZE: usize = 20; 

pub struct QuickSort {
    x: usize,
    y: usize,
    pivot_ptr: usize,
    needs_switch: bool,
    moving_left_ptr: bool,
    left_special: usize,
    right_special: usize,
    moving_pivot: bool,
    reason: Reasons,
    partition_stack: Vec<(usize, usize)>,
    curr_partition_start: usize,
    curr_partition_end: usize,
}

impl Sorter for QuickSort {
    fn new() -> QuickSort {
        QuickSort {
            x: 0,
            y: VECTOR_SIZE - 2, // deve ser partition end - 1
            pivot_ptr: (VECTOR_SIZE / 2) - 1,
            needs_switch: false,
            moving_left_ptr: true,
            left_special: usize::MAX,
            right_special: usize::MAX,
            moving_pivot: true,
            reason: Reasons::Comparing,
            partition_stack: vec![],
            curr_partition_start: 0,
            curr_partition_end: VECTOR_SIZE - 1,
        }
    }

    fn special(&self) -> (usize, usize) {
        if self.moving_pivot {
            return (self.pivot_ptr, self.curr_partition_end);
        }
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
        println!("{}", array[self.pivot_ptr]);
        println!("{:?}", self.partition_stack);
        if self.y == 0 {
            return true;
        } 

        if self.y < self.x {
            self.moving_pivot = true;
            self.needs_switch = true;
            self.partition_stack.push((self.curr_partition_start, self.x - 1));
            self.partition_stack.push((self.x + 1, self.curr_partition_end));
            return false;
        }

        // Se for início da iteração, movendo pivot 
        if self.moving_pivot {
            self.reason = Reasons::Comparing;
            self.needs_switch = true;
            return false;
        }
        // Movimentação dos ponteiros pela partição
        if self.moving_left_ptr{
            if array[self.x] >= array[self.pivot_ptr]{
                self.left_special = self.x;
                self.moving_left_ptr = false;
            } else {
                self.x += 1;
            }
        } else {
            if array[self.y] < array[self.pivot_ptr] {
                self.right_special = self.y;
            } else {
                self.y -= 1;
            }
        }
        if self.left_special != usize::MAX && self.right_special != usize::MAX {
            self.needs_switch = true;
        }
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        if self.moving_pivot {
            if self.y < self.x {
                array.swap(self.pivot_ptr, self.x);
                (self.x, self.y) = self.partition_stack.pop().unwrap();
                self.pivot_ptr = (self.y - self.x) / 2;
            } else {
                array.swap(self.pivot_ptr, self.curr_partition_end);
                self.pivot_ptr = self.curr_partition_end;
                self.moving_pivot = false;
            }
            self.needs_switch = false;
            return;
        }
        array.swap(self.x, self.y);
        self.needs_switch = false;
        self.moving_left_ptr = true;
        self.left_special = usize::MAX;
        self.right_special = usize::MAX;
    }

    fn reset_state(&mut self) {
        *self = QuickSort::new();
    }
}

