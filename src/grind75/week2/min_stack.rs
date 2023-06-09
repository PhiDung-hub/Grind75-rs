// Problem: https://leetcode.com/problems/min-stack/

struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            data: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.min.is_empty() || val <= self.get_min() {
            self.min.push(val);
        }
        self.data.push(val);
    }

    fn pop(&mut self) {
        if self.top() == self.get_min() {
            self.min.pop();
        }
        self.data.pop();
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-1);
        assert_eq!(min_stack.get_min(), -2);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
