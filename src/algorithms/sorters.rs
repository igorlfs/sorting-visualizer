pub trait Sorter {
    fn run(numbers: &mut Vec<u32>);
}

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn run(arr: &mut Vec<u32>) {
        let len: usize = arr.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sorters::BubbleSort;
    use crate::algorithms::sorters::Sorter;

    #[test]
    fn run() {
        let mut arr: Vec<u32> = vec![5, 2, 3, 4, 1];
        BubbleSort::run(&mut arr);
        let expected: Vec<u32> = vec![1, 2, 3, 4, 5];
        assert_eq!(arr, expected);
    }
}
