macro_rules! say {
    ($a: expr) => {
        println!("Hello world!");
        println!("{}", $a);
    };
}

fn main() {
    say!("this");
}
