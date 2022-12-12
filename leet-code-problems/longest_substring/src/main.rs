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

    let longest_length = find_longest_length(&s);

    println!("Longest length: {}", longest_length);
}

fn find_longest_length(s: &str) -> u32 {
    let mut unique_letters: Vec<char> = Vec::new();

    // Iterate over the string, bank characters that aren't repeats.
    for letter in s.chars() {

        match unique_letters.last() {
            Some(last) => {
                // If next letter is NOT the same as the last letter in our vec, push it in.
                if letter != *last {
                    unique_letters.push(letter);
                }
            },
            // Empty vec?  Start by pushing the first letter of the string.
            None => unique_letters.push(letter)
        };
    }

    println!("Unique letters: {:?}", unique_letters);

    unique_letters.len().try_into().unwrap()
}
