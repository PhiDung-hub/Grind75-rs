#[allow(unused_imports)]
use std::collections::*;
use std::io;
// use std::io::prelude::*;
// use std::fs::File;

pub fn main() -> io::Result<()> {
    let mut s = "xzssxz".to_string();
    s +=  " helloo";
    println!("{}", -25/10);


    Ok(())
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
