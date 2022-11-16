#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Colorado
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let coin = Coin::Quarter(UsState::Colorado);

    println!("{:?}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);

    println!("{}", six.unwrap());
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
