/******************************************************************************

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.

__QUESTIONS__
No space constraints


__MODELING__

(            -> If stack is empty, add to stack: [ '(' ]
)            -> Check! Compare next character with top of stack character.
                Is the next character..
                  ..a _matching_ & _closed_ character? .pop() and continue <--
                  ..a _different_ & _open_ character? Add to stack.
                  ..a _different_ & _closed_ character? !Invalid!
                [ ]
[            -> Empty stack, add to stack: [ '[' ]
]            -> Top-of-stack character matches with this new character, .pop() and continue
{            -> Add to stack.
}            -> .pop() and continue.

******************************************************************************/

fn main() {
    println!("Hello, world!");
}
