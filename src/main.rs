use algo_practise::find_max;

pub fn main() {
    let x = find_max!(1, 5, 2);
    assert_eq!(x, 5);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {}

    #[test]
    fn example_2() {}

    #[test]
    fn example_3() {}
}

