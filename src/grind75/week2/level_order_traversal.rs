// Problem: https://leetcode.com/problems/binary-tree-level-order-traversal/
use crate::structs::binary_tree::*;
use std::collections::VecDeque;

pub fn level_order(root: NodeRef) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }
    let (mut this_level, mut next_level) = (1, 0);

    let mut explore_queue: VecDeque<NodeRef> = VecDeque::new();

    explore_queue.push_back(root);

    let mut result = vec![];

    while explore_queue.front().is_some() {
        let mut this_level_nodes = vec![];
        for _ in 0..this_level {
            let node_rc = explore_queue.pop_front().unwrap().unwrap();
            let node = &*node_rc.borrow();
            let TreeNode { left, right, val } = node;
            this_level_nodes.push(*val);
            if left.is_some() {
                explore_queue.push_back(left.clone());
                next_level += 1;
            }
            if right.is_some() {
                explore_queue.push_back(right.clone());
                next_level += 1;
            }
        }
        result.push(this_level_nodes);
        this_level = next_level;
        next_level = 0;
    }

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let target_tree = TreeNode::from_bfs(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(level_order(target_tree), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn example_2() {
        let target_tree = TreeNode::from_bfs(vec![Some(1)]);
        assert_eq!(level_order(target_tree), vec![vec![1]]);
    }

    #[test]
    fn example_3() {
        let target_tree = TreeNode::from_bfs(vec![]);
        assert_eq!(level_order(target_tree), vec![] as Vec<Vec<i32>>);
    }
}
