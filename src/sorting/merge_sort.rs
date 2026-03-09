fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return Vec::from(arr);
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    let left_sorted = merge_sort(left);
    let right_sorted = merge_sort(right);
    merge(&left_sorted, &right_sorted)
}

fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(arr1.len() + arr2.len());

    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < arr1.len() && i2 < arr2.len() {
        if arr1[i1] <= arr2[i2] {
            res.push(arr1[i1]);
            i1 += 1;
        } else {
            res.push(arr2[i2]);
            i2 += 1;
        }
    }

    // Add remaining elements
    while i1 < arr1.len() {
        res.push(arr1[i1]);
        i1 += 1;
    }
    while i2 < arr2.len() {
        res.push(arr2[i2]);
        i2 += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use rand::Rng;
    use crate::create_unsorted_dataset;
    use super::*;

    #[test]
    fn basic() {
        let mut tests = vec![1,6,3,8,2];
        assert_eq!(merge_sort(&mut tests), vec![1,2,3,6,8]);
    }

    #[test]
    fn already_sorted() {
        let mut tests = vec![1,2,3,4,5,6];
        assert_eq!(merge_sort(&mut tests), vec![1,2,3,4,5,6]);
    }

    #[test]
    fn empty() {
        let mut tests = vec![];
        assert_eq!(merge_sort(&mut tests), vec![]);
    }

    #[test]
    fn one_element() {
        let mut tests = vec![3];
        assert_eq!(merge_sort(&mut tests), vec![3]);
    }

    #[test]
    fn big_dataset() {
        let mut data = create_unsorted_dataset();

        let res = merge_sort(&mut data);

        for i in 1..res.iter().len() {
            assert!(res[i] >= res[i - 1]);
        }
    }
}