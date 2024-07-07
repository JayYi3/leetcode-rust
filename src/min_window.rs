pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;
    let mut need: HashMap<char, i32> = HashMap::new();
    let mut window: HashMap<char, i32> = HashMap::new();

    for c in t.chars() {
        *need.entry(c).or_insert(0) += 1;
    }

    let s: Vec<char> = s.chars().collect();
    let (mut left, mut right) = (0, 0);
    let mut valid = 0;
    let (mut start, mut len) = (0, usize::MAX);

    while right < s.len() {
        let c = s[right];
        right += 1;

        if need.contains_key(&c) {
            *window.entry(c).or_insert(0) += 1;
            if window[&c] == need[&c] {
                valid += 1;
            }
        }

        while valid == need.len() {
            if right - left < len {
                start = left;
                len = right - left;
            }

            let d = s[left];
            left += 1;

            if need.contains_key(&d) {
                if window[&d] == need[&d] {
                    valid -= 1;
                }
                *window.get_mut(&d).unwrap() -= 1;
            }
        }
    }

    if len == usize::MAX {
        String::new()
    } else {
        s[start..start + len].iter().collect()
    }
}
