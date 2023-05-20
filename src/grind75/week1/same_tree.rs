// Problem: https://leetcode.com/problems/same-tree/description/
use crate::structs::binary_tree::*;

pub fn is_same_tree_builtin(p: NodeRef, q: NodeRef) -> bool {
    p == q // compiler performs recursion matching 
}

pub fn is_same_tree(p: NodeRef, q: NodeRef) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let TreeNode {
                left: p_left,
                right: p_right,
                val: p_val,
            } = &*p.borrow();
            let TreeNode {
                left: q_left,
                right: q_right,
                val: q_val,
            } = &*q.borrow();
            p_val == q_val && is_same_tree(p_left.clone(), q_left.clone()) && is_same_tree(p_right.clone(), q_right.clone())
        }
        _ => false,
    }
}

pub fn is_same_tree_iterative(p: NodeRef, q: NodeRef) -> bool {
    let mut stack = vec![(p, q)];
    while let Some((p_node, q_node)) = stack.pop() {
        match (p_node, q_node) {
            (None, None) => {}
            (Some(p), Some(q)) => {
                let TreeNode {
                    left: p_left,
                    right: p_right,
                    val: p_val,
                } = &*p.borrow();
                let TreeNode {
                    left: q_left,
                    right: q_right,
                    val: q_val,
                } = &*q.borrow();

                if p_val != q_val {
                    return false;
                }

                stack.push((p_left.clone(), q_left.clone()));
                stack.push((p_right.clone(), q_right.clone()));
            }
            _ => {
                return false;
            }
        };
    }
    true
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn equal_cases() {
        let tree1 = from_bfs(vec![Some(2), Some(3), None, Some(1)]);
        let tree2 = from_bfs(vec![Some(2), Some(3), None, Some(1)]);
        assert_eq!(is_same_tree(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_same_tree_builtin(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_same_tree_iterative(tree1.clone(), tree2.clone()), true);

        let tree1 = from_bfs(vec![]);
        let tree2 = from_bfs(vec![]);
        assert_eq!(is_same_tree(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_same_tree_builtin(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_same_tree_iterative(tree1.clone(), tree2.clone()), true);
    }

    #[test]
    fn different_nodes() {
        let tree1 = from_bfs(vec![Some(2), Some(3), Some(2), Some(1)]);
        let tree2 = from_bfs(vec![Some(2), Some(3), None, Some(1)]);
        assert_eq!(is_same_tree(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_same_tree_builtin(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_same_tree_iterative(tree1.clone(), tree2.clone()), false);
    }

    #[test]
    fn different_values() {
        let tree1 = from_bfs(vec![Some(2), Some(3), Some(2), Some(1)]);
        let tree2 = from_bfs(vec![Some(2), Some(3), Some(4), Some(1)]);
        assert_eq!(is_same_tree(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_same_tree_builtin(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_same_tree_iterative(tree1.clone(), tree2.clone()), false);

        let tree1 = from_bfs(vec![Some(0), Some(-5)]);
        let tree2 = from_bfs(vec![Some(0), Some(-8)]);
        assert_eq!(is_same_tree(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_same_tree_builtin(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_same_tree_iterative(tree1.clone(), tree2.clone()), false);
    }
}
