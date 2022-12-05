use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn main_one() {
    let x = 5;

    let r = &x;

    println!("r: {}", r);
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let announcement = "Today is MONDAY!";

    let result = longest_with_an_announcement(string1.as_str(), string2, announcement);

    println!("The longest string is {}", result);
}

fn main_excerpt() {
    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Accouncement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
