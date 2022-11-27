fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]); // Provides a slice to `first_word`.
                                             // It's an immutable reference to a sequence of chars.
                                             // `['h', 'e', 'l', 'l', 'o', ' ']`
                                             // `[ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ]

    println!("{}", word);

    let word = first_word(&my_string[..]);   // Slice of ['h', .., 'd'];

    println!("{}", word);
    
    let word = first_word(&my_string);

    println!("{}", word);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);

    println!("{}", word);

    let word = first_word(&my_string_literal[..]);

    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
