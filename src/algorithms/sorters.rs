pub trait Sorter {
    fn new(len: usize) -> Self
    where
        Self: Sized;
    fn modify_state(&mut self) -> bool;
    fn run(&mut self, numbers: &mut Vec<u32>);
    fn step(&self, numbers: &mut Vec<u32>);
    fn reset(&mut self);
}

pub struct BubbleSort {
    x: usize,
    y: usize,
    len: usize,
}

impl Sorter for BubbleSort {
    fn new(len: usize) -> BubbleSort {
        BubbleSort { x: 0, y: 0, len }
    }
    fn modify_state(&mut self) -> bool {
        if self.y < self.len - 2 - self.x {
            self.y += 1;
        } else {
            self.x += 1;
            self.y = 0;
        }
        if self.x == self.len - 1 {
            return true;
        }
        false
    }
    fn run(&mut self, numbers: &mut Vec<u32>) {
        while self.x != self.len - 1 {
            self.step(numbers);
            self.modify_state();
        }
    }
    fn step(&self, numbers: &mut Vec<u32>) {
        if numbers[self.y] > numbers[self.y + 1] {
            numbers.swap(self.y, self.y + 1);
        }
    }
    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sorters::BubbleSort;
    use crate::algorithms::sorters::Sorter;

    #[test]
    fn run() {
        let mut arr: Vec<u32> = vec![5, 2, 3, 4, 1];
        let mut sorter = BubbleSort::new(arr.len());
        sorter.run(&mut arr);
        let expected: Vec<u32> = vec![1, 2, 3, 4, 5];
        assert_eq!(arr, expected);
    }

    #[test]
    fn modify_state() {
        let mut sorter = BubbleSort::new(4);
        sorter.modify_state();
        assert_eq!(sorter.y, 1);
        sorter.modify_state();
        assert_eq!(sorter.y, 2);
        sorter.modify_state();
        assert_eq!(sorter.y, 0);
    }
}
