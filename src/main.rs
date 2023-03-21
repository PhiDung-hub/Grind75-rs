pub fn solution(arr: &mut Vec<i32>) -> i32 {
    arr.sort();
    let (mut lo, mut hi) = (0, arr.len() - 1);
    while lo <= hi {
        let (lo_abs, hi_abs) = (arr[lo].abs(), arr[hi].abs());
        if lo_abs > hi_abs {
            lo += 1;
        } else if lo_abs < hi_abs {
            hi -= 1;
        } else {
            return lo_abs;
        }
    }

    0
}

pub fn main() {
    let mut vec = vec![3, 2, -2, 5, -3];
    assert_eq!(solution(&mut vec), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut vec = vec![3, 2, -2, 5, -3];
        assert_eq!(solution(&mut vec), 3);
    }

    #[test]
    fn example_2() {}

    #[test]
    fn example_3() {}
}
