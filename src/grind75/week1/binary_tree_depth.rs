// Problem: https://leetcode.com/problems/maximum-depth-of-binary-tree/
use crate::structs::binary_tree::*;

pub fn max_depth(root: NodeRef) -> i32 {
    let mut stack = vec![(root, 0)];
    let mut max_depth = 0;
    while let Some((node_ref, mut depth)) = stack.pop() {
        if let Some(node_rc) = node_ref {
            let node = &*node_rc.borrow();
            depth += 1;
            let TreeNode { left, right, .. } = node;
            max_depth = max_depth.max(depth);
            stack.push((left.clone(), depth));
            stack.push((right.clone(), depth));
        }
    }
    max_depth
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {}

    #[test]
    fn example_2() {}
}
