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
}

impl Bundle {
    pub fn new(numbers: Vec<u32>, options: Vec<Options>) -> Bundle {
        assert_eq!(numbers.len(), options.len());
        Bundle { numbers, options }
    }

    /// Sets `options` to Default.
    pub fn reset_options(&mut self) {
        for item in &mut self.options {
            *item = Options::Default;
        }
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
        self.options[a] = Options::Comparing;
        self.options[b] = Options::Comparing;
    }

    /// Clears last comparing indexes and set new ones.
    pub fn set_switching(&mut self, (a, b): (usize, usize)) {
        self.reset_options();
        self.options[a] = Options::Switching;
        self.options[b] = Options::Switching;
    }

    /// Checks if `options` is all Default.
    pub fn all_default(&self) -> bool {
        self.options.iter().all(|&x| x == Options::Default)
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
        bundle.options[0] = Options::Switching;
        assert!(!bundle.all_default());
    }
}
