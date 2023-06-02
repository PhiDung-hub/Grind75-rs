pub fn bubble_sort<T: Ord>(array: &mut [T]) {
    if array.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = array.len();

    // Replace outer for loop with an if statement for computation efficiency.
    while !sorted {
        sorted = true;

        for i in 0..n - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }

        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }
    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }
}
