// Problem: https://leetcode.com/problems/validate-binary-search-tree/
use crate::structs::binary_tree::*;

pub fn is_valid_bst(root: NodeRef) -> bool {
    let mut stack_node_with_bounds = vec![(root, None, None)];
    while let Some((node_ref, left_bound, right_bound)) = stack_node_with_bounds.pop() {
        let node_rc = node_ref.unwrap();
        let TreeNode { val, left, right } = &*node_rc.borrow();

        if left_bound.is_some() && *val <= left_bound.unwrap() {
            return false;
        }

        if right_bound.is_some() && *val >= right_bound.unwrap() {
            return false;
        }

        if left.is_some() {
            stack_node_with_bounds.push((left.clone(), left_bound, Some(*val)));
        }

        if right.is_some() {
            stack_node_with_bounds.push((right.clone(), Some(*val), right_bound));
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let tree = TreeNode::from_bfs(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(is_valid_bst(tree), true);
    }

    #[test]
    fn example_2() {
        let tree = TreeNode::from_bfs(vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert_eq!(is_valid_bst(tree), false);
    }

    #[test]
    fn example_3() {}
}
