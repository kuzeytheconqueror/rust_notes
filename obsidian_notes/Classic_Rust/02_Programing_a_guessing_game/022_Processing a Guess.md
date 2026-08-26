At the first part, we will be ask user fopr input. and then process the input. there are some similarities with C++. Interesting code compare to JAVA. 

```
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

So this code has a lot of info, we need to iterate with it

This code contains a lot of information, so let’s go over it line by line. To obtain user input and then print the result as output, we need to bring the `io` input/output library into scope. The `io` library comes from the standard library, known as `std`:

```
use std::io
``` 
- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the _prelude_, and you can see everything in it [in the standard library documentation](https://doc.rust-lang.org/stable/std/prelude/index.html).


If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a `use` statement. Using the `std::io` library provides you with a number of useful features, including the ability to accept user input.

As you saw in Chapter 1, the `main` function is the entry point into the program:

```
fn main() {
```

The `fn` syntax declares a new function; the parentheses, `()`, indicate there are no parameters; and the curly bracket, `{`, starts the body of the function.

As you also learned in Chapter 1, `println!` is a macro that prints a string to the screen:

```
    println!("Guess the number!");

    println!("Please input your guess.");
```

## Related files 
- guessing_game_04

---

> [!tip] Related Notes
> - [[021_ Setting up a New Project|021 Setting up a New Project]] — the project this code lives in
> - [[023_Storing Values with Variables|023 Storing Values with Variables]] — the `let` binding introduced here
> - [[024_Receiving User Input|024 Receiving User Input]] — the `stdin` part of this code
> - [[025_Handling Potential Failure with Result|025 Handling Potential Failure with Result]] — the `Result` returned by `read_line`

---

← [[021_ Setting up a New Project|021 Setting up a New Project]] · [[Rust Notes|⌂ Rust Notes]] · [[023_Storing Values with Variables|023 Storing Values with Variables]] →
