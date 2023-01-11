#[derive(Default)]
pub struct MyQueue {
    stack_front: Vec<i32>,
    stack_back: Vec<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.stack_back.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.move_back_to_front();
        self.stack_front.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.move_back_to_front();
        *self.stack_front.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_back.is_empty() && self.stack_front.is_empty()
    }

    // Amortized O(1)
    fn move_back_to_front(&mut self) {
        if self.stack_front.is_empty() {
            while let Some(x) = self.stack_back.pop() {
                self.stack_front.push(x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.peek(), 2);
        assert_eq!(queue.pop(), 2);
    }
}
