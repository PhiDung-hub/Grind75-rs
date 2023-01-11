// Problem: https://leetcode.com/problems/binary-tree-level-order-traversal/
use crate::structs::binary_tree::NodeRef;
use std::collections::VecDeque;

pub fn level_order(root: NodeRef) -> Vec<Vec<i32>> {
    let (mut this_level, mut next_level) = (0, 0);

    let mut explore_queue: VecDeque<NodeRef> = VecDeque::new();

    explore_queue.push_back(root);

    let mut result = vec![];

    while let Some(node_rc) = explore_queue.pop_front() {}

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {}

    #[test]
    fn example_2() {}

    #[test]
    fn example_3() {}
}
