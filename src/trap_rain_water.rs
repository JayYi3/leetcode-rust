pub fn trap(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut pool_height = 0;
    let mut trapped = 0;

    while left < right {
        pool_height = pool_height.max(height[left].min(height[right]));

        if height[left] <= height[right] {
            trapped += pool_height - height[left];
            left += 1;
        } else {
            trapped += pool_height - height[right];
            right -= 1;
        }
    }

    trapped
}
