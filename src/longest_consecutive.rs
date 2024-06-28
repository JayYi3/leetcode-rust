pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 0;
    for &num in &set {
        if !set.contains(&(num - 1)) {
            let count = (num..).take_while(|n| set.contains(n)).count();
            longest = longest.max(count);
        }
    }
    longest as i32
}