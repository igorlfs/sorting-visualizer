use super::{Reasons, Sorter};
const VECTOR_SIZE: usize = 20; 

fn median(x: usize, y: usize, z: usize, array: &[usize]) -> usize {
    if array[x] >= array[y]{
        if array[y] >= array[z] {
            return y;
        } else {
            return z;
        }
    }  

    if array[y] >= array[z] {
        if array[z] >= array[x] {
            return z;
        } else{
            return x;
        }
    } 

    if array[z] >= array[x] {
        if array[x] >= array[y] {
            return x;
        } else {
            return y;
        }
    }

    usize::MAX
}

pub struct QuickSort {
    x: usize,
    y: usize,
    pivot_ptr: usize,
    needs_switch: bool,
    reason: Reasons,
    special: (usize, usize),
    partition_stack: Vec<(usize, usize)>,
    moving_left_ptr: bool,
    moving_pivot: bool,
    returning_pivot: bool,
    curr_partition_start: usize,
    curr_partition_end: usize,
}

impl Sorter for QuickSort {
    fn new() -> QuickSort {
        QuickSort {
            x: 0,
            y: VECTOR_SIZE - 2,
            pivot_ptr: usize::MAX,
            needs_switch: false,
            reason: Reasons::Comparing,
            special: (usize::MAX, usize::MAX),
            partition_stack: vec![(0, VECTOR_SIZE - 1)],
            moving_left_ptr: true,
            moving_pivot: true,
            returning_pivot: false,
            curr_partition_start: 0,
            curr_partition_end: VECTOR_SIZE - 1,
         }
    }

    fn special(&self) -> (usize, usize) {
        if self.pivot_ptr == usize::MAX {
            return (usize::MAX, usize::MAX);
        }
        self.special
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
        if self.pivot_ptr != usize::MAX { println!("{}", array[self.pivot_ptr])};
        println!("{} {}", self.curr_partition_start, self.curr_partition_end);
        if self.partition_stack.len() == 0 {
            return true;
        }
        
        if self.pivot_ptr == usize::MAX {
           self.special = (self.pivot_ptr, self.pivot_ptr);
           (self.curr_partition_start, self.curr_partition_end) = *self.partition_stack.last().unwrap();
           self.pivot_ptr = median(self.curr_partition_start, (self.curr_partition_end - self.curr_partition_start)/2 + self.curr_partition_start, self.curr_partition_end, array);
           return false;
        }

        if self.moving_pivot {
            self.special = (self.pivot_ptr, self.curr_partition_end);
            self.needs_switch = true;
            return false;
        }

        self.special = (self.x, self.y);
        
        // Se ponteiros se cruzarem
        if self.y < self.x {
            self.moving_pivot = true;
            self.returning_pivot = true;
            return false;
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
        if self.moving_pivot {
            if self.returning_pivot {
                array.swap(self.x, self.pivot_ptr);
                self.moving_pivot = true;
                self.returning_pivot = false;
                

                if self.partition_stack.len() > 0 {
                    if *self.partition_stack.last().unwrap() == (0, VECTOR_SIZE - 1){
                        self.partition_stack.pop();
                    }
                }
                
                if self.x - self.curr_partition_start >= 2 {
                    self.partition_stack.push((self.curr_partition_start, self.x - 1));
                } 

                if self.curr_partition_end - self.x >= 2 {
                    self.partition_stack.push((self.x + 1, self.curr_partition_end));
                }

                // Aqui, provavelmente checar antes se existe algo na stack
                if self.partition_stack.len() > 0 {
                    (self.curr_partition_start, self.curr_partition_end) = self.partition_stack.pop().unwrap();
                }
                (self.x, self.y) = (self.curr_partition_start, self.curr_partition_end);
                self.pivot_ptr = median(self.x, (self.x + self.y) / 2, self.y, array);
            } else {
                array.swap(self.pivot_ptr, self.curr_partition_end);
                self.pivot_ptr = self.curr_partition_end;

                self.needs_switch = false;
                self.moving_pivot = false;
            }
            return;
        }
        array.swap(self.x, self.y);
        self.needs_switch = false;
    }

    fn reset_state(&mut self) {
        *self = QuickSort::new();
    }
}
