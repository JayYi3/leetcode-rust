pub fn is_palindrome(s: String) -> bool {
    let mut iter = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    iter.clone().eq(iter.rev())
}

pub fn is_palindrome_2(s: String) -> bool {
    let mut chars = s.chars().filter(|c| c.is_alphanumeric());
    while let (Some(left), Some(right)) = (chars.next(), chars.next_back()) {
        if !left.eq_ignore_ascii_case(&right) {
            return false;
        }
    }
    true
}