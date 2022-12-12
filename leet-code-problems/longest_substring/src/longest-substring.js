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

function getLongestNonRepeatingString (s) {
  if (!s.length) return '';
  if (s.length == 1) return s;

  const characters = [];
  let longest = [''];

  for (let i = 0; i < s.length; i++) {
    characters.push(s[i]);
  }

  // Iterate through each character in the string
  // If the first character, then that's the longest string.
  // If a second or later character, compare it with the previou character.
    // If it's different, add that character to the current substring.
    // If a double, conclude the substring and start a new one with the current character.

  for (let i = 0; i < characters.length; i++) {
    if (i == 0) {
      longest[longest.length - 1] = characters[i];
    } else {
      if (characters[i] == characters[i - 1]) { // Next character is a repeat!
        longest.push(characters[i]);
      } else {
        longest[longest.length - 1] += characters[i];
      }
    }
  };

  let longestLength = 0;
  let longestString = '';

  for (let i = 0; i < longest.length; i++) {
    if (longest[i].length > longestLength) {
      longestLength = longest[i].length;
      longestString = longest[i];
    }
  }

  return longestString;
}

console.log(getLongestNonRepeatingString(''));
console.log(getLongestNonRepeatingString('3'));
console.log(getLongestNonRepeatingString('catdd'));
console.log(getLongestNonRepeatingString('catxxdogsxxgoldfish'));
