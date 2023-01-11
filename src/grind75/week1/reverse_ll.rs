// Problem: https://leetcode.com/problems/reverse-linked-list/
use crate::structs::list::*;

pub fn reverse_list(head: Link<i32>) -> Link<i32> {
    let mut prev = None;
    let mut head = head;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

pub fn reverse_list_recursive(head: Link<i32>) -> Link<i32> {
    fn recurse(head: Link<i32>, prev: Link<i32>) -> Link<i32> {
        match head {
            Some(mut node) => {
                let next_head = node.next.take();
                node.next = prev;
                recurse(next_head, Some(node))
            }
            None => prev,
        }
    }

    recurse(head, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let list = List::from_vec(vec![1, 2, 3]);
        let mut reverse_list = List::with_head(reverse_list(list.head.clone()));
        assert_eq!(reverse_list.to_vec(), vec![3, 2, 1]);
        let list = List::from_vec(vec![1, 2, 3]);
        let mut reverse_list = List::with_head(reverse_list_recursive(list.head.clone()));
        assert_eq!(reverse_list.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn empty_case() {
        let list = List::from_vec(vec![]);
        let mut reverse_list = List::with_head(reverse_list(list.head.clone()));
        assert_eq!(reverse_list.to_vec(), vec![]);
        let list = List::from_vec(vec![]);
        let mut reverse_list = List::with_head(reverse_list_recursive(list.head.clone()));
        assert_eq!(reverse_list.to_vec(), vec![]);
    }

    #[test]
    fn basic_case_2() {
        let list = List::from_vec(vec![3, 2, 4, 1, 5]);
        let mut reverse_list = List::with_head(reverse_list(list.head.clone()));
        assert_eq!(reverse_list.to_vec(), vec![5, 1, 4, 2, 3]);
        let list = List::from_vec(vec![3, 2, 4, 1, 5]);
        let mut reverse_list = List::with_head(reverse_list_recursive(list.head.clone()));
        assert_eq!(reverse_list.to_vec(), vec![5, 1, 4, 2, 3]);
    }
}
