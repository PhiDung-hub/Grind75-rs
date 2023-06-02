// Problem: https://leetcode.com/problems/longest-palindromic-substring

// Sliding window approach
pub fn longest_palindrome(s: String) -> String {
    fn is_palidrone(s: &[u8]) -> bool {
        let reversed_s = s.iter().rev();
        s.iter().zip(reversed_s).all(|(l, r)| l == r) // check palindrome for a string s.
    }

    // iterate size in reverse so this will return the first found
    for window_size in (1..=s.len()).rev() {
        match s.as_bytes().windows(window_size).find(|sub_str| is_palidrone(sub_str)) {
            Some(str) => return String::from_utf8(str.to_vec()).unwrap(),
            None => continue,
        }
    }
    "".to_string()
}

// Manacher's algorithm
pub fn longest_palindrome_manacher(s: String) -> String {
    // build a new string with # between each char
    let mut bogus_s = String::with_capacity(s.len() * 2 + 1);
    for c in s.chars() {
        bogus_s.push('#');
        bogus_s.push(c);
    }
    bogus_s.push('#');

    let mut p = vec![0; bogus_s.len()]; // store longest palindromic length of current center
    let mut center = 0;
    let mut right = 0;

    let (mut i_max, mut p_max) = (0, 0);
    for i in 1..bogus_s.len() - 1 {
        // Manacher optimization, utilizes computed values.
        if i < right {
            // image of current index `p[i]` w.r.t current center `p[mirror]`.
            let mirror = 2 * center - i;
            p[i] = std::cmp::min(right as i32 - i as i32, p[mirror]);
        }

        // expand palindrome centered at current i, reuse p[i] -> speedup
        while (i + 1 + p[i] as usize) < bogus_s.len()
            && ((i - 1) as i32 - p[i]) >= 0
            && bogus_s.as_bytes()[i + (1 + p[i] as usize)] == bogus_s.as_bytes()[i - (1 + p[i] as usize)]
        {
            p[i] += 1;
        }

        if i + p[i] as usize > right {
            center = i;
            right = i + p[i] as usize;
        }

        // get max len index
        if p[i] > p_max {
            i_max = i;
            p_max = p[i];
        }
    }

    // rebuild the string
    let mut answer = String::new();
    for i in (i_max - p_max as usize)..=(i_max + p_max as usize) {
        if bogus_s.as_bytes()[i] != '#' as u8 {
            answer.push(bogus_s.as_bytes()[i] as char);
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let string = "babad".to_string();
        let expected = "bab".to_string();
        assert_eq!(longest_palindrome_manacher(string.clone()), expected);
        assert_eq!(longest_palindrome(string), expected);
    }

    #[test]
    fn example_2() {
        let string = "cbbd".to_string();
        let expected = "bb".to_string();
        assert_eq!(longest_palindrome_manacher(string.clone()), expected);
        assert_eq!(longest_palindrome(string), expected);
    }

    #[test]
    fn example_3() {
        let string = "a".to_string();
        let expected = "a".to_string();
        assert_eq!(longest_palindrome_manacher(string.clone()), expected);
        assert_eq!(longest_palindrome(string), expected);
    }
}
