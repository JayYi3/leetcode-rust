pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    let mut stack = Vec::new();
    for i in 0..temperatures.len() {
        while let Some(&top) = stack.last() {
            if temperatures[i] <= temperatures[top] {
                break;
            }
            let top = stack.pop().unwrap();
            result[top] = (i - top) as i32;
        }
        stack.push(i);
    }
    result
}
