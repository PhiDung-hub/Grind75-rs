// Problem: https://leetcode.com/problems/middle-of-the-linked-list/
use crate::structs::list::*;

pub fn middle_node(head: Link<i32>) -> Link<i32> {
    let (mut iter1, mut iter2) = (&head, &head);
    while iter2.is_some() && iter2.as_ref().unwrap().next.is_some() {
        iter1 = &iter1.as_ref().unwrap().next;
        iter2 = &iter2.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    iter1.clone()
}

pub fn middle_node_2_pass(head: Link<i32>) -> Link<i32> {
    let mut len = 0;
    let mut iter = &head;

    while let Some(node) = iter {
        iter = &node.next;
        len += 1;
    }

    let mut head = head;
    for _ in 0..len / 2 {
        head = head.unwrap().next;
    }
    head
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let list = List::from_vec(vec![1, 2, 3]);
        assert_eq!(middle_node(list.head.clone()).unwrap().val, 2);
        assert_eq!(middle_node_2_pass(list.head.clone()).unwrap().val, 2);
    }

    #[test]
    fn example_2() {
        let list = List::from_vec(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(middle_node(list.head.clone()).unwrap().val, 4);
        assert_eq!(middle_node_2_pass(list.head.clone()).unwrap().val, 4);
    }

    #[test]
    fn example_3() {
        let list = List::from_vec(vec![1]);
        assert_eq!(middle_node(list.head.clone()).unwrap().val, 1);
        assert_eq!(middle_node_2_pass(list.head.clone()).unwrap().val, 1);
    }
}
