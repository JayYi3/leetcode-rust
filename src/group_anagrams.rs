pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        map.entry(key).or_insert(vec![]).push(s);
    }
    map.into_values().collect()
}
