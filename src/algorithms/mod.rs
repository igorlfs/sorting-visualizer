trait Algorithm {
    fn run(numbers: &mut Vec<i32>);
}

struct BubbleSort;

impl Algorithm for BubbleSort {
    fn run(arr: &mut Vec<i32>) {
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
    use crate::algorithms::Algorithm;
    use crate::algorithms::BubbleSort;

    #[test]
    fn run() {
        let mut arr: Vec<i32> = vec![5, 2, 3, 4, 1];
        BubbleSort::run(&mut arr);
        let expected: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(arr, expected);
    }
}
