// Problem: https://leetcode.com/problems/palindrome-linked-list/
use crate::structs::list::*;

pub fn is_palindrome(mut head: Link<i32>) -> bool {
    // 1. traverse the list, get len & get middle of the list
    let mut values = vec![];
    while let Some(node) = head {
        values.push(node.val);
        head = node.next;
    }
    let values_len = values.len();
    if values_len < 2 {
        return true;
    }

    let split_idx = values_len / 2;
    let (first_half, second_half) = values.split_at_mut(split_idx);

    // 2. reverse the first half of the linkedlist
    first_half.reverse();

    // 3. compare 2 parts
    if values_len % 2 == 0 {
        first_half == second_half
    } else {
        first_half == &mut second_half[1..]
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let list = List::from_vec(vec![1]);
        assert!(is_palindrome(list.head.clone()));
    }

    #[test]
    fn example_2() {
        let list = List::from_vec(vec![1, 3, 4, 3, 1]);
        assert!(is_palindrome(list.head.clone()));
        let list = List::from_vec(vec![1, 3, 4, 4, 3, 1]);
        assert!(is_palindrome(list.head.clone()));
    }

    #[test]
    fn example_3() {
        let list = List::from_vec(vec![1, 3, 4, 1, 3, 1]);
        assert!(!is_palindrome(list.head.clone()));
        let list = List::from_vec(vec![1, 3]);
        assert!(!is_palindrome(list.head.clone()));
    }
}
