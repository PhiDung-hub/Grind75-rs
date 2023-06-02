/// Sort a slice where the number of unique element is K.
///
/// Complexity: `O(N+ K)` where `N` Number of elements, `K` Number of buckets (or unique elements).
pub fn bucket_sort(target_array: &[usize]) -> Vec<usize> {
    if target_array.is_empty() {
        return vec![];
    }

    let mut result = vec![];

    let max = *target_array.iter().max().unwrap();
    let len = target_array.len();
    let mut buckets = vec![vec![]; len + 1];

    for &element in target_array {
        buckets[len * element / max].push(element);
    }

    for bucket in buckets.iter_mut() {
        super::insertion_sort(bucket);
        result.append(bucket);
    }

    result
}

pub fn bucket_sort_with_custom_nextsort(target_array: &[usize], next_sort: fn(array: &mut [usize])) -> Vec<usize> {
    if target_array.is_empty() {
        return vec![];
    }

    let mut result = vec![];

    let max = *target_array.iter().max().unwrap();
    let len = target_array.len();
    let mut buckets = vec![vec![]; len + 1];

    for &element in target_array {
        buckets[len * element / max].push(element);
    }

    for bucket in buckets.iter_mut() {
        next_sort(bucket);
        result.append(bucket);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::super::quick_sort;
    use super::*;

    #[test]
    fn empty() {
        let arr: [usize; 0] = [];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
        let res = bucket_sort_with_custom_nextsort(&arr, quick_sort);
        assert!(is_sorted(&res));
    }

    #[test]
    fn one_element() {
        let arr: [usize; 1] = [4];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
        let res = bucket_sort_with_custom_nextsort(&arr, quick_sort);
        assert!(is_sorted(&res));
    }

    #[test]
    fn basic() {
        let arr: [usize; 4] = [35, 53, 1, 0];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
        let res = bucket_sort_with_custom_nextsort(&arr, quick_sort);
        assert!(is_sorted(&res));
    }

    #[test]
    fn odd_number_of_elements() {
        let arr: Vec<usize> = vec![1, 21, 5, 11, 58];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
        let res = bucket_sort_with_custom_nextsort(&arr, quick_sort);
        assert!(is_sorted(&res));
    }

    #[test]
    fn repeated_elements() {
        let arr: Vec<usize> = vec![42, 9, 42, 0, 1, 1, 45, 42, 0, 45, 9, 1, 1];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
        let res = bucket_sort_with_custom_nextsort(&arr, quick_sort);
        assert!(is_sorted(&res));
    }
}
