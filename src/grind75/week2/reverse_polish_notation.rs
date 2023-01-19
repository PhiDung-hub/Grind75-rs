// Problem: https://leetcode.com/problems/evaluate-reverse-polish-notation/

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut operand_stack: Vec<String> = Vec::new();

    fn is_operand(input: String) -> bool {
        match input.as_str() {
            "-" | "+" | "*" | "/" => true,
            _ => false,
        }
    }

    fn cal(op1: String, op2: String, operator: String) -> i32 {
        let op1: i32 = op1.parse().unwrap();
        let op2: i32 = op2.parse().unwrap();

        match operator.as_str() {
            "-" => op1 - op2,
            "+" => op1 + op2,
            "*" => op1 * op2,
            "/" => op1 / op2,
            _ => panic!("Invalid operator"),
        }
    }

    let mut result = 0;
    for token in tokens {
        if !is_operand(token.clone()) {
            operand_stack.push(token);
        } else {
            let op2 = operand_stack.pop().unwrap();
            let op1 = operand_stack.pop().unwrap();
            result = cal(op1, op2, token);
            operand_stack.push(result.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let tokens = ["2", "1", "+", "3", "*"].map(String::from).to_vec();
        assert_eq!(eval_rpn(tokens), 9);
    }

    #[test]
    fn example_2() {
        let tokens = ["4", "13", "5", "/", "+"].map(String::from).to_vec();
        assert_eq!(eval_rpn(tokens), 6);
    }

    #[test]
    fn example_3() {
        let tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"].map(String::from).to_vec();
        assert_eq!(eval_rpn(tokens), 22);
    }
}
