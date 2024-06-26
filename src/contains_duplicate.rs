pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}
