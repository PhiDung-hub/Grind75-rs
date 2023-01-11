// Problem: https://leetcode.com/problems/symmetric-tree/
use crate::structs::binary_tree::*;

pub fn is_symmetric(root: NodeRef) -> bool {
    if root.is_none() {
        return true;
    }
    fn is_symmetric_branch(left: &NodeRef, right: &NodeRef) -> bool {
        match (left, right) {
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
                if p_val != q_val {
                    return false;
                }

                is_symmetric_branch(p_left, q_right) && is_symmetric_branch(q_left, p_right)
            }
            _ => false,
        }
    }

    let root_ref = root.unwrap();
    let root_node = &*root_ref.borrow();
    is_symmetric_branch(&root_node.left, &root_node.right)
}

pub fn is_symmetric_iterative(root: NodeRef) -> bool {
    fn dfs(root: NodeRef, direction: &str) -> Vec<Option<i32>> {
        let mut result: Vec<Option<i32>> = Vec::new();
        let mut stack = vec![root];

        while let Some(node_ref) = stack.pop() {
            if let Some(node_rc) = node_ref {
                let node = &*node_rc.borrow();
                let TreeNode { val, left, right } = node;
                result.push(Some(*val));
                match direction {
                    "left" => {
                        stack.push(right.clone());
                        stack.push(left.clone());
                    }
                    "right" => {
                        stack.push(left.clone());
                        stack.push(right.clone());
                    }
                    _ => panic!("Invalid traversal direction"),
                }
            } else {
                result.push(None);
            }
        }
        result
    }

    let root_ref = root.unwrap();
    let root_node = &*root_ref.borrow();

    let left_tree_vec = dfs(root_node.left.clone(), "left");
    let right_tree_vec = dfs(root_node.right.clone(), "right");
    left_tree_vec == right_tree_vec
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
