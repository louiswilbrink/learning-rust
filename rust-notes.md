# Rust



## Core Principles



- Rust is an *ahead-of-time compiled* language.
- Rust has *no classes*, only **types** and **traits**.
- Rust has no `NULL` value; use the `Option` type.
- Rust is a strong, **statically typed** language, meaning that it must know the types of *all variables* (and parameters) at compile time.
- Rust performs **type inferencing**; Rust will automatically infer a variable's type based on the type of the assigned value.
- Rust's **types** might seem similar to classes because you can have multiple **instances** of a type, and types have **methods**.
- Rust adheres to **functional scope**.
- Rust also has catch-all value  `_` similar to Scala.
- Rust will **panic** when a program exits with an error.
- Rust avoids **memory overflow** errors by using *two's complement wrapping*, but relying on this behavior is considered an error.
- Rust provides *type-specific* methods to check if overflow occurred.
- **Constants** are immutable *always*.
- **Constants** are upper-case & snake-case by convention.
- **Tuple** and **Arrays** have *fixed* length.
- Rust encourages you in favor of **immutibility** but gives you tools to opt out of that when necessary.
- **Variables** are immutable by default, and declared using the `let` keyword.
- Rust avoids mutation bugs by making variables immutable by default *or* explicitly conveying intent for mutation using the `mut` keyword.
- The `mut` keyword makes a variable mutable, but maintains the *same type*.
- the `mut` keyword aids the programmer by *conveying intent* to change the variable's value in other parts of the code.
- **Shadowing** lets you *temporarily* reuse the same variable name and assign a new value *and type*.
- **Shadowing** is achieved when you use the `let` keyword *again* ahead of a variable that was previously declared.
- **Shadowing** is often used  when you want to convert a value from one type to another type without coming up with a new variable name.
- **Shadowing** ends either when the variable itself is shadowed *again* or when the scope ends.
- The difference between `mut` and **shadowing** is that shadowing allows you to assign a *different type* to the variable.
- Calling a function with `!` appended on the end calls a Rust **macro** instead of a function.
- `use` statements bring a library or **trait** into the **scope** of the program.
- `&` prepended to an argument denotes that the arguement is a **reference** (aka **pass-by-reference**).
- Rust's major advantage is how safe and easy it is to use **references**.
- **References** are immutable by default.
- The double colons syntax (`::`) is used to call an **associated function** on a particular **type**.
- Primatives have an associated function `new` that creates instances of the primative.
- In Rust, `String` is a growable, UTF-8 encoded bit of text.
- In Rust, the right way to *suppress* warnings is to actually *write error handling.*
- Rust uses `{}` for String interpolation.
- A **crate** is a collection of Rust source code files.  **Binary crates** are executable while **library crates** are not executable on their own.
- `..=` syntax denotes an *inclusive* range.
- **Enum** is a **type** that could have one of a possible set of values, called **variants**.
- The `match` expression is a **control flow construct** that executes code based on the expression's resolved **type**.  This is called **pattern matching** in Scala.
- A `match` expression's possible patterns are called **arms**.
- The `loop` keyword continually executes a block of code.  Use this in conjuction with `continue` to restart the loop and `break` to exit the loop.
- Number literals can use `_` as a visual separator to make the number easier to read, such as `1_000`.
- **Scalar** data types represent a single value of one of it's primary scalar types: `integer`, `float`, `boolean`, and `char`.
- **Compound** data types can group multiple values into one type.  **Tuples** and **arrays** are compound data types.
- **Tuples** are a list of values of varying types.  Tuples are *fixed length*.
- Rust can use **destructuring** to assign or access **tuples**.
- **Destructured tuples** can be used in `match` **arms**.
- Access **tuple** elements directly using **dot notation**.
- A **tuple** without *any values* has a special name: the **unit** value (written out as empty parentheses `()`).
- The **unit** value is returned by an expression if no other value is returned.
- **Arrays** are a collection or list of values with the *same type* and has a *fixed length*.
- The Rust program will **panic** and say `"index out of bounds"` if you attempt to access an array element past the end of the arrays during runtime.
- Rust demonstrates its **memory safety principles** by checking for array access beyond the specified fixed length and *immediately exiting*.  Your code is never allowed to access an invalid memory location.
- A **Vector Type** is allowed to grow or shrink in size.
- `char` literals are specified with single quotes, `String` literals are specified with double quotes.
- **Functions** are snake-case by convention.
- Unlike JavaScript, Rust functions can be defined below where they're called (side effect of ahead-of-time compilation).
- In function signatures, you *must* declare the type of each parameter.  Rust *requires* **type annotations**.
- In Rust, variable assignments are **statements**, not expressions, and do *not* return a value.
- Function blocks may use the `return` keyword within the function body, but otherwise the *last expression* is implicitly considered to be the return value.
- In Rust, blocks of code evaluate to the last expression in them.
- **Expressions** do not include ending semicolons.  Adding a semicolon to the end of an expression turns it into a **statement**.
- If a function returns a value, the **return type** must be annotated in the **function signature** using  the arrow `->` syntax. 
- In Rust, `if` expressions do not need to be surrounded by parenthesis.
- Similar to `match` expressions, the blocks of code associated with the conditions of the `if` expression are sometimes called **arms**.
- In Rust, `if` expressions *must* evaluate to a `bool`.  Rust will not automatically try to convert non-Boolean types to a Boolean.
- Rust has no **ternary operator**, but `if` expressions can be used in assignments.
- All blocks in an `if` expression must evaluate to the *same type* otherwise a compile error is thrown.
- Rust has three types of loops: `loop`, `for` and `while`.
- `loop` will run a block of code continually until it reaches the `break` keyword.
- A `loop` block can *return a value* and be assigned to a **variable**.
- You may pass the result of a `loop`'s continuous operation by appending an **expression** to the `break` statement.
- If you have loops within loops, `break` and `continue` apply to the *innermost* loop at that point.
- You can optionally specify a **loop label** and reference them when using `break` or `continue`.
- **Loop labels** must begin with a single quote and look like `'counting_up: loop {`.
- In Rust, a `for` loop on an array works the same way as a `for..of` loop in JavaScript.  
- `for` loops on arrays are safe because they avoid going beyond (or fall short of) the final element in the collection.
- `for` loops are the *most commonly used* loop construct in Rust.
- In Rust, memory is allocated at compile time.  Any violation and Rust will not compile the program.
- Memory is structured in either **stacks** or **heaps**.
- **Stacks** are *last in, first out* and you *push* and *pop* data onto the stack.  
- All data in a stack *must* have a *known, fixed* size.  Otherwise, the data must be stored in a heap.
- **Heaps** allow for data of unknown size that may shrink or grow during runtime.
- When you allocate data on the heap, a **pointer** is returned marking its address or location in memory.
- A **pointer** can be stored in a stack since it will have a known, fixed size.
- Pushing data onto a **stack** is *faster* than allocating on the **heap** because the memory allocator doesn't need to search for a place to store new data.
- Accessing data on the **stack** is *faster* because you don't have to follow a **pointer** to get there.
- When a function is called, function arguments and local variables get pushed onto a stack, and popped off after execution.
- One of the main purposes of **Ownership** in Rust is to manage *heap data*. 
- **Ownership Rules**
  - Each value has an *owner*.
  - There can only be *one owner* at a time.
  - When the owner goes out of scope, the value is *dropped*.
