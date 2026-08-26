Next, we’ll create a _variable_ to store the user input, like this:

```
    let mut guess = String::new();
```

Now the program is getting interesting! There’s a lot going on in this little line. We use the `let` statement to create the variable. Here’s another example:

```
let apples = 5;
```

This line creates a new variable named `apples` and binds it to the value `5`. In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change. We’ll be discussing this concept in detail in the [“Variables and Mutability”](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#variables-and-mutability) section in Chapter 3. To make a variable mutable, we add `mut` before the variable name:

**Note:** A really vierd feature. Not sure why its designed like this. Needs to be resarched. 
```
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

> [!note]
> The `//` syntax starts a comment that continues until the end of the line. Rust ignores everything in comments. We’ll discuss comments in more detail in [Chapter 3](https://doc.rust-lang.org/stable/book/ch03-04-comments.html).

Returning to the guessing game program, you now know that `let mut guess` will introduce a mutable variable named `guess`. The equal sign (`=`) tells Rust we want to bind something to the variable now. On the right of the equal sign is the value that `guess` is bound to, which is the result of calling `String::new`, a function that returns a new instance of a `String`. [`String`](https://doc.rust-lang.org/stable/std/string/struct.String.html) is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. An _associated function_ is a function that’s implemented on a type, in this case `String`. This `new` function creates a new, empty string. You’ll find a `new` function on many types because it’s a common name for a function that makes a new value of some kind.

In full, the `let mut guess = String::new();` line has created a mutable variable that is currently bound to a new, empty instance of a `String`. Whew!

## Related files 
- guessing_game_04

---

> [!tip] Related Notes
> - [[022_Processing a Guess|022 Processing a Guess]] — where these variables are used
> - [[031_ Variables and Mutability|031 Variables and Mutability]] — variables and `mut` in depth
> - [[026_Printing Values with println! Placeholders|026 Printing Values with println! Placeholders]] — printing the stored values

---

← [[022_Processing a Guess|022 Processing a Guess]] · [[Rust Notes|⌂ Rust Notes]] · [[024_Receiving User Input|024 Receiving User Input]] →
