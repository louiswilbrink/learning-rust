// constant declaration.
const ENVIRONMENT: i32 = 1;

// Provide ability to print/display type to console.
#[derive(Debug)]
// Struct
struct User {
    username: String,
    email: String,
    age: i32,
    is_employed: bool
}

// Trait with no default behavior defined.
pub trait Summary {
    fn summarize(&self) -> String;
}

// A simple type.
pub struct Tweet {
    pub username: String,
    pub content: String,
}

// Type overriding trait method.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Fn parameter using a reference to a trait implementation instead of a concrete type.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Example of trait bound fn parameter.
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Struct method declaration.
impl User {
    // Getter
    fn age(&self) -> i32 {
        self.age
    }

    // Method with `self` reference.
    fn is_old(&self) -> bool {
        self.age > 25
    }

    fn username(&self) -> &str {
        &self.username
    }
}

fn main() {
    // variable declaration.
    let x: i32 = 5;

    // mutable variable declaration.
    let mut user_name: String = String::from("Louis");

    // Shadowing.
    let x: bool = true;

    // Tuple.
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);

    // Tulple destructuring assignment.
    let (a,b,c) = my_tuple;

    // Array.
    let array: [i32; 5] = [1,2,3,4,5];

    // Accessing arrays.
    let first_element = array[0];
    let last_element = array[4];

    // Enum declaration.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter
    }

    let coin = Coin::Dime;

    // match expression.
    let cents: i32 = match coin {
        Penny => 1,
        Nickel => 5,
        Dime => 10,
        Quarter => 25
    };

    println!("The coin is worth {} cents.", cents);
    
    // Vector declaration.

    // 'new' type method.
    // (Also an associated function of a type).
    let new_string = String::new();

    // Borrowing function call.
    // Aka passing a reference or slice.
    // Aka non-ownership transfer.
    borrowing_fn(&user_name);

    // Struct assignment.
    let user = User { 
        username: String::from("louiswilbrink"),
        email: String::from("lw@lw.com"),
        age: 38,
        is_employed: false
    };

    // Struct using spread operator.
    let user_two = User {
        username: String::from("louissecondaccount"),
        email: String::from("lw2@lw.com"),
        ..user
    };

    println!("The second user is {:?}", user_two);
}

fn borrowing_fn(s: &str) -> &str {
    &s
}
