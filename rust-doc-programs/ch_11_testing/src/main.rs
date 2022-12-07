fn main() {
    println!("Hello, world!");
    add(1,1);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    println!("Yay");
}
