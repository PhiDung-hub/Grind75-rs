pub fn merge_sort<T: Ord + Copy>(mut v: Vec<T>) -> Vec<T> {
    if v.len() < 2 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());

    let b = v.split_off(v.len() / 2);

    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();

    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        match b_peek.take() {
                            Some(val) => res.push(val),
                            None => panic!("None value found during function call, CODE 1"),
                        }
                        b_peek = b_it.next();
                    } else {
                        match a_peek.take() {
                            Some(val) => res.push(val),
                            None => panic!("None value found during function call, CODE 2"),
                        }
                        a_peek = a_it.next();
                    }
                }
                // right vector (b) is exhausted
                None => {
                    match a_peek.take() {
                        Some(val) => res.push(val),
                        None => panic!("None value found during function call, CODE 3"),
                    }
                    res.extend(a_it);
                    return res;
                }
            },
            // left vector (a) is exhausted
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort1() {
        let v = vec![4, 6, 1, 8, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 13])
    }

    #[test]
    fn test_merge_sort2() {
        let v = vec![9, 6, 1, 18, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 6, 9, 13, 18])
    }
    
    #[test]
    fn test_merge_sort3() {
        let v = vec![1, 3, 4, 8, 13, 6];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 13])
    }
}
