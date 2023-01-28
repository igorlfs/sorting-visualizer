pub trait Sorter {
    fn new() -> Self
    where
        Self: Sized;
    fn modify_state(&mut self, len: usize) -> bool;
    fn run<T: Ord>(&mut self, array: &mut Vec<T>);
    fn step<T: Ord>(&self, array: &mut Vec<T>);
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
    fn run<T: Ord>(&mut self, array: &mut Vec<T>) {
        let len: usize = array.len();
        while self.x != len - 1 {
            self.step(array);
            self.modify_state(len);
        }
    }
    fn step<T: Ord>(&self, array: &mut Vec<T>) {
        if array[self.y] > array[self.y + 1] {
            array.swap(self.y, self.y + 1);
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
}