- Rust does *not* have a garbage collector.
- Strings are *not immutable* and are allocated to the **heap**.
- In Rust, memory is automatically returned once the variable that owns it *goes out of scope*.
- Assigning one variable of *vector* data to another variable *invalidates* the first variable, and compilation will fail if you attempt to reference it again.
- Assigning a variable of *vector* data to another variable effectively *moves it*.
- Assigning one variable of *scalar* data to another variable works fine.  Both variables are valid and point to *separate data* on the stack.
- In Rust, there are no **shallow copies** of vector data; it's more of a **move**.
- Rust will never automatically create a **deep copy** of your data through assignment.
- Rust provides deep copy functionality through the explicit `clone` method, but note that it is always *expensive*.
- Types with the `Copy` **trait** are able to be *trivially* copied from one variable to another, and *both* variables remain valid.
- When a function is called, the *vector* variables used as arguments *move into that function* and are *no longer valid* in the calling function.  Use **references** in order to avoid this behavior.
- Return values transfer ownership back to the parent (or calling) function.
- Rust has a feature for using a value *without* transferring ownership, called **references**.
- **Tuples** and **arrays** are saved on the stack.
- Functional programming builds a functional body; your body can perform hundreds of acts.  Object-oriented programming builds a scene with actors.
- In Rust, function calls effectively assigns local variables to arguments and *moves them* to the called function.
- Rust uses **references** to pass around values *without* transferring ownership.
- Use **references** to avoid transferring ownership of values to functions.
- **References** have the `usize` type, an integer value that represents a memory location.  It's a pointer value.
- The opposite of referencing is **dereferencing** and is accomplished by using the `*` operator.
- The `&` operator lets us create a reference that *refers* to a value but *does* *not* own it.
- Because ownership *isn't* transferred, the value that the reference points to *is not* dropped after functional scope is concluded.
- The action of creating a reference is called **borrowing**.  You can borrow a value, but you don't *own* it.
- Through **references**, Rust allows you to borrow *values*.
- Like variables, **references** are immutable by default.
- You *cannot* mutate a borrowed value unless you add the `mut` keyword in the function signature.
- **Mutable references** can only be referenced *once*.
- Rust only allows **mutation** in a very controlled fashion, which avoids **data races**.
- At any given time, you can have *either* one mutable reference *or* any number of *immutable* references.
- In Rust, you'll always know what will *never* change; and what *can* change is going to be atomic and with minimal or zero side effect.
- A **reference's scope** starts when it is introduced up until it is *last used*.
- You can create a mutable reference to a value *after* the *immutable* reference is *last used*.
- The compiler can tell when a reference is no longer being used (even  before the end of **functional scope**) using **non-lexical lifetimes**.
- On MQ B2B, the *hardest* bug to resolve was caused by mutating data and *not knowing when or where* it was occurring.
- The Rust compiler will not let you create **dangling pointers**.  It will recognize & complain if a value can drop while the reference to it persists.
- A **slice** is a reference to a contiguous sequence of elements in a collection.  It does *not* have ownership; it is a reference.
- Slices are of type `&str`.
- Rust uses **ownership** rules, enforced at compile time, to eliminate an entire class of memory mismanagement errors.
- In Rust, assessing the value of a reference does *not* require some access method (like in Scala).
- **String literals** are actually slices of a String type.
- By using **slices**, we avoid transferring ownership but keep values allocated for the duration of **reference scope**.
- The concepts of **ownership**, **borrowing**, and **slices** ensure memory safety in Rust programs at compile time.
- The biggest source of tech debt comes from the cavalier decision to mutate data.
- In Rust, there are no `constructor` methods, since there are no classes!
- **Structs** are a collection of type definitions and defined similarly to how types and interfaces are defined in TypeScript.
- Rust allows **field init shorthand** like in JavaScript when assigning fields to variables with identical names.
-  Rust also has a **spread operator** used to combine or update structs.
- When using the spread operator, or **struct update operator**, any fields containing vector data forces the assignment to *move* to the new variable, invalidating the source variable.  Only when all update fields implement the `Copy` trait would both variables remain valid.
- It is possible for a **struct** to store *references* to data owned by *something else*, which requires the use of **lifetimes**.
- **Tuple structs** are lists with defined types for each element.
- **Unit-like structs** are structs without any fields.
- Rust doesn't automatically print tuples or structs to the console.  To do so, add the **debug attribute** to the variable type definition, which adds the `Debug` trait.
- The invocation context of an instance is denoted by `self` in Rust, instead of `this` in JavaScript.
- Methods for types are summarized within a separated **implementation block**.
- Rust allows **getters**, but they are not automatically implemented.
- Struct fields can be private, and getters can provide a public API to read them.
- Rust has a feature called **automatic referencing and dereferencing** that makes calling references to methods a lot cleaner.
- In Rust, **type definitions** keep fields and methods separate.
- **enum definitions** can include an associated value for each variant.
- **Enum** types can have implementations/methods/associated functions.
- **Enum** types can have an `Option` variant to allow the value to be something or nothing.
- Rust does *not* have the `Null` value.  Instead it has the **enum** type where the value may or may not be present.
- You must convert an `Option<T>` to a defined type before you can perform type operations with it.  This avoids all errors related to assuming something isn't `Null` when it turns out it is.
- If *any* variable might possibly be `Null`, it *must* be an `Option<T>` type.  This forces you to explicitly handle the case where the value is `Null`.  This means all types can be assumed to *not be* `Null`.
- The `Option<T>` type has methods to check for `None`, or to convert the value into a specific result.
- Rust has multiple variations of the `getOrElse()` method in Scala.
- To get the value of a `Option<T>` type, use the `unwrap()` method.
- Use the method `unwrap_or(some_value)` to evaluate to a default value if the `Option<T>` is `None`.
- In a `match` expression, the Rust compiler confirms that *all possible cases* are handled.
- The `if let` syntactic sugar is useful when you only want to match one pattern and ignore all of the rest.
- Rust's modules system is composed of **paths**, **modules**, **crates**, and **packages**.
- A package may contain multiple **binary crates** but only *one* **library crate**.
- When declaring modules, the compiler will automatically look for the corresponding file with the same name aka `/src/module_name.rs`.
- Code within a `module` is private by default.  Use the `pub` keyword to make code public.
- Rust has everything good: statically typed language, simple memory management rules, built-in module system, built-in version management, compiles into agnostic binaries, pattern matching, and rigid mutability rules.  Including Rust in Linux kernels provides a strong industry signal.
- In Rust, all items in a child module are *private* to the parent module by default.  However, child modules can use items defined in the parent module.
- Making a module public using the `pub` keyword does *not* make its contents public.
- Starting relative paths using the `super` keyword is shorthand for referencing the parent module (or `..` syntax in Unix).
- Use the `super` keyword when referencing parent module items is often needed in the child module. 
- Making a **struct** public using `pub` *doesn't* make all of it's field public.  However, making an `enum` public *does* make all it's variants public.
- Rust's vector data structures are colloquially called **collections**, and are stored on the **heap**.
- The **vector** data structure is a list that you can grow and shrink using `push` & `pop` methods.  They contain data of the *same type* and are stored on the **heap**.
- In Rust, you can only concat a `String` with a slice (not another `String`).
- In Rust, the `String` type does not support indexing to access a specific character location.  Since `String` types are stored on the **heap**, accessing an element would mean iterating over the `String` and would *not* be performant.
- In Rust, the `String` type is a list of **bytes** stored as a vector of `u8` values or Unicode scalar values, or letters.  At compile time, we *don't know* which encoding is used.
- Use the `String` method `.chars()` to separate each character in a `String` that you can use to iterated over.
- In Rust, **Hashmap** keys have a specified type.
- If you insert a variable into a **Hashmap**, ownership is transferred to the hashmap and the variables will no longer be valid.
- In many cases, Rust requires you to acknowledge the *possibility* of an error and take some action before your code will compile.
- Rust groups errors into two main categories: **recoverable** and **unrecoverable** errors.
- Rust doesn't have **exceptions** (which will always terminate execution).
- **Recoverable errors** are a special `Result` type which requires handling.
- **Unrecoverable errors** causes Rust to "panic" and stop execution.
- When a **panic** occurs, the Rust program will "unwind", meaning Rust walks back up the stack and cleans up data from each scope.
- In some languages, reading beyond a data structure may result in accessing a different location in memory.  This is called a **buffer overread**, and Rust will panic and terminate execution before allowing this.
- Similar to writing `catch` error notifications, Rustaceans will use the `.expect()` method to clarify what went wrong.
- All actions that *might* fail, such as opening a file or requesting data from a server, should return a `Result<T,E>` enum in order to handle errors.
- In Rust, errors should be propagated from low-level functions to calling functions.
- The `?` operator evaluates the result of an operation and will return an error to the calling function on failure; otherwise it will resolve to the value inside `Ok` variant.
- The `?` operator provides **optional chaining**.
- The `?` operator can *only* be used in functions with `Result<T,E>` return type.
- Rust makes potential values *knowable*: a function will either have a successful or failed `Result` *or* an **optional value** of `Some` or `None`.
- Types can have **validations** that checks input values for correctness when invoking it's `new` method.
- **Validations** provide confidence to functions that the values in a type conform to certain constraints & expectations.
- **Generic types** are used to allow functions to accept different parameter types, making them more flexible.
- Using **generics** in function signatures, **structs**, or **enums** requires annotating their *names* with `<T>`.
- Even **structs** and **enums** can have methods; they're defined in the same implentation closure.
- In Rust, available methods in a generic type *might only be defined* for specific types.
- A **types** behavior consists of its **methods**.
- **Traits** are methods that can be shared among **types**.
- **Traits** define shared behavior among **types**.
- Defining **trait bounds** in a function means that it can accept any generic type as long as it has *certain behavior*.
- **Traits** are analogous to **interfaces** in other languages.
- **Traits** declarations *can* include a method body in order to define default behavior.
- A **type** may override the default behavior of a **trait**.
- **Traits** allow you to compose behavior in **types**, saving a lot of duplication and encouraging code reuse and organization.
- Function parameters may use a *reference to a **trait*** instead of a concrete **type**.  That function parameter gains all trait methods.  The function can be called using any **type** that implements the **trait**.
- Functions that specify a parameters type as a reference to a **trait** is called the `impl Trait` syntax.
- The **trait bound form** is the verbose syntax for assigning a parameter a reference to a trait.
- **Trait bound** information in a function signature defines what behaviors are required for any type that might potentially be used when calling the function.
- You can use the `+` operator to implement multiple traits on a single function parameter.
- Functions can utilize the `where` clause in their signature to specify **trait bounds** in a readable fashion when dealing with multiple generic types.
- The `impl Trait` syntax can also be a function's return type.
- Using **traits** as function parameter and return types provides flexibility and encourages code reusability because you can call the function using *any type* that implements those traits.
- You can only use `impl Trait` syntax if you're returning a *single* type.
- In function signatures, parameters can have concrete types or **generic** types, or somewhere in between with **traits**, which requires that the generic type needs to have a particular behavior.
- **Lifetimes** are a **generic**.
- **Lifetimes** ensure that references are valid for as long as we need them to be.
- Every **reference** in Rust has a **lifetime**, which is mostly implicit or inferred.
- You can prevent **dangling references** with lifetimes.
- Rust uses a **borrow checker** to ensure at compile time that all reference are valid for as long as we're using them.
- Rust's **borrow checker** compares scopes to determine when borrows are valid.
- Sometimes the **borrow checker** cannot determine intrinsically the lifetime of a reference; Adding a **lifetime annotation** provides enough information to the borrow checker to perform its analysis.
- A **lifetime annotation**, or **lifetime specifier**, describes the relationships of lifetimes of multiple referenced parameters *without effecting lifetimes*.
- The names of **lifetime parameters** start with an apostrophe, are all lowercase, and usually very short, just like generic types.  They are added between the `&` and parameter type.
- When specifying **lifetimes**, the function name also needs annotation, similar to how generic types are declared.
- **Lifetime annotations** help describe the relationship of lifetimes in a function, allowing the **borrow checker** to know definitively that a reference will remain valid during run time.
- **Lifetime annotations** relate reference parameter lifetimes to one another, so that assumptions are not needed by the borrow checker.
- Having functions contain the lifetime contract helps the Rust compiler recognize and enforce the constraints with less inference or assumptions.
- Many of the Rust-only rules (ownership, lifetimes) are aimed at corralling problems into very narrow sites in the code, commonly in function signatures.
- **Lifetime annotations** can tell the compiler things like, "Hey the lifetime of the return reference will be at least as long as the smaller of the two *reference parameter* lifetimes."  Knowing this, the **borrow checker** can tell if a borrowed value does not live long enough.
- When a function has references as the input parameters *and* return type, then **lifetimes** ensure that valid data makes it into *and out of* the function without downstream side effects.
- **Lifetime syntax** is all about connecting the lifetimes of parameters and return values of a function.
- You can have **lifetime annotations** in **structs**.
- Using **lifetime annotations** in **structs** tells the compiler that the instance can't outlive a reference held in one of it's fields.
- The **borrow checker** didn't *always* infer reference lifetimes; it required lifetime annotation on *every reference*.  But later versions of Rust added common-sense inferencing (called **lifetime ellision rules**), so only truly ambiguous cases require *explicit* lifetime annotations. 
- In the background, the compiler will add lifetime annotations according to the **3 lifetime elision rules**.  If after applying the rules the compiler still can't figure out what the return type's lifetime is, the programmer will need to explicitly add lifetime annotations.
- Lifetimes on parameters are called **input lifetimes** and lifetimes on return values are called **output lifetimes**.
- A reference that can live for the *entire duration* of the programs has a **static lifetime**.
- **Generics**, **trait bounds**, and **lifetimes** all work together to make function calls safe and flexible.
- In Rust, you can add attributes and metadata to structs, modules, and functions.
- Rust's built-in testing offers benchmarking, parallel or single-threaded execution, ability to ignore or filter out tests or run a specific test, and has a variety of assertion macros and test attributes to grade functionality.
- Test functions can specify return types as a means of checking expected values; this is particularly useful when testing functions that return the `Result<T,E>` type.
- 



