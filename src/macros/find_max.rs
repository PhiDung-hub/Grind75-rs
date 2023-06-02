#[macro_export]
macro_rules! find_max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::max($x, find_max!($($y),+))
    );
}
