use std::cmp::min;

use super::{Reasons, Sorter};

#[derive(PartialEq)]
enum State {
    Init,
    Comparing,
    Merging,
    Over,
}

pub struct MergeSort {
    power: usize,
    slice: usize,
    special: (usize, usize),
    reason: Reasons,
    state: State,
    temp: Vec<usize>,
    i: usize,
    j: usize,
    k: usize,
    merge_tracker: usize,
}

impl Sorter for MergeSort {
    fn new() -> Self {
        MergeSort {
            power: 1,
            slice: usize::MAX,
            reason: Reasons::Comparing,
            special: (usize::MAX, usize::MAX),
            state: State::Init,
            temp: vec![],
            i: usize::MAX,
            j: usize::MAX,
            k: usize::MAX,
            merge_tracker: 0,
        }
    }

    fn special(&self) -> (usize, usize) {
        self.special
    }

    fn reason(&self) -> Reasons {
        self.reason
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        let size: usize = array.len();
        if self.slice == usize::MAX {
            self.slice = 0;
        }
        if self.state == State::Over {
            if size < 2 * self.power {
                return true;
            }
            if self.slice < size - 2 * self.power {
                self.slice += 2 * self.power;
            } else {
                self.slice = 0;
                self.power *= 2;
            }
            self.state = State::Init;
        }
        self.switch(array);
        false
    }

    fn modify_state(&mut self, array: &[usize]) -> bool {
        self.power > array.len()
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        let end_of_slice = min(self.slice + 2 * self.power - 1, array.len() - 1);
        if self.state == State::Init {
            self.temp = array.to_owned();
            self.i = self.slice;
            self.j = self.slice + self.power;
            self.k = self.slice;
            self.state = State::Comparing;
        }
        if self.state == State::Comparing {
            if self.i < self.slice + self.power && self.j <= end_of_slice {
                self.special = (self.i, self.j);
                self.reason = Reasons::Comparing;
                if array[self.i] < array[self.j] {
                    self.temp[self.k] = array[self.i];
                    self.i += 1;
                } else {
                    self.temp[self.k] = array[self.j];
                    self.j += 1;
                }
                self.k += 1;
            } else {
                while self.i < array.len() && self.i < self.slice + self.power {
                    self.temp[self.k] = array[self.i];
                    self.k += 1;
                    self.i += 1;
                }
                self.state = State::Merging;
                self.merge_tracker = self.slice;
            }
        }
        if self.state == State::Merging {
            self.special = (self.merge_tracker, self.merge_tracker);
            self.reason = Reasons::Switching;
            array[self.merge_tracker] = self.temp[self.merge_tracker];
            if self.merge_tracker >= end_of_slice {
                self.state = State::Over;
            } else {
                self.merge_tracker += 1;
            }
        }
    }

    fn reset_state(&mut self) {
        *self = MergeSort::new();
    }
}

#[cfg(test)]
mod tests {

    use super::MergeSort;
    use crate::{
        algorithms::{
            Sorter, {CEIL, FLOOR, REPETITIONS, SIZE},
        },
        util,
    };

    #[test]
    fn run() {
        for _ in 0..REPETITIONS {
            let mut sorter = MergeSort::new();
            let mut array = util::gen_random_vector(FLOOR, CEIL, SIZE);

            let mut expected = array.clone();
            expected.sort();

            sorter.run(&mut array);

            assert_eq!(array, expected);
        }
    }
}
