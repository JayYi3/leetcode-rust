pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::cmp::Ordering;
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        match (numbers[i] + numbers[j]).cmp(&target) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            Ordering::Equal => return vec![i as i32 + 1, j as i32 + 1],
        }
    }
    unreachable!()
}
