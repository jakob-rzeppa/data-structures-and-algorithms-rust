fn insertion_sort(arr: &mut [i32]) {
    for i in 2..arr.len() {
        let mut marker = i;
        let tmp = arr[i];

        while marker > 0 && arr[marker - 1] > tmp {
            arr[marker] = arr[marker - 1];
            marker -= 1;
        }

        arr[marker] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use crate::create_unsorted_dataset;
    use super::*;

    #[test]
    fn basic() {
        let mut tests = vec![1,6,3,8,2];
        insertion_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,6,8]);
    }

    #[test]
    fn already_sorted() {
        let mut tests = vec![1,2,3,4,5,6];
        insertion_sort(&mut tests);
        assert_eq!(tests, vec![1,2,3,4,5,6]);
    }

    #[test]
    fn empty() {
        let mut tests = vec![];
        insertion_sort(&mut tests);
        assert_eq!(tests, vec![]);
    }

    #[test]
    fn one_element() {
        let mut tests = vec![3];
        insertion_sort(&mut tests);
        assert_eq!(tests, vec![3]);
    }

    #[test]
    fn big_dataset() {
        let mut data = create_unsorted_dataset();

        insertion_sort(&mut data);

        for i in 1..data.iter().len() {
            assert!(data[i] >= data[i - 1]);
        }
    }
}