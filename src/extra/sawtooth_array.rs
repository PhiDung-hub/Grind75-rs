pub fn sawtooth_count(array: &[i32]) -> usize {
    let n = array.len();

    if n < 2 {
        panic!("Invalid input array!");
    }

    let mut sawtooth_counts = 0;

    let mut diff = array[1] - array[0];
    let mut subarray_len = if diff == 0 { 0 } else { 2 };

    for i in 2..n {
        let this_diff = array[i] - array[i - 1];

        if (diff < 0 && this_diff > 0) || (diff > 0 && this_diff < 0)  {
            // a sawtooth pattern found
            subarray_len += 1;
        } else {
            if subarray_len > 0 {
                sawtooth_counts += subarray_len * (subarray_len - 1) / 2;
            }
            subarray_len = if this_diff == 0 { 0 } else { 2 };
        }

        diff = this_diff;
    }

    if subarray_len > 0 {
        sawtooth_counts += subarray_len * (subarray_len - 1) / 2;
    }

    sawtooth_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(sawtooth_count(&[9, 8, 7, 6, 5]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(sawtooth_count(&[10, 10, 10]), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(sawtooth_count(&[1, 2, 1, 3, 2]), 10);
    }

    #[test]
    fn test4() {
        assert_eq!(sawtooth_count(&[944846388, 993352669, 473719581, 896064794, 646066085, 727243804]), 15);
    }
}
