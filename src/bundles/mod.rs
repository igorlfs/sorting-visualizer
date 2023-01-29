#[derive(Eq, PartialEq, PartialOrd, Ord, Default, Clone, Debug, Copy)]
pub enum Options {
    #[default]
    Default,
    Comparing,
    Switching,
}

#[derive(PartialEq, PartialOrd)]
pub struct Bundle {
    numbers: Vec<u32>,
    options: Vec<Options>,
    indexes: (usize, usize),
}

impl Bundle {
    pub fn new(numbers: Vec<u32>, options: Vec<Options>) -> Bundle {
        assert_eq!(numbers.len(), options.len());
        Bundle {
            numbers,
            options,
            indexes: (usize::MAX, usize::MAX),
        }
    }

    /// Sets `options` to Default.
    pub fn reset_options(&mut self) {
        let (a, b) = (self.indexes.0, self.indexes.1);
        if a != usize::MAX {
            self.options[a] = Options::Default;
        }
        if b != usize::MAX {
            self.options[b] = Options::Default;
        }
    }

    pub fn clear_indexes(&mut self) {
        self.indexes = (usize::MAX, usize::MAX);
    }

    pub fn numbers(&self) -> &[u32] {
        self.numbers.as_ref()
    }

    pub fn options(&self) -> &[Options] {
        self.options.as_ref()
    }

    pub fn set_numbers(&mut self, numbers: Vec<u32>) {
        self.numbers = numbers;
    }

    pub fn numbers_mut(&mut self) -> &mut Vec<u32> {
        &mut self.numbers
    }

    /// Clears last comparing indexes and set new ones.
    pub fn set_comparing(&mut self, (a, b): (usize, usize)) {
        self.reset_options();
        self.indexes.0 = a;
        self.indexes.1 = b;
        self.options[a] = Options::Comparing;
        self.options[b] = Options::Comparing;
    }

    /// Clears last comparing indexes and set new ones.
    pub fn set_switching(&mut self, (a, b): (usize, usize)) {
        self.reset_options();
        self.indexes.0 = a;
        self.indexes.1 = b;
        self.options[a] = Options::Switching;
        self.options[b] = Options::Switching;
    }

    /// Checks if `options` is all Default.
    pub fn all_default(&self) -> bool {
        self.indexes.0 == usize::MAX || self.indexes.1 == usize::MAX
    }

    pub(crate) fn indexes(&self) -> (usize, usize) {
        self.indexes
    }
}

#[cfg(test)]
mod tests {
    use super::{Bundle, Options};

    #[test]
    fn set_comparing() {
        let arr: Vec<u32> = vec![5, 2, 3, 4, 1];
        let options: Vec<Options> = vec![Options::Default; arr.len()];
        let mut bundle = Bundle::new(arr, options);

        bundle.set_comparing((0, 1));

        assert_eq!(bundle.options[0], Options::Comparing);
        assert_eq!(bundle.options[1], Options::Comparing);
    }

    #[test]
    fn set_switching() {
        let arr: Vec<u32> = vec![5, 2, 3, 4, 1];
        let options: Vec<Options> = vec![Options::Default; arr.len()];
        let mut bundle = Bundle::new(arr, options);

        bundle.set_switching((0, 1));

        assert_eq!(bundle.options[0], Options::Switching);
        assert_eq!(bundle.options[1], Options::Switching);
    }

    #[test]
    fn all_default() {
        let arr: Vec<u32> = vec![5, 2, 3, 4, 1];
        let options: Vec<Options> = vec![Options::Default; arr.len()];
        let mut bundle = Bundle::new(arr, options);

        assert!(bundle.all_default());
        bundle.set_switching((0, 1));
        assert!(!bundle.all_default());
    }

    #[should_panic]
    #[test]
    fn new() {
        let arr: Vec<u32> = vec![1];
        let options: Vec<Options> = vec![];
        Bundle::new(arr, options);
    }
}
