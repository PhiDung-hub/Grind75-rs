// Problem: https://leetcode.com/problems/climbing-stairs/

pub fn climbing_stairs(n: i32) -> i32 {
    if n < 3 {
        return n;
    }
    let mut prev2_climb = 1;
    let mut prev_climb = 2;

    let mut result = 0;
    for _ in 3..=n {
        result = prev_climb + prev2_climb;
        prev2_climb = prev_climb;
        prev_climb = result;
    }
    return result;

}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn base_cases() {
        assert_eq!(climbing_stairs(1), 1);
        assert_eq!(climbing_stairs(2), 2);
        
    }

    #[test]
    fn example_2() {
        assert_eq!(climbing_stairs(3), 3);
        assert_eq!(climbing_stairs(4), 5);
    }

    #[test]
    fn example_3() {
    }
}
