pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            p += 1;
        }
    }
    v.swap(0, p);
    p
}

pub fn pivot3<T: PartialOrd>(v: &mut [T]) -> (usize, usize) {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            p += 1;
        }
    }
    v.swap(0, p);
    (p, p)
}

pub fn quick_sort<T: PartialOrd + Send + 'static>(v: &mut [T]) {
    if v.len() < 2 {
        return;
    }
    let p = pivot(v);
    let (left, right) = v.split_at_mut(p);

    rayon::join(|| quick_sort(left), || quick_sort(&mut right[1..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![3, 4, 7, 5, 2, 1, 9];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 7, 9]);
    }
}
