/// Quick sort is unstable
fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    qsort(arr, 0, arr.len() - 1);
}

fn qsort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot = arr[(low + high) / 2];
        let mut i = low;
        let mut j = high;
        while i < j {
            // find element to swap left side
            while arr[i] < pivot {
                i += 1;
            }
            // find element to swap right side
            while arr[j] > pivot {
                j -= 1;
            }
            arr.swap(i, j);
        }
        if i > low {
            qsort(arr, low, i - 1);
        }
        if i < high {
            qsort(arr, i + 1, high);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::create_unsorted_dataset;
    use super::*;

    #[test]
    fn basic() {
        let mut tests = vec![1,6,3,8,2];
        quick_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,6,8]);
    }

    #[test]
    fn already_sorted() {
        let mut tests = vec![1,2,3,4,5,6];
        quick_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,4,5,6]);
    }

    #[test]
    fn empty() {
        let mut tests = vec![];
        quick_sort(&mut tests);
        assert_eq!(tests, vec![]);
    }

    #[test]
    fn one_element() {
        let mut tests = vec![3];
        quick_sort(&mut tests);
        assert_eq!(tests, vec![3]);
    }

    #[test]
    fn big_dataset() {
        let mut data = create_unsorted_dataset();

        quick_sort(&mut data);

        for i in 1..data.iter().len() {
            assert!(data[i] >= data[i - 1]);
        }
    }
}