## Light Truths



- `rustup` updates & manages Rust installations.
- `rustc` is the compiler and used to run all Rust files (`.rs`).
- In Rust project stucture, the top-level directory should only contain `README` files, license files, configuration files, and anything else not related to your code.
- All Rust tools are found in `~/.cargo/bin`.
- To execute code, first **compile** the `.rs` file, then execute the resulting binary (ie `./main`).  Compiling and running a program are *separate* steps in Rust.
- `.toml` stands for "Tom’s Obvious, Minimal Language", lol.
- `std::io::StdIn` is a **type** that represents a handle to your terminal's standard input (aka the command line).
- The `Result` type is an **enum**, which is a way of saying a value is one of a possible set of values (called **variants**).
- The `Result` type encodes error-handling information.
- `rustc --explain E0308` pulls up documentation on the specific error.



## Types



### Result



The [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) type is an **enum** used to return and propagate errors.



It contains two **variants**: `OK(T)` and `Err(E)`.



Functions return `Result` whenever errors are expected and recoverable.



`Result` has an `expect("Failed, me Lord.")` method which will terminate execution and display the provided message.



## Cargo



Rust's build system and package manager.



| Command                    | Utility                                                      |
| -------------------------- | ------------------------------------------------------------ |
| `$ cargo new project_name` | Creates a new Rust project with proper directory structure, package management, and source files. |
| `$ cargo build`            | Builds a Rust project by installing dependencies found in `Cargo.toml` and compiling all source code (`/src`) into a `/target` directory. |
| `$ cargo run`              | Executes fully built binaries.                               |
| `$ cargo check`            | Ensures your code is able to compile, but doesn't produce an executable. |
| `$ cargo build --release`  | Creates a release build.                                     |
| `$ cargo update`           | Update all packages/crates.                                  |
| `$ cargo doc --open`       | Builds documenation provided by all of your dependences and opens it in a browser. |



## Installation



```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Select option 1
$ source "$HOME/.cargo/env"
$ rustc --version
rustc 1.65.0 (897e37553 2022-11-02)
$ cargo --version
cargo 1.65.0 (4bc8f24d3 2022-10-20)
```



Uninstall:



```bash
$ rustup self uninstall
```





## Resources



Learning primarily through Rust's documentation command:



```bash
$ rustup doc
# Opens file:///Users/louiswilbrink/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/book/index.html
```





