// Problem: https://leetcode.com/problems/binary-tree-right-side-view/
use crate::structs::binary_tree::*;
use std::collections::VecDeque;

pub fn right_side_view(root: NodeRef) -> Vec<i32> {
    let mut result = vec![];

    if root.is_none() {
        return result;
    }

    let mut visitor = VecDeque::<NodeRef>::new();
    visitor.push_back(root);
    while !visitor.is_empty() {
        let val = visitor.back().cloned().unwrap().unwrap().borrow().val;
        result.push(val);
        let n = visitor.len();
        for _ in 0..n {
            let node_rc = visitor.pop_front().unwrap().unwrap();
            let node = &*node_rc.borrow();
            let TreeNode { left, right, .. } = node;
            if left.is_some() {
                visitor.push_back(left.clone());
            }
            if right.is_some() {
                visitor.push_back(right.clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let tree = from_bfs([Some(1), Some(2), Some(3), None, Some(5), None, Some(4)].to_vec());
        assert_eq!(right_side_view(tree), [1, 3, 4]);
    }

    #[test]
    fn example_2() {
        let tree = from_bfs([Some(1), Some(2), Some(3), None, Some(5), None].to_vec());
        assert_eq!(right_side_view(tree), [1, 3, 5]);
    }

    #[test]
    fn example_3() {
        let tree = from_bfs([Some(1), None, Some(3)].to_vec());
        assert_eq!(right_side_view(tree), [1, 3]);
    }

    #[test]
    fn example_4() {
        let tree = from_bfs([].to_vec());
        assert_eq!(right_side_view(tree), []);
    }
}
