use std::cmp::Ordering;

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;
    let mut mid: usize = high / 2;

    loop {
        match arr[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {
                let new_mid = (high - mid) / 2 + mid;
                if new_mid == mid {
                    return None;
                }
                low = mid;
                mid = new_mid;
            },
            Ordering::Greater => {
                let new_mid = (mid - low) / 2;
                if new_mid == mid {
                    return None;
                }
                high = mid;
                mid = new_mid;
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::collections::HashSet;

    fn create_sorted_dataset(size: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut dataset = HashSet::new();

        // Generate unique random numbers
        while dataset.len() < size {
            dataset.insert(rng.gen_range(0..i32::MAX));
        }

        // Convert to sorted vector
        let mut sorted_data: Vec<i32> = dataset.into_iter().collect();
        sorted_data.sort_unstable();
        sorted_data
    }

    #[test]
    fn test() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 4), Some(3));
        assert_eq!(binary_search(&arr, 6), None);
    }

    #[test]
    fn test_large_dataset() {
        let arr: Vec<i32> = create_sorted_dataset(1000);
        let res_index = 483;
        let res_value = arr[res_index];

        assert_eq!(binary_search(&arr, res_value), Some(res_index));
    }
}
