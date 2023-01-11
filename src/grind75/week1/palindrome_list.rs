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

pub fn is_palindrome_optimized_space(head: Link<i32>) -> bool {
    if head.is_none() {
        return true;
    }
    let middle_node = get_middle_node(head.clone());
    println!("{}", middle_node.clone().unwrap().val);
    let reversed_second_half_start = reverse_list(middle_node.unwrap().next);

    let mut iter1 = head;
    let mut iter2 = reversed_second_half_start.clone();
    while iter2.is_some() {
        if iter1.clone().unwrap().val != iter2.clone().unwrap().val {
            return false;
        }

        iter1 = iter1.unwrap().next;
        iter2 = iter2.unwrap().next;
    }

    reverse_list(reversed_second_half_start);

    true
}

pub fn reverse_list(mut head: Link<i32>) -> Link<i32> {
    let mut prev = None;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

pub fn get_middle_node(head: Link<i32>) -> Link<i32> {
    let (mut iter1, mut iter2) = (&head, &head);
    while iter2.is_some() && iter2.as_ref().unwrap().next.is_some() {
        iter1 = &iter1.as_ref().unwrap().next;
        iter2 = &iter2.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    iter1.clone()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let list = List::from_vec(vec![1]);
        assert_eq!(is_palindrome(list.head.clone()), true);
        assert_eq!(is_palindrome_optimized_space(list.head.clone()), true);
    }

    #[test]
    fn example_2() {
        let list = List::from_vec(vec![1, 3, 4, 3, 1]);
        assert_eq!(is_palindrome(list.head.clone()), true);
        assert_eq!(is_palindrome_optimized_space(list.head.clone()), true);
        let list = List::from_vec(vec![1, 3, 4, 4, 3, 1]);
        assert_eq!(is_palindrome(list.head.clone()), true);
        assert_eq!(is_palindrome_optimized_space(list.head.clone()), true);
    }

    #[test]
    fn example_3() {
        let list = List::from_vec(vec![1, 3, 4, 1, 3, 1]);
        assert_eq!(is_palindrome(list.head.clone()), false);
        assert_eq!(is_palindrome_optimized_space(list.head.clone()), true);
        let list = List::from_vec(vec![1, 3]);
        assert_eq!(is_palindrome(list.head.clone()), false);
        assert_eq!(is_palindrome_optimized_space(list.head.clone()), true);
    }
}
