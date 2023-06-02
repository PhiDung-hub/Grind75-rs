// Problem: https://leetcode.com/problems/container-with-most-water/

pub fn max_area(height: Vec<i32>) -> i32 {
    if height.len() < 2 {
        panic!("Invalid height input, must have size at least 2");
    }
    let mut max_water = 0;
    let (mut left, mut right) = (0, height.len() - 1);

    while left < right {
        let cur_water = std::cmp::min(height[left], height[right]) * (right - left) as i32;
        if cur_water > max_water {
            max_water = cur_water;
        }

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_water
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let height = [1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec();
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn example_2() {
        let height = [1, 3, 2, 5, 25, 24, 5].to_vec();
        assert_eq!(max_area(height), 24);
    }

    #[test]
    fn example_3() {
        let height = [1, 2, 4, 3].to_vec();
        assert_eq!(max_area(height), 4);
    }
}
