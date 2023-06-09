pub mod bucket_sort;
pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;

pub use self::bubble_sort::bubble_sort;
pub use self::bucket_sort::bucket_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::quick_sort::quick_sort;

use std::cmp::PartialOrd;

pub fn is_sorted<T: PartialOrd>(array: &[T]) -> bool {
    if array.is_empty() {
        return true;
    }

    let mut prev = &array[0];

    for item in array.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item;
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;

        assert!(is_sorted(&[] as &[isize]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1, 1]));

        assert!(!is_sorted(&[1, 0]));
        assert!(!is_sorted(&[2, 3, 1, -1, 5]));
    }
}
