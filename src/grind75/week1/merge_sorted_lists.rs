// Problem: https://leetcode.com/problems/merge-two-sorted-lists
use crate::structs::list::{Link, Node};

pub fn merge_two_lists(list1: Link<i32>, list2: Link<i32>) -> Link<i32> {
    match (list1, list2) {
        (Some(x), Some(y)) => Some(Box::new(if x.val < y.val {
            Node {
                val: x.val,
                next: merge_two_lists(x.next, Some(y)),
            }
        } else {
            Node {
                val: y.val,
                next: merge_two_lists(Some(x), y.next),
            }
        })),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::list::*;

    #[test]
    fn example_1() {
        let mut list1 = List::from_vec(vec![1, 2, 5]);
        let mut list2 = List::from_vec(vec![2, 4, 6]);
        let mut merge_list = List {
            head: merge_two_lists(list1.head.take(), list2.head.take()),
        };
        assert_eq!(merge_list.to_vec(), vec![1, 2, 2, 4, 5, 6])
    }

    #[test]
    fn example_2() {
        let mut list1 = List::from_vec(vec![1, 3, 5]);
        let mut list2 = List::from_vec(vec![2, 4, 6]);
        let mut merge_list = List {
            head: merge_two_lists(list1.head.take(), list2.head.take()),
        };
        assert_eq!(merge_list.to_vec(), vec![1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn example_3() {
        let mut list1 = List::from_vec(vec![1, 2, 5]);
        let mut list2 = List::from_vec(vec![6]);
        let mut merge_list = List {
            head: merge_two_lists(list1.head.take(), list2.head.take()),
        };
        assert_eq!(merge_list.to_vec(), vec![1, 2, 5, 6])
    }
}
