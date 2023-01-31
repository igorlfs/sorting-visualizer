/// A Sorter is a sorting algorithm split in two stages: the `step` and the `state`.
/// A `step` can be any single step an algorithm takes, such as comparing or switching numbers
/// A `state` controls the variables that the `step` is going to use.
pub trait Sorter {
    fn new() -> Self
    // The Compiler will complain if we don't do this
    where
        Self: Sized;

    /// Returns the indexes currently being compared or about to switch.
    fn get_special(&self) -> (usize, usize);

    /// Return the loop's variables
    fn get_state(&self) -> (usize, usize);

    /// Runs the algorithm all at once.
    fn run(&mut self, array: &mut Vec<usize>);

    /// Takes a single step in running the algorithm.
    /// Returns true if all states have been covered.
    fn step(&mut self, array: &mut Vec<usize>) -> bool;

    /// Modifying the state is analogous to stepping in a loop.
    /// Returns true if all states have been traversed.
    fn modify_state(&mut self, array: &[usize]) -> bool;

    /// Handles switching positions in an array
    fn switch(&mut self, array: &mut Vec<usize>);

    /// Set the Sorter's state to it's initial state.
    fn reset_state(&mut self);
}

pub struct BubbleSort {
    x: usize,
    y: usize,
    needs_switch: bool,
}

impl BubbleSort {}

impl Sorter for BubbleSort {
    fn new() -> BubbleSort {
        BubbleSort {
            x: 0,
            y: usize::MAX,
            needs_switch: false,
        }
    }
    fn get_special(&self) -> (usize, usize) {
        if self.y != usize::MAX {
            (self.y, self.y + 1)
        } else {
            (usize::MAX, usize::MAX)
        }
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn run(&mut self, array: &mut Vec<usize>) {
        loop {
            if self.step(array) {
                break;
            }
        }
        self.reset_state();
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
        if self.x == array.len() - 1 {
            return true;
        }
        if self.y < array.len() - 1 - self.x {
            self.y = if self.y == usize::MAX { 0 } else { self.y + 1 };
        } else {
            self.x += 1;
            self.y = 0;
        }
        self.needs_switch = array[self.y] > array[self.y + 1];
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.swap(self.y, self.y + 1);
        self.needs_switch = false;
    }

    fn reset_state(&mut self) {
        self.x = 0;
        self.y = usize::MAX;
    }
}

#[cfg(test)]
mod tests_bubble {
    use crate::algorithms::BubbleSort;
    use crate::algorithms::Sorter;

    #[test]
    fn run() {
        let mut sorter = BubbleSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 6];

        sorter.run(&mut arr);

        let expected: Vec<usize> = vec![2, 5, 6];

        assert_eq!(arr, expected);

        // We must reset after running
        assert_eq!((sorter.x, sorter.y), (0, usize::MAX))
    }

    #[test]
    fn step() {
        let mut sorter = BubbleSort::new();
        let mut arr: Vec<usize> = vec![5, 2];

        // Selects indexes 0 and 1 for comparing
        sorter.step(&mut arr);
        assert_eq!(vec![5, 2], arr);
        assert_eq!(sorter.get_special(), (0, 1));

        // Swaps 2 and 5
        sorter.step(&mut arr);
        assert_eq!(vec![2, 5], arr);
        assert_eq!(sorter.get_special(), (0, 1));
    }

    // TODO: Test modify_state

    #[test]
    fn test_switch() {
        let mut sorter = BubbleSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 6];

        sorter.y = 0;
        sorter.switch(&mut arr);
        assert_eq!(arr, vec![2, 5, 6]);
        assert!(!sorter.needs_switch);
    }
}

pub struct SelectionSort {
    x: usize,
    y: usize,
    min: usize,
    needs_switch: bool,
    special: (usize, usize),
}

impl Sorter for SelectionSort {
    fn new() -> SelectionSort {
        SelectionSort {
            x: 0,
            y: 1,
            min: 0,
            needs_switch: false,
            special: (usize::MAX, usize::MAX),
        }
    }

