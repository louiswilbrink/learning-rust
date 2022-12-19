// Given a string s, find the length of the longest substring without repeating characters.

// __Requirements__
// My input s is a String.
// My output will be an integer representing the length of a substring.
// The substring CANNOT have repeating characters.

// __Constraints__
// (No constraints that are not already summarized by requirements)

// __Assumptions__
// Can I assume that s will always be a string or should I include type checking in my routine?  String s can safely be assumed to be of type String and no type checking is necessary
// Can I assume that String s will be a string of more than 0 characters?  No, check for an empty string (and return '');
// Can I assume that the string will be alphanumeric?  alphabet-only?  or can I make neither assumption and ensure my code can run on any set of characters with ASCII-values? It will be alpha-numberic only.

// __Clarifying questions__
// If there are two substrings of the same length, can either be returned?  Both? Either is fine.
// In the case where String s is a list of the *same* characters, what should be the return value? A single character.

// __Modeling__
// 'catdd' => 'catd'
// 'drinkkcompa' => 'kcompa'
// 'ddd' => 'd'
// '3' => '3'

fn main() {
    let s = String::from("catdd");

    let mut longest_length = find_longest_length(&String::from("catdd"));
    println!("Longest length: {}", longest_length);

    let longest_length = find_longest_length(&String::from("drinkkcompa"));
    println!("Longest length: {}", longest_length);

    let longest_length = find_longest_length(&String::from("ddd"));
    println!("Longest length: {}", longest_length);

    let longest_length = find_longest_length(&String::from("3"));
    println!("Longest length: {}", longest_length);
}


fn find_longest_length(mut s: &str) -> u32 {
    if let 0 | 1 = s.len() {
        return s.len().try_into().unwrap()
    }

    let mut substrings: Vec<String> = Vec::new();

    let mut unique_letters: Vec<char> = Vec::new();

    let mut last_letter: char = '*';

    // Loop through each character in the string.
    for (i, letter) in s.chars().enumerate() {
        // If current letter is not a repeat, add it to a substring (vec) 
        // and update last_letter.
        if last_letter != letter {
            unique_letters.push(letter);
            last_letter = letter;
        } else {
            // ..otherwise, save unique_letters as a String for `substrings`.
            substrings.push(unique_letters.into_iter().collect());

            // ..and start a new unique_letters, starting with the repeated character.
            unique_letters = Vec::new();
            unique_letters.push(letter);
        }
    }
    
    // Push any remaining unique_letters into the substrings collection.
    substrings.push(unique_letters.into_iter().collect());

    let mut longest = substrings[0].clone();

    for substring in substrings {
        if substring.len() > longest.len() {
            longest = substring.clone();
        }
    }

    println!("Longest substring: {}", longest);

    longest.len().try_into().unwrap()
}

fn build_non_repeating_string(s: &str) -> (String, &str) {
    let mut unique_letters: Vec<char> = Vec::new();

    // Iterate over the string, bank characters that aren't repeats.
    for letter in s.chars() {
        unique_letters.push(letter);
    }

    let unique: String = unique_letters.into_iter().collect();

    println!("Unique: {}", unique);

    (String::from("louis"), &s[2..3])
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
