// Problem: https://leetcode.com/problems/balanced-binary-tree/
use crate::structs::binary_tree::*;

pub fn is_balanced(root: NodeRef) -> bool {
    fn dfs(root: NodeRef) -> i32 {
        match root {
            Some(root) => {
                let left = dfs(root.borrow().left.clone());
                let right = dfs(root.borrow().right.clone());
                if (left - right).abs() > 1 || left == -1 || right == -1 {
                    return -1;
                }
                std::cmp::max(left, right) + 1
            }
            None => 0,
        }
    }
    dfs(root) != -1
}

// pub fn is_balanced_iterative(root: NodeRef) -> bool {
//     fn dfs(root: NodeRef) {
//         let mut stack: Vec<NodeRef> = Vec::new();
//         stack.push(root.clone());
//         while let Some(node_ref) = stack.pop() {
//             if let Some(node_ref) = node_ref {
//                 let node = &mut *node_ref.borrow_mut();
//                 let TreeNode { left, right, .. } = node;
//                 if left.is_none() && right.is_none() {
//
//                 }
//                 stack.push(right.clone()); 
//                 stack.push(left.clone());
//             }
//         }
//     }
//     false
// }

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
