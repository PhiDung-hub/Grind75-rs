use std::cell::RefCell;
use std::collections::*;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Hash for TreeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
        match &self.left {
            Some(node_rc) => {
                let node = &*node_rc.borrow();
                node.hash(state)
            }
            None => (false, 0).hash(state),
        }
        match &self.right {
            Some(node_rc) => {
                let node = &*node_rc.borrow();
                node.hash(state)
            }
            None => (false, 1).hash(state),
        }
    }
}

pub type NodeRef = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn find(root: &NodeRef, target: i32) -> NodeRef {
    let mut stack = vec![root.clone()];

    while let Some(node_ref) = stack.pop() {
        if let Some(node_rc) = node_ref {
            let node = &*node_rc.borrow();
            let TreeNode { val, left, right } = node;
            if *val == target {
                return Some(node_rc.clone());
            }
            stack.push(right.clone());
            stack.push(left.clone());
        }
    }

    None
}
pub fn dfs(root: NodeRef) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut stack = vec![root];
    while let Some(node_ref) = stack.pop() {
        if let Some(node_rc) = node_ref {
            let node = &*node_rc.borrow();
            let TreeNode { val, left, right } = node;
            result.push(*val);
            stack.push(right.clone());
            stack.push(left.clone());
        }
    }
    result
}

pub fn bfs(root: NodeRef) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while let Some(node_ref) = queue.pop_front() {
        if let Some(node_rc) = node_ref {
            let node = &*node_rc.borrow();
            let TreeNode { val, left, right } = node;
            result.push(*val);
            queue.push_back(left.clone());
            queue.push_back(right.clone());
        }
    }
    result
}

// Value None => Node is empty
pub fn from_bfs(v: Vec<Option<i32>>) -> NodeRef {
    if v.is_empty() {
        return None;
    } else if v[0].is_none() {
        panic!("Invalid array")
    }

    let root_ref: NodeRef = Some(Rc::new(RefCell::new(TreeNode::new(v[0]?))));
    let mut prev_level_nodes: VecDeque<NodeRef> = VecDeque::new();
    prev_level_nodes.push_front(root_ref.clone());

    let mut node_in_prev_level = 1;
    let mut i = 1;

    'outer_while_loop: loop {
        let mut node_in_this_level = 0;

        for _ in 0..node_in_prev_level {
            if i >= v.len() {
                break 'outer_while_loop;
            }

            let node_rc = prev_level_nodes.pop_back()??;
            let node = &mut *node_rc.borrow_mut();

            // left arm
            if let Some(value) = v[i] {
                let left_child_ref = Some(Rc::new(RefCell::new(TreeNode::new(value))));
                node.left = left_child_ref.clone();
                node_in_this_level += 1;
                prev_level_nodes.push_front(left_child_ref);
            } else {
                node.left = None;
            }

            // input vector is exhausted
            if i >= v.len() - 1 {
                break 'outer_while_loop;
            }

            // right arm
            if let Some(value) = v[i + 1] {
                let right_child_ref = Some(Rc::new(RefCell::new(TreeNode::new(value))));
                node.right = right_child_ref.clone();
                node_in_this_level += 1;
                prev_level_nodes.push_front(right_child_ref);
            } else {
                node.right = None;
            }

            i += 2;
        }

        if node_in_this_level == 0 && i < v.len() {
            panic!("Invalid input vector, a None node can't have children");
        }

        node_in_prev_level = node_in_this_level;
    }

    root_ref
}

// Leet code representation
pub fn to_bfs_option(root: NodeRef) -> Vec<Option<i32>> {
    if root.is_none() {
        return vec![];
    }
    let mut result = vec![root.as_ref().map(|node| node.borrow().val)];
    let mut queue: VecDeque<NodeRef> = VecDeque::new();
    queue.push_back(root);
    while let Some(node_ref) = queue.pop_front() {
        let node_rc = node_ref.unwrap();
        let node = &*node_rc.borrow();

        let TreeNode { left, right, .. } = node;
        if left.is_some() {
            queue.push_back(left.clone());
        }
        if right.is_some() {
            queue.push_back(right.clone());
        }

        if left.is_some() || right.is_some() {
            result.push(left.as_ref().map(|node| node.borrow().val));
            result.push(right.as_ref().map(|node| node.borrow().val));
        }
    }
    // clean the last none if exists.
    while result.last().unwrap().is_none() {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bfs_typical_case() {
        let root_ref = from_bfs(vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), Some(7)]);
        assert_eq!(bfs(root_ref.clone()), vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(dfs(root_ref), vec![1, 2, 4, 6, 7, 3, 5]);
    }

    #[test]
    #[should_panic(expected = "Invalid array")]
    fn from_bfs_wrong_input_1() {
        from_bfs(vec![None, Some(1), Some(2)]);
    }

    #[test]
    #[should_panic(expected = "Invalid input vector, a None node can't have children")]
    fn from_bfs_wrong_input_2() {
        from_bfs(vec![Some(1), None, None, None]);
    }

    #[test]
    #[should_panic(expected = "Invalid input vector, a None node can't have children")]
    fn from_bfs_wrong_input_3() {
        from_bfs(vec![Some(1), None, Some(2), None, None, None]);
    }

    #[test]
    fn test_to_bfs_option() {
        let root = from_bfs(vec![Some(1), None, Some(2), Some(3)]);
        assert_eq!(to_bfs_option(root), vec![Some(1), None, Some(2), Some(3)])
    }
}
