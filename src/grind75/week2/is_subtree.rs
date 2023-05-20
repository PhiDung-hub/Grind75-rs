// Problem: https://leetcode.com/problems/subtree-of-another-tree/
use crate::structs::binary_tree::*;

fn is_same_tree(p: &NodeRef, q: &NodeRef) -> bool {
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
            p_val == q_val && is_same_tree(p_left, q_left) && is_same_tree(p_right, q_right)
        }
        _ => false,
    }
}

pub fn is_subtree(root: NodeRef, sub_root: NodeRef) -> bool {
    match (root.clone(), sub_root.clone()) {
        (None, _) => false,
        (Some(_), None) => true,
        (Some(root_rc), Some(_)) => {
            let root_node = &*root_rc.borrow();

            if is_same_tree(&root, &sub_root) {
                return true;
            }

            is_subtree(root_node.left.clone(), sub_root.clone()) || is_subtree(root_node.right.clone(), sub_root.clone())
        }
    }
}

pub fn is_subtree_builtin(root: NodeRef, sub_root: NodeRef)  -> bool{
    if sub_root.is_none() || root == sub_root {
        return true;
    }
    if let Some(node) = root {
        let node = node.borrow();
        is_subtree(node.left.clone(), sub_root.clone()) || is_subtree(node.right.clone(), sub_root.clone())
    } else {
        return false;
    }
}

pub fn is_subtree_improved(root: NodeRef, sub_root: NodeRef) -> bool {
    if sub_root.is_none() {
        return true;
    }

    let result = &mut false;
    fn check_sub_tree(root: &NodeRef, sub_root: &NodeRef, result: &mut bool) {
        if root.is_none() {
            return;
        }

        let root_rc = root.as_deref().unwrap();
        let root_node = &*root_rc.borrow();

        check_sub_tree(&root_node.left, sub_root, result);
        check_sub_tree(&root_node.right, sub_root, result);

        // NOTE: Optimization step, reduce call stack to `is_same_tree()`. sub_root can't be None
        // because pre-checked in first line of the main function.
        if root_node.val != sub_root.as_deref().unwrap().borrow().val || *result {
            return;
        }

        *result = is_same_tree(root, sub_root);
    }

    check_sub_tree(&root, &sub_root, result);
    *result
}

// Representation including null
pub fn to_preorder_repr(root: NodeRef) -> Vec<Option<i32>> {
    if root.is_none() {
        return vec![];
    }
    let mut result = vec![];
    let mut explore_stack = vec![root];
    while let Some(node_ref) = explore_stack.pop() {
        if let Some(node_rc) = node_ref {
            let node = &*node_rc.borrow();

            let TreeNode { left, right, val } = node;
            result.push(Some(*val));

            explore_stack.push(right.clone());
            explore_stack.push(left.clone());
        } else {
            result.push(None);
        }
    }

    result
}

pub fn is_subtree_iterative_optimized(root: NodeRef, sub_root: NodeRef) -> bool {
    let root_repr = to_preorder_repr(root);
    let sub_root_repr = to_preorder_repr(sub_root);
    fn create_lsp_table(needle: &Vec<Option<i32>>) -> Vec<usize> {
        if needle.is_empty() {
            return vec![];
        }

        let mut lsp_table = Vec::with_capacity(needle.len());
        lsp_table.push(0);

        for &needle_val in &needle[1..] {
            let mut distance = *lsp_table.last().unwrap();

            while distance > 0 && needle_val != needle[distance] {
                distance = lsp_table[distance - 1];
            }

            if needle_val == needle[distance] {
                distance += 1;
            }
            lsp_table.push(distance);
        }

        lsp_table
    }

    fn kmp_find(haystack: &Vec<Option<i32>>, needle: &Vec<Option<i32>>) -> bool {
        if needle.is_empty() {
            return true;
        }
        if needle.len() > haystack.len() {
            return false;
        }
        let lsp_table = create_lsp_table(needle);
        let mut needle_position: usize = 0;

        for (_, &val) in haystack.iter().enumerate() {
            while needle_position > 0 && val != needle[needle_position] {
                needle_position = lsp_table[needle_position - 1];
            }

            if val == needle[needle_position] {
                needle_position += 1;

                if needle_position == needle.len() {
                    // let found_pos = idx - needle_position + 1;
                    return true;
                }
            }
        }
        false
    }

    kmp_find(&root_repr, &sub_root_repr)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn equal_case_1() {
        let tree1 = from_bfs(vec![Some(1), None, Some(2), Some(3), None, Some(1)]);
        let tree2 = from_bfs(vec![Some(2), Some(3), None, Some(1)]);
        assert_eq!(is_subtree(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_improved(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_iterative_optimized(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_builtin(tree1.clone(), tree2.clone()), true);
    }

    #[test]
    fn equal_case_2() {
        let tree1 = from_bfs(vec![Some(1)]);
        let tree2 = from_bfs(vec![]);
        assert_eq!(is_subtree(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_improved(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_iterative_optimized(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_builtin(tree1.clone(), tree2.clone()), true);
    }

    #[test]
    fn equal_case_3() {
        let tree1 = from_bfs(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let tree2 = from_bfs(vec![Some(4), Some(1), Some(2)]);
        assert_eq!(is_subtree(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_improved(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_iterative_optimized(tree1.clone(), tree2.clone()), true);
        assert_eq!(is_subtree_builtin(tree1.clone(), tree2.clone()), true);
    }

    #[test]
    fn different_nodes() {
        let tree1 = from_bfs(vec![Some(2), Some(3), Some(2), Some(1)]);
        let tree2 = from_bfs(vec![Some(2), Some(3), None, Some(1)]);
        assert_eq!(is_subtree(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_subtree_improved(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_subtree_iterative_optimized(tree1.clone(), tree2.clone()), false);
    }

    #[test]
    fn different_values_1() {
        let tree1 = from_bfs(vec![Some(2), Some(3), Some(2), Some(1)]);
        let tree2 = from_bfs(vec![Some(2), Some(3), Some(4), Some(1)]);
        assert_eq!(is_subtree(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_subtree_improved(tree1.clone(), tree2.clone()), false);
        assert_eq!(is_subtree_iterative_optimized(tree1.clone(), tree2.clone()), false);
    }
}