    fn get_special(&self) -> (usize, usize) {
        self.special
    }

    fn get_state(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn run(&mut self, array: &mut Vec<usize>) {
        loop {
            if self.step(array) {
                break;
            }
        }
        self.reset_state();
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
        if self.x == array.len() - 1 {
            return true;
        }
        self.special = (self.y, self.min);
        if array[self.y] < array[self.min] {
            self.min = self.y;
        }
        self.y += 1;
        if self.y == array.len() {
            self.needs_switch = true;
        }
        false
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        self.special = (self.x, self.min);
        array.swap(self.x, self.min);
        self.x += 1;
        self.min = self.x;
        self.y = self.x + 1;
        self.needs_switch = false;
    }

    fn reset_state(&mut self) {
        self.x = 0;
        self.y = 1;
        self.min = 0;
        self.special = (usize::MAX, usize::MAX);
    }
}

#[cfg(test)]
mod tests_selection {
    use super::{SelectionSort, Sorter};

    #[test]
    fn run() {
        let mut sorter = SelectionSort::new();
        let mut arr: Vec<usize> = vec![9, 2, 8, 10, 5];

        sorter.run(&mut arr);

        let expected: Vec<usize> = vec![2, 5, 8, 9, 10];

        assert_eq!(arr, expected);
    }

    #[test]
    fn step() {
        let mut sorter = SelectionSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 3];

        // Selects indexes 0 and 1 for comparing
        sorter.step(&mut arr);
        assert_eq!(sorter.get_special(), (1, 0));

        // Selects indexes 1 and 2 for comparing
        sorter.step(&mut arr);
        assert_eq!(sorter.get_special(), (2, 1));

        // Swaps 2 and 5
        sorter.step(&mut arr);
        assert_eq!(vec![2, 5, 3], arr);
        assert_eq!(sorter.get_special(), (0, 1));
    }

    // TODO: Test modify_state

    #[test]
    fn test_switch() {
        let mut sorter = SelectionSort::new();
        let mut arr: Vec<usize> = vec![5, 2, 6];

        sorter.min = 1;
        sorter.switch(&mut arr);
        assert_eq!(arr, vec![2, 5, 6]);
        assert_eq!(sorter.x, 1);
        assert_eq!(sorter.x, sorter.min);
        assert_eq!(sorter.y, 2);
        assert!(!sorter.needs_switch);
    }
}

pub struct InsertionSort {
	x: usize,
	y: usize,
	needs_switch: bool,
}

impl InsertionSort {}

impl Sorter for InsertionSort{
	fn new() -> InsertionSort {
		InsertionSort {
			x: 0,
			y: 1,
			needs_switch: false,
		}
	}

	fn get_state(&self) -> (usize, usize) {
		(self.x, self.y)
	}
	
	fn get_special(&self) -> (usize, usize) {
		(self.x, self.y)
	}
	
	fn run(&mut self, array: &mut Vec<usize>) {
		loop {
			if self.step(array){
				break;
			}	
		}
	}
	
	fn step(&mut self, array: &mut Vec<usize>) -> bool {
		if self.needs_switch {
			self.switch(array)
		} else {
			return self.modify_state(array);	
		}
		false
	}

	fn modify_state(&mut self, array: &[usize]) -> bool {
		if self.y == array.len() {
			return true;
		} 
		
		self.needs_switch = array[self.y] < array[self.x] && self.y > 0;
		if !self.needs_switch {
			self.y = self.y + 1;
			self.x = self.x + 1;
		}
		false
	}

	fn switch(&mut self, array: &mut Vec<usize>){
		array.swap(self.y, self.x);
		self.needs_switch = false;
		if self.y != 1{
			self.x = self.x - 1;
			self.y = self.y - 1;
		}
	}

	fn reset_state(&mut self) {
		self.x = 0;
		self.y = 1;
	}
}
