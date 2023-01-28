use crate::algorithms::sorters::Sorter;

#[derive(Eq, PartialEq, PartialOrd, Ord, Default, Clone, Debug)]
pub enum Options {
    #[default]
    Default,
    Selected,
}

#[derive(PartialEq, PartialOrd)]
pub struct Bundle {
    numbers: Vec<u32>,
    options: Vec<Options>,
}

impl Bundle {
    pub fn new(numbers: Vec<u32>, options: Vec<Options>) -> Bundle {
        Bundle { numbers, options }
    }
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

    pub fn set_selected(&mut self, sorter: &dyn Sorter) {
        self.reset_options();
        let (a, b): (usize, usize) = sorter.get_state();
        self.options[a] = Options::Selected;
        self.options[b] = Options::Selected;
    }
}
