#[allow(dead_code)]
enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

fn main() {
    let x: bool = None as Option<i32> == None;
    println!("{}", x);
}
