pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];

    let mut left = 1;
    let mut right = 1;
    let len = nums.len();

    for i in 0..len {
        result[i] *= left;
        left *= nums[i];
        result[len - 1 - i] *= right;
        right *= nums[len - 1 - i];
    }
    result
}
