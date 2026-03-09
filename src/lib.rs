mod binary_search;
mod sorting;
mod data_structures;

#[cfg(test)]
fn create_unsorted_dataset() -> Vec<i32> {
    use std::collections::HashSet;
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut dataset = HashSet::new();

    // Generate unique random numbers
    while dataset.len() < 1000 {
        dataset.insert(rng.gen_range(0..i32::MAX));
    }

    dataset.into_iter().collect()
}