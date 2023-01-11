// Problem: https://leetcode.com/problems/diameter-of-binary-tree/
use crate::structs::binary_tree::*;

pub fn diameter_of_binary_tree(root: NodeRef) -> i32 {
    if root.is_none() {
        return 0;
    }

    fn get_diameter(root: &NodeRef, d: &mut i32) -> i32 {
        match root {
            Some(root_rc) => {
                let node = &*root_rc.borrow();
                let TreeNode { left, right, .. } = node;
                let left_d = get_diameter(&left, d);
                let right_d = get_diameter(&right, d);
                *d = (left_d + right_d).max(*d);
                1 + left_d.max(right_d)
            }
            None => {
                return 0;
            }
        }
    }

    let mut d = 0;
    get_diameter(&root, &mut d);
    d
}

// pub fn diameter_of_binary_tree_iterative(root: NodeRef) -> i32 {
//     if root.is_none() {
//         return 0;
//     }
//
//     let mut explore_stack = vec![(root, false)];
//     let mut max_d = 0;
//     let mut d_map: HashMap<NodeRef, i32> = HashMap::new();
//
//     while let Some((node_ref, seen)) = explore_stack.last_mut() {
//         let node_rc = node_ref.clone().unwrap();
//         let node = &*node_rc.borrow();
//         let TreeNode { left, right, .. } = node;
//         if !*seen {
//             *seen = true;
//             if left.is_some() {
//                 explore_stack.push((left.clone(), false));
//             }
//             if right.is_some() {
//                 explore_stack.push((right.clone(), false));
//             }
//         } else {
//             // TODO: use HashMap to loop and get max height from both tree;
//         }
//     }
//     max_d
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
