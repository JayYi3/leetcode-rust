pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut stack = Vec::new();

    // Calculate left boundaries for each position.
    for i in 0..n {
        while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
            stack.pop();
        }
        left[i] = if stack.is_empty() {
            -1
        } else {
            *stack.last().unwrap() as i32
        };
        stack.push(i);
    }

    stack.clear();

    // Calculate right boundaries for each position.
    for i in (0..n).rev() {
        while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
            stack.pop();
        }
        right[i] = if stack.is_empty() {
            n as i32
        } else {
            *stack.last().unwrap() as i32
        };
        stack.push(i);
    }

    // Calculate maximum area
    let mut res = 0;
    for i in 0..n {
        res = res.max(heights[i] * (right[i] - left[i] - 1));
    }

    res
}
