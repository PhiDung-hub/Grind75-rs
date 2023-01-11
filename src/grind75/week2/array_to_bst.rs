// Problem: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
use crate::structs::binary_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> NodeRef {
    if nums.is_empty() {
        return None;
    }

    let (smaller_half, larger_half) = nums.split_at(nums.len() / 2);
    let root_rc = Rc::new(RefCell::new(TreeNode::new(larger_half[0])));

    root_rc.borrow_mut().left = sorted_array_to_bst(smaller_half.to_vec());
    root_rc.borrow_mut().right = sorted_array_to_bst(larger_half[1..].to_vec());
    Some(root_rc)
}

pub fn sorted_array_to_bst_iterative(nums: Vec<i32>) -> NodeRef {
    let len = nums.len();

    let root_rc = Rc::new(RefCell::new(TreeNode::new(nums[(len - 1) / 2])));
    let mut stack = vec![(0, len - 1, root_rc.clone())];

    while let Some((start, end, node_rc)) = stack.pop() {
        let mid = start + (end - start) / 2;
        if start < mid {
            let end = mid - 1;
            let new_node_rc = Rc::new(RefCell::new(TreeNode::new(nums[start + (end - start) / 2])));
            stack.push((start, end, new_node_rc.clone()));
            node_rc.borrow_mut().left = Some(new_node_rc);
        }

        if end > mid {
            let start = mid + 1;
            let new_node_rc = Rc::new(RefCell::new(TreeNode::new(nums[start + (end - start) / 2])));
            stack.push((start, end, new_node_rc.clone()));
            node_rc.borrow_mut().right = Some(new_node_rc);
        }
    }

    Some(root_rc)
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
