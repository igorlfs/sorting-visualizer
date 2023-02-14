use std::cmp::min;

use super::{Reasons, Sorter};

pub struct MergeSort {
    power: usize,
    slice: usize,
    special: (usize, usize),
    reason: Reasons,
}

impl MergeSort {
    fn merge(array: &mut [usize], from: usize, mid: usize, to: usize) {
        let mut temp: Vec<usize> = array.to_owned();
        let mut k = from;
        let mut i = from;
        let mut j = mid + 1;
        while i <= mid && j <= to {
            if array[i] < array[j] {
                temp[k] = array[i];
                i += 1;
            } else {
                temp[k] = array[j];
                j += 1;
            }
            k += 1;
        }
        while i < array.len() && i <= mid {
            temp[k] = array[i];
            k += 1;
            i += 1;
        }
        array[from..(to + 1)].copy_from_slice(&temp[from..(to + 1)]);
    }
}

impl Sorter for MergeSort {
    fn new() -> Self {
        MergeSort {
            power: 1,
            slice: usize::MAX,
            reason: Reasons::Limits,
            special: (usize::MAX, usize::MAX),
        }
    }

    fn get_special(&self) -> (usize, usize) {
        self.special
    }

    fn get_reason(&self) -> Reasons {
        self.reason
    }

    fn get_state(&self) -> (usize, usize) {
        (usize::MAX, usize::MAX)
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        let size: usize = array.len();
        if self.slice == usize::MAX {
            self.slice = 0;
        } else if self.slice < size {
            self.slice += 2 * self.power;
        }
        if self.slice >= size {
            self.slice = 0;
            self.power *= 2;
        }
        self.switch(array);
        self.modify_state(array)
    }

    fn modify_state(&mut self, array: &[usize]) -> bool {
        self.power > array.len()
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        let from = self.slice;
        let mid = self.slice + self.power - 1;
        let to = min(self.slice + 2 * self.power - 1, array.len() - 1);
        self.special = (from, to);
        MergeSort::merge(array, from, mid, to);
    }

    fn reset_state(&mut self) {
        self.power = 1;
        self.slice = usize::MAX;
        self.special = (usize::MAX, usize::MAX);
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::Reasons;

    use super::{MergeSort, Sorter};

    #[test]
    fn run() {
        let mut sorter = MergeSort::new();
        let mut arr: Vec<usize> = vec![6, 5, 3, 1, 8, 7, 2, 4];

        sorter.run(&mut arr);

        let expected: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(arr, expected);
    }

    #[test]
    fn merge() {
        let mut arr = vec![9, 13, 10, 11];
        MergeSort::merge(&mut arr, 0, 1, 3);

        let expected = vec![9, 10, 11, 13];
        assert_eq!(arr, expected);
    }

    #[test]
    fn reset_state() {
        let mut sorter = MergeSort {
            power: 99,
            slice: 1,
            reason: Reasons::Limits,
            special: (2, 3),
        };

        sorter.reset_state();

        assert_eq!(sorter.power, 1);
        assert_eq!(sorter.slice, usize::MAX);
        assert_eq!(sorter.special, (usize::MAX, usize::MAX));
    }
}
