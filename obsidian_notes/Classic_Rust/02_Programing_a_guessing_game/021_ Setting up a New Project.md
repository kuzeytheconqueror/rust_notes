We will be basically using CARGO to create a guessing game. 

```
$ cargo new guessing_game
$ cd guessing_game
```

So we talked about the package structure at [[004_Hello Cargo!]] . It looks like this: 

```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

And at the begining the main.rs looks like: ,
```
fn main() {
    println!("Hello, world!");
}
```

Now we can compile this with with `cargo run`: 
```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/guessing_game`
Hello, world!
```

## Related files 
- guessing_game_04

---

> [!tip] Related Notes
> - [[004_Hello Cargo!|004 Hello Cargo!]] — the Cargo basics used here
> - [[005_Building and Running a Cargo Project|005 Building and Running a Cargo Project]] — building and running this project
> - [[022_Processing a Guess|022 Processing a Guess]] — the first code written in this project

---

← [[005_Building and Running a Cargo Project|005 Building and Running a Cargo Project]] · [[Rust Notes|⌂ Rust Notes]] · [[022_Processing a Guess|022 Processing a Guess]] →
