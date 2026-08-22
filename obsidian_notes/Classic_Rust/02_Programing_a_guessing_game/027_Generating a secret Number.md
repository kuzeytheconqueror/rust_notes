Next, we need to generate a secret number that the user will try to guess. The secret number should be different every time so that the game is fun to play more than once. We’ll use a random number between 1 and 100 so that the game isn’t too difficult. Rust doesn’t yet include random number functionality in its standard library. However, the Rust team does provide a [`rand` crate](https://crates.io/crates/rand) with said functionality.

## Increasing Functionality with Crate

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The `rand` crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.

Cargo’s coordination of external crates is where Cargo really shines. Before we can write code that uses `rand`, we need to modify the _Cargo.toml_ file to include the `rand` crate as a dependency. Open that file now and add the following line to the bottom, beneath the `[dependencies]` section header that Cargo created for you. Be sure to specify `rand` exactly as we have here, with this version number, or the code examples in this tutorial may not work:

Filename: Cargo.toml
```
[dependencies]
rand = "0.8.5"
```

In the _Cargo.toml_ file, everything that follows a header is part of that section that continues until another section starts. In `[dependencies]`, you tell Cargo which external crates your project depends on and which versions of those crates you require. In this case, we specify the `rand` crate with the semantic version specifier `0.8.5`. Cargo understands [Semantic Versioning](http://semver.org) (sometimes called _SemVer_), which is a standard for writing version numbers. The specifier `0.8.5` is actually shorthand for `^0.8.5`, which means any version that is at least 0.8.5 but below 0.9.0.

Cargo considers these versions to have public APIs compatible with version 0.8.5, and this specification ensures that you’ll get the latest patch release that will still compile with the code in this chapter. Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use.

Now, without changing any of the code, let’s build the project, as shown in Listing 2-2.
```
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

[Listing 2-2](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#listing-2-2): The output from running `cargo build` after adding the `rand` crate as a dependency

You may see different version numbers (but they will all be compatible with the code, thanks to SemVer!) and different lines (depending on the operating system), and the lines may be in a different order.

When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the _registry_, which is a copy of data from [Crates.io](https://crates.io/). Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

After updating the registry, Cargo checks the `[dependencies]` section and downloads any crates listed that aren’t already downloaded. In this case, although we only listed `rand` as a dependency, Cargo also grabbed other crates that `rand` depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

If you immediately run `cargo build` again without making any changes, you won’t get any output aside from the `Finished` line. Cargo knows it has already downloaded and compiled the dependencies, and you haven’t changed anything about them in your _Cargo.toml_ file. Cargo also knows that you haven’t changed anything about your code, so it doesn’t recompile that either. With nothing to do, it simply exits.

If you open the _src/main.rs_ file, make a trivial change, and then save it and build again, you’ll only see two lines of output:

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

These lines show that Cargo only updates the build with your tiny change to the _src/main.rs_ file. Your dependencies haven’t changed, so Cargo knows it can reuse what it has already downloaded and compiled for those.

## Ensuring Reproducible Builds

Cargo has a mechanism that ensures that you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the `rand` crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the _Cargo.lock_ file the first time you run `cargo build`, so we now have this in the _guessing_game_ directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the _Cargo.lock_ file. When you build your project in the future, Cargo will see that the _Cargo.lock_ file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the _Cargo.lock_ file. Because the _Cargo.lock_ file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

## Updating a Crate to get a new Version

When you _do_ want to update a crate, Cargo provides the command `update`, which will ignore the _Cargo.lock_ file and figure out all the latest versions that fit your specifications in _Cargo.toml_. Cargo will then write those versions to the _Cargo.lock_ file. Otherwise, by default, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0. If the `rand` crate has released the two new versions 0.8.6 and 0.999.0, you would see the following if you ran `cargo update`:

```
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```

Cargo ignores the 0.999.0 release. At this point, you would also notice a change in your _Cargo.lock_ file noting that the version of the `rand` crate you are now using is 0.8.6. To use `rand` version 0.999.0 or any version in the 0.999._x_ series, you’d have to update the _Cargo.toml_ file to look like this instead (don’t actually make this change because the following examples assume you’re using `rand` 0.8):

```
[dependencies]
rand = "0.999.0"
```

The next time you run `cargo build`, Cargo will update the registry of crates available and reevaluate your `rand` requirements according to the new version you have specified.

There’s a lot more to say about [Cargo](https://doc.rust-lang.org/cargo/) and [its ecosystem](https://doc.rust-lang.org/cargo/reference/publishing.html), which we’ll discuss in Chapter 14, but for now, that’s all you need to know. Cargo makes it very easy to reuse libraries, so Rustaceans are able to write smaller projects that are assembled from a number of packages.

## Generating a Random Variable

Let’s start using `rand` to generate a number to guess. The next step is to update _src/main.rs_, as shown in Listing 2-3.

Filename: src/main.rs

```
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```
[Listing 2-3](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#listing-2-3): Adding code to generate a random number

First, we add the line `use rand::Rng;`. The `Rng` trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. Chapter 10 will cover traits in detail.

Next, we’re adding two lines in the middle. In the first line, we call the `rand::thread_rng` function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system. Then, we call the `gen_range` method on the random number generator. This method is defined by the `Rng` trait that we brought into scope with the `use rand::Rng;` statement. The `gen_range` method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form `start..=end` and is inclusive on the lower and upper bounds, so we need to specify `1..=100` to request a number between 1 and 100.

>[!note]
>You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the `cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser. If you’re interested in other functionality in the `rand` crate, for example, run `cargo doc --open` and click `rand` in the sidebar on the left.

The second new line prints the secret number. This is useful while we’re developing the program to be able to test it, but we’ll delete it from the final version. It’s not much of a game if the program prints the answer as soon as it starts!

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

You should get different random numbers, and they should all be numbers between 1 and 100. Great job!

## Related files 
- guessing_game_04