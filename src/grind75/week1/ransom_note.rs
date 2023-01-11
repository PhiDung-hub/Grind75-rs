// Problem: https://leetcode.com/problems/ransom-note/

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if magazine.len() < ransom_note.len() {
        return false;
    }
    let mut map = [0; 26];
    let mut flags = 0; // this is more than 26 bits, use it for check map -> save space.

    let to_idx = |c: char| c as usize - 'a' as usize;
    ransom_note.chars().for_each(|c| {
        let i = to_idx(c);
        map[i] += 1;
        flags |= 1 << i;
    });

    for c in magazine.chars() {
        let i = to_idx(c);
        map[i] -= 1;
        if map[i] == 0 {
            flags ^= 1 << i;
            if flags == 0 {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(can_construct("aa".to_string(), "ab".to_string()), false);
    }

    #[test]
    fn example_2() {
        assert_eq!(can_construct("aa".to_string(), "aba".to_string()), true);
    }

    #[test]
    fn example_3() {
        assert_eq!(can_construct("aabcdefz".to_string(), "abcdefgz".to_string()), false);
    }
}
