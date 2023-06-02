// Problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
use crate::structs::binary_tree::*;
use std::collections::*;

pub fn lowest_common_ancestor(root: NodeRef, p: NodeRef, q: NodeRef) -> NodeRef {
    let mut parent_map = HashMap::<i32, NodeRef>::new();
    let mut explore_stack = vec![root];
    while let Some(node) = explore_stack.pop() {
        let TreeNode { left, right, .. } = &*node.as_ref()?.borrow();
        if left.is_some() {
            explore_stack.push(left.clone());
            parent_map.insert(left.as_ref()?.borrow().val, node.clone());
        }
        if right.is_some() {
            explore_stack.push(right.clone());
            parent_map.insert(right.as_ref()?.borrow().val, node.clone());
        }
    }

    let mut p_ref = &p;
    let mut p_ancestors = HashSet::<i32>::new();

    loop {
        let p_val = p_ref.as_ref()?.borrow().val;
        p_ancestors.insert(p_val);
        match parent_map.get(&p_val) {
            Some(node_ref) => p_ref = node_ref,
            None => break,
        }
    }

    let mut q_ref = &q;
    loop {
        let q_val = q_ref.as_ref()?.borrow().val;
        if p_ancestors.contains(&q_val) {
            return q_ref.clone();
        }
        match parent_map.get(&q_val) {
            Some(node_ref) => q_ref = node_ref,
            None => break,
        }
    }

    unreachable!()
}

pub fn lowest_common_ancestor_recursive(root: NodeRef, p: NodeRef, q: NodeRef) -> NodeRef {
    if root == p || root == q {
        return root;
    }

    let TreeNode { left, right, .. } = &*root.as_ref()?.borrow();
    let next_left = lowest_common_ancestor_recursive(left.clone(), p.clone(), q.clone());
    let next_right = lowest_common_ancestor_recursive(right.clone(), p, q);

    match (next_left.is_some(), next_right.is_some()) {
        (true, true) => root.clone(),
        (false, false) => None,
        (true, false) => next_left,
        (false, true) => next_right,
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let tree = from_bfs(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = find(&tree, 5);
        let q = find(&tree, 1);
        let expected_result = find(&tree, 3);
        assert_eq!(lowest_common_ancestor_recursive(tree.clone(), p.clone(), q.clone()), expected_result);
        assert_eq!(lowest_common_ancestor(tree, p, q), expected_result);
    }

    #[test]
    fn example_2() {
        let tree = from_bfs(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = find(&tree, 5);
        let q = find(&tree, 4);
        let expected_result = find(&tree, 5);
        assert_eq!(lowest_common_ancestor_recursive(tree.clone(), p.clone(), q.clone()), expected_result);
        assert_eq!(lowest_common_ancestor(tree, p, q), expected_result);
    }

    #[test]
    fn example_3() {
        let tree = from_bfs(vec![Some(1), Some(2)]);
        let p = find(&tree, 1);
        let q = find(&tree, 2);
        let expected_result = find(&tree, 1);
        assert_eq!(lowest_common_ancestor_recursive(tree.clone(), p.clone(), q.clone()), expected_result);
        assert_eq!(lowest_common_ancestor(tree, p, q), expected_result);
    }

    #[test]
    fn assymetric_case() {
        let tree = from_bfs(vec![Some(-1), Some(0), Some(3), Some(-2), Some(4), None, None, Some(8)]);
        let p = find(&tree, 8);
        let q = find(&tree, 0);
        let expected_result = find(&tree, 0);
        assert_eq!(lowest_common_ancestor_recursive(tree.clone(), p.clone(), q.clone()), expected_result);
        assert_eq!(lowest_common_ancestor(tree, p, q), expected_result);
    }
}
