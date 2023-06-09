// Problem: https://leetcode.com/problems/invert-binary-tree
use crate::structs::binary_tree::*;
use std::collections::VecDeque;

trait TreeNodeSwap {
    fn swap(&mut self);
}

impl TreeNodeSwap for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }
}

pub fn invert_tree(root: NodeRef) -> NodeRef {
    root.map(|node_ref| {
        let node = &mut *node_ref.borrow_mut();
        node.swap();
        let TreeNode { left, right, .. } = node;
        invert_tree(left.clone());
        invert_tree(right.clone());
        node_ref.clone()
    })
}

pub fn invert_tree_iterative(root: NodeRef) -> NodeRef {
    root.as_ref()?;

    let mut stack: VecDeque<NodeRef> = VecDeque::new();
    stack.push_back(root.clone());
    while !stack.is_empty() {
        let node_rc = stack.pop_front().unwrap().unwrap();
        let node = &mut *node_rc.borrow_mut();
        let TreeNode { left, right, .. } = node;

        if left.is_some() {
            stack.push_back(left.clone());
        }
        if right.is_some() {
            stack.push_back(right.clone());
        }
        node.swap();
    }
    root
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn typical_case() {
        let tree = from_bfs(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
        assert_eq!(bfs(invert_tree(tree)), vec![4, 7, 2, 9, 6, 3, 1]);
        let tree = from_bfs(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
        assert_eq!(bfs(invert_tree_iterative(tree)), vec![4, 7, 2, 9, 6, 3, 1]);
    }

    #[test]
    fn simple_case() {
        let tree = from_bfs(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(bfs(invert_tree(tree)), vec![1, 3, 2]);
        let tree = from_bfs(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(bfs(invert_tree_iterative(tree)), vec![1, 3, 2]);
    }

    #[test]
    fn partially_null_case() {
        let tree = from_bfs(vec![Some(2), Some(3), None, Some(1)]);
        assert_eq!(to_bfs_option(invert_tree(tree)), vec![Some(2), None, Some(3), None, Some(1)]);
        let tree = from_bfs(vec![Some(2), Some(3), None, Some(1)]);
        assert_eq!(to_bfs_option(invert_tree_iterative(tree)), vec![Some(2), None, Some(3), None, Some(1)]);
    }

    #[test]
    fn empty_tree() {
        let tree = from_bfs(vec![]);
        assert_eq!(bfs(invert_tree(tree)), vec![]);
        let tree = from_bfs(vec![]);
        assert_eq!(bfs(invert_tree_iterative(tree)), vec![]);
    }
}
