fn bubble_sort(arr: &mut [i32]) {
    if arr.len() <= 1 { return }

    let mut max = arr.len() - 1;
    loop {
        let mut swapped = false;

        for i in 1..=max {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }

        if !swapped { break }

        max = max - 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::create_unsorted_dataset;
    use super::*;

    #[test]
    fn basic() {
        let mut tests = vec![1,6,3,8,2];
        bubble_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,6,8]);
    }

    #[test]
    fn already_sorted() {
        let mut tests = vec![1,2,3,4,5,6];
        bubble_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,4,5,6]);
    }

    #[test]
    fn empty() {
        let mut tests = vec![];
        bubble_sort(&mut tests);
        assert_eq!(tests, vec![]);
    }

    #[test]
    fn one_element() {
        let mut tests = vec![3];
        bubble_sort(&mut tests);
        assert_eq!(tests, vec![3]);
    }

    #[test]
    fn big_dataset() {
        let mut data = create_unsorted_dataset();

        bubble_sort(&mut data);

        for i in 1..data.iter().len() {
            assert!(data[i] >= data[i - 1]);
        }
    }
}