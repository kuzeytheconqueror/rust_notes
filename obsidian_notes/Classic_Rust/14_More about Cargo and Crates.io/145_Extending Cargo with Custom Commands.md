Cargo is designed so that you can extend it with new subcommands without having
to modify it. If a binary in your `$PATH` is named `cargo-something`, you can
run it as if it were a Cargo subcommand by running `cargo something`. Custom
commands like this are also listed when you run `cargo --list`. Being able to
use `cargo install` to install extensions and then run them just like the
built-in Cargo tools is a super-convenient benefit of Cargo’s design!

## Summary

Sharing code with Cargo and [crates.io](https://crates.io/) is
part of what makes the Rust ecosystem useful for many different tasks. Rust’s
standard library is small and stable, but crates are easy to share, use, and
improve on a timeline different from that of the language. Don’t be shy about
sharing code that’s useful to you on [crates.io](https://crates.io/)<!-- ignore
-->; it’s likely that it will be useful to someone else as well!

---
> [!info] Source
> Nearly verbatim from [Extending Cargo with Custom Commands](https://doc.rust-lang.org/stable/book/ch14-05-extending-cargo.html) in *The Rust Programming Language*.

> [!tip] Related Notes
> - [[140_More about Cargo and Crates.io|More about Cargo and Crates.io]] — chapter overview
> - [[144_Installing Binaries with cargo install|Installing Binaries with cargo install]] — previous section
> - [[150_Smart Pointers|Smart Pointers]] — next section

---
← [[144_Installing Binaries with cargo install|Installing Binaries with cargo install]] · [[README|⌂ Classic Rust Index]] · [[150_Smart Pointers|Smart Pointers]] →
