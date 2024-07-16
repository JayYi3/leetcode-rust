pub fn generate_parentheses(n: i32) -> Vec<String> {
    fn backtrack(result: &mut Vec<String>, path: &mut String, open: i32, close: i32, max: i32) {
        if path.len() == (max * 2) as usize {
            result.push(path.clone());
            return;
        }

        if open < max {
            path.push('(');
            backtrack(result, path, open + 1, close, max);
            path.pop();
        }

        if close < open {
            path.push(')');
            backtrack(result, path, open, close + 1, max);
            path.pop();
        }
    }
    
    let mut result = Vec::new();
    let mut path = String::new();
    backtrack(&mut result, &mut path, 0, 0, n);
    result
}
