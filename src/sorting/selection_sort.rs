fn selection_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mut index_to_insert = arr.len() - 1;

    loop {
        let mut max_pos = 0;
        for i in 0..=index_to_insert {
            if arr[i] > arr[max_pos] {
                max_pos = i;
            }
        }

        arr.swap(index_to_insert, max_pos);

        if index_to_insert == 0 {
            return;
        }
        index_to_insert = index_to_insert - 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::create_unsorted_dataset;
    use super::*;

    #[test]
    fn basic() {
        let mut tests = vec![1,6,3,8,2];
        selection_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,6,8]);
    }

    #[test]
    fn already_sorted() {
        let mut tests = vec![1,2,3,4,5,6];
        selection_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,4,5,6]);
    }

    #[test]
    fn empty() {
        let mut tests = vec![];
        selection_sort(&mut tests);
        assert_eq!(tests, vec![]);
    }

    #[test]
    fn one_element() {
        let mut tests = vec![3];
        selection_sort(&mut tests);
        assert_eq!(tests, vec![3]);
    }

    #[test]
    fn big_dataset() {
        let mut data = create_unsorted_dataset();

        selection_sort(&mut data);

        for i in 1..data.iter().len() {
            assert!(data[i] >= data[i - 1]);
        }
    }
}