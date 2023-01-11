// Problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
use crate::structs::binary_tree::*;

pub fn lowest_common_acentor(root: NodeRef, p: NodeRef, q: NodeRef) -> NodeRef {
    let p_val = p.map(|node| node.borrow().val).unwrap();
    let q_val = q.map(|node| node.borrow().val).unwrap();
    let (min, max) = match p_val > q_val {
        true => (q_val, p_val),
        false => (p_val, q_val),
    };

    let mut stack = vec![root];

    while let Some(node) = stack.pop() {
        if let Some(node) = node {
            let node_val = node.borrow().val;

            // Common Ancestor must be unique for binary search tree
            if node_val >= min && node_val <= max {
                return Some(node);
            }

            if node_val > max {
                stack.push(node.borrow().left.clone());
            } else {
                stack.push(node.borrow().right.clone());
            }
        }
    }
    panic!("Not a valid Binary Search Tree");
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
