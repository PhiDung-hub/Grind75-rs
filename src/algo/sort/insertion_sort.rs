use std::cmp::PartialOrd;

pub fn insertion_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    for i in 1..array.len() {
        let current_element = array[i];
        let mut j = i - 1;

        // NOTE: this loop perform batch swapping in an efficient manner

        // loop to swap element until the order is satisfied.
        while array[j] > current_element {
            array[j + 1] = array[j];
            if j == 0 {
                break;
            }
            j -= 1;
        }

        // perform the last step of swapping
        if j==0 && array[0] > current_element {
            array[0] = current_element;
        } else {
            array[j+1] = current_element;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
