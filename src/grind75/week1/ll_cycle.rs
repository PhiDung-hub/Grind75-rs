use crate::structs::too_many_ll::safe_deque::*;
use std::collections::HashSet;

pub fn has_cycle(head: Link<i32>) -> bool {
    let mut cache = HashSet::new();
    let mut next = head;
    while let Some(node) = next {
        let mut n = node.borrow_mut();
        if !cache.insert(n.elem) {
            return true;
        }
        next = n.next.take();
    }
    false
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {

    }
}
