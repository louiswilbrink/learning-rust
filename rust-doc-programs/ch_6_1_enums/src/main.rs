#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {

    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    let some = Some(5);

    println!("{:?}", some);

    let absent_number: Option<i32> = None;

    println!("{:?}", absent_number);

    let absent_string: Option<String> = None;

    println!("{:?}", absent_string);

    m.call();

    println!("{:?}", four);
    println!("{:?}", six);

}
