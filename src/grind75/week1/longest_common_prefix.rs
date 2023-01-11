// Problem: https://leetcode.com/problems/longest-common-prefix/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    // TODO: change to iterate by characters for all strings
    strs.iter().skip(1).fold(strs[0].clone(), |common_prefix, string| {
        common_prefix
            .chars()
            .zip(string.chars())
            .take_while(|(x, y)| x == y)
            .map(|(x, _)| x)
            .collect()
    })
}

pub fn longest_common_prefix_imperative(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    // compare until
    strs.iter().skip(1).fold(strs[0].clone(), |common_prefix, string| {
        common_prefix
            .chars()
            .zip(string.chars())
            .take_while(|(x, y)| x == y)
            .map(|(x, _)| x)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn leetcode_example_1() {
        let vec_string = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        assert_eq!(longest_common_prefix(vec_string), "fl".to_string());
    }

    #[test]
    fn example_2() {
        let vec_string = vec!["car".to_string(), "flow".to_string(), "cop".to_string()];
        assert_eq!(longest_common_prefix(vec_string), "".to_string());
    }

    #[test]
    fn empty_case() {
        let vec_string = vec![];
        assert_eq!(longest_common_prefix(vec_string), "".to_string());
    }
}
