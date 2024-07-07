pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut result = Vec::with_capacity(nums.len() - k as usize + 1);

    let mut queue = VecDeque::new();

    for idx in 0..nums.len() {
        while let Some(&pos) = queue.front() {
            if pos + k as usize > idx {
                break;
            }

            queue.pop_front();
        }
        while let Some(&pos) = queue.back() {
            let old = nums[pos];
            if old > nums[idx] {
                break;
            }

            queue.pop_back();
        }

        queue.push_back(idx);

        if idx + 1 >= k as usize {
            if let Some(&max) = queue.front() {
                result.push(nums[max]);
            }
        }
    }

    result
}
