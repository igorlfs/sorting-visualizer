pub trait Sorter {
    fn new() -> Self
    where
        Self: Sized;
    fn modify_state(&mut self, len: usize) -> bool;
    fn get_state(&self) -> (usize, usize);
    fn run(&mut self, array: &mut Vec<u32>);
    fn step(&self, array: &mut Vec<u32>);
    fn reset(&mut self);
}

pub struct BubbleSort {
    x: usize,
    y: usize,
}

impl Sorter for BubbleSort {
    fn new() -> BubbleSort {
        BubbleSort { x: 0, y: 0 }
    }
    fn modify_state(&mut self, len: usize) -> bool {
        if self.y < len - 2 - self.x {
            self.y += 1;
        } else {
            self.x += 1;
            self.y = 0;
        }
        if self.x == len - 1 {
            return true;
        }
        false
    }
    fn run(&mut self, array: &mut Vec<u32>) {
        let len: usize = array.len();
        loop {
            self.step(array);
            if self.modify_state(len) {
                break;
            }
        }
        self.reset();
    }
    fn step(&self, array: &mut Vec<u32>) {
        if array[self.y] > array[self.y + 1] {
            array.swap(self.y, self.y + 1);
        }
    }
    fn get_state(&self) -> (usize, usize) {
        (self.y, self.y + 1)
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
        let mut sorter = BubbleSort::new();
        sorter.run(&mut arr);
        let expected: Vec<u32> = vec![1, 2, 3, 4, 5];
        assert_eq!(arr, expected);
    }

    #[test]
    fn modify_state() {
        let len: usize = 4;
        let mut sorter = BubbleSort::new();
        sorter.modify_state(len);
        assert_eq!(sorter.y, 1);
        sorter.modify_state(len);
        assert_eq!(sorter.y, 2);
        sorter.modify_state(len);
        assert_eq!(sorter.y, 0);
    }

    #[test]
    fn reset() {
        let mut sorter = BubbleSort::new();
        sorter.reset();
        assert_eq!(sorter.x, 0);
        assert_eq!(sorter.y, 0);
    }
}
