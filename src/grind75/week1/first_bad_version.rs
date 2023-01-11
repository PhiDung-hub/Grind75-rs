// Problem: https://leetcode.com/problems/first-bad-version/
// Test in leetcode
pub struct Solution {}
impl Solution {
    pub fn is_bad_version(&self, version: i32) -> bool {
        version > 0
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        let mut correct_idx = -1;
        while low <= high {
            let search_idx = (high - low) / 2 + low;
            if self.is_bad_version(search_idx) {
                if !self.is_bad_version(search_idx - 1) {
                    return search_idx;
                }
                correct_idx = search_idx;
                high = search_idx - 1;
            } else {
                low = search_idx + 1;
            }
        }
        correct_idx
    }
}
