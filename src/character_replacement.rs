
fn character_replacement(s: String, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut char_count = HashMap::new();
    let mut max_count = 0;
    let mut max_length = 0;
    let mut start = 0;
    let s_chars: Vec<char> = s.chars().collect();

    for (end, &c) in s_chars.iter().enumerate() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
        max_count = max_count.max(*count);

        if (end - start + 1) as i32 - max_count > k {
            *char_count.get_mut(&s_chars[start]).unwrap() -= 1;
            start += 1;
        }

        max_length = max_length.max(end - start + 1);
    }

    max_length as i32
}