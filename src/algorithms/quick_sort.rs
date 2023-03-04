use super::{Reasons, Sorter};
const VECTOR_SIZE: usize = 20;

fn median(x: usize, y: usize, z: usize, array: &[usize]) -> usize {
    if (array[x] > array[y]) ^ (array[x] > array[z]) {
        return x;
    }

    if (array[y] > array[x]) ^ (array[y] > array[z]) {
        return y;
    }
    return z;
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
        if self.partition_stack.len() == 0 {
            return true;
        }

        self.reason = Reasons::Comparing;

        if self.pivot_ptr == usize::MAX {
            self.special = (self.pivot_ptr, self.pivot_ptr);
            (self.curr_partition_start, self.curr_partition_end) =
                *self.partition_stack.last().unwrap();
            self.pivot_ptr = median(
                self.curr_partition_start,
                (self.curr_partition_end - self.curr_partition_start) / 2
                    + self.curr_partition_start,
                self.curr_partition_end,
                array,
            );
            return false;
        }

        if self.moving_pivot {
            if self.returning_pivot {
                self.special = (self.pivot_ptr, self.x);
            } else {
                self.special = (self.pivot_ptr, self.curr_partition_end);
            }
            self.needs_switch = true;
            return false;
        }

        self.special = (self.x, self.y);

        // Se ponteiros se cruzarem
        if self.y < self.x {
            self.moving_pivot = true;
            self.returning_pivot = true;
            self.needs_switch = true;
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
                if self.y == self.curr_partition_start {
                    self.needs_switch = true;
                    self.returning_pivot = true;
                    self.moving_pivot = true;
                } else {
                    self.y -= 1;
                }
            } else {
                self.moving_left_ptr = true;
                self.needs_switch = true;
            }
        }
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        self.reason = Reasons::Switching;
        if self.moving_pivot {
            if self.returning_pivot {
                self.special = (self.x, self.pivot_ptr);
                array.swap(self.x, self.pivot_ptr);
                self.moving_pivot = true;
                self.returning_pivot = false;

                if self.x - self.curr_partition_start >= 2 {
                    self.partition_stack
                        .push((self.curr_partition_start, self.x - 1));
                }

                if self.curr_partition_end - self.x >= 2 {
                    self.partition_stack
                        .push((self.x + 1, self.curr_partition_end));
                }

                if self.partition_stack.len() > 0 {
                    if *self.partition_stack.last().unwrap() == (0, VECTOR_SIZE - 1) {
                        self.partition_stack.pop();
                    }
                }

                if self.partition_stack.len() > 0 {
                    (self.curr_partition_start, self.curr_partition_end) =
                        self.partition_stack.pop().unwrap();
                }
                (self.x, self.y) = (self.curr_partition_start, self.curr_partition_end);
                self.pivot_ptr = median(self.x, (self.x + self.y) / 2, self.y, array);
            } else {
                if self.partition_stack.len() == 0 {
                    self.needs_switch = false;
                    return;
                }
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

#[cfg(test)]
mod tests {
    use super::{QuickSort, VECTOR_SIZE};
    use crate::{
        algorithms::{
            constants::{FLOOR, REPETITIONS, CEIL},
            Sorter,
        },
        util,
    };

    #[test]
    fn run() {
        for _ in 0..REPETITIONS {
            let mut sorter = QuickSort::new();
            let mut array = util::gen_random_vector(FLOOR, CEIL, VECTOR_SIZE);
            let mut expected = array.clone();
            expected.sort();

            sorter.run(&mut array);

            assert_eq!(array, expected);
        }
    }
}
