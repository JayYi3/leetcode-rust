fn encode(strs: Vec<String>) -> String {
    strs.into_iter()
        .map(|s| format!("{}:{}#", s.len(), s))
        .collect()
}

fn decode(s: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < s.len() {
        let mut j = i;
        while s.chars().nth(j).unwrap() != ':' {
            j += 1;
        }
        let len: usize = s[i..j].parse().unwrap();
        let start = j + 1;
        let end = start + len;
        result.push(s[start..end].to_string());
        i = end + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode(vec!["abc".to_string(), "def".to_string()]),
            "3:abc#3:def#"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("3:abc#3:def#".to_string()),
            vec!["abc".to_string(), "def".to_string()]
        );
    }

    #[test]
    fn test_encode_decode() {
        let strs = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(decode(encode(strs.clone())), strs);
    }
}