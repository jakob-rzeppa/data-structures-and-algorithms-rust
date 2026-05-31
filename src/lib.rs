#[allow(dead_code)]
mod binary_search;
#[allow(dead_code)]
mod sorting;
#[allow(dead_code)]
mod data_structures;
#[allow(dead_code)]
mod shortest_path;

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
