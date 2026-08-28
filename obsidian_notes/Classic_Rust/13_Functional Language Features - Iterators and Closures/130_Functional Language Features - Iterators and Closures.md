Rust’s design has taken inspiration from many existing languages and
techniques, and one significant influence is _functional programming_.
Programming in a functional style often includes using functions as values by
passing them in arguments, returning them from other functions, assigning them
to variables for later execution, and so forth.

In this chapter, we won’t debate the issue of what functional programming is or
isn’t but will instead discuss some features of Rust that are similar to
features in many languages often referred to as functional.

More specifically, we’ll cover:

- _Closures_, a function-like construct you can store in a variable
- _Iterators_, a way of processing a series of elements
- How to use closures and iterators to improve the I/O project in Chapter 12
- The performance of closures and iterators (spoiler alert: They’re faster than
  you might think!)

We’ve already covered some other Rust features, such as pattern matching and
enums, that are also influenced by the functional style. Because mastering
closures and iterators is an important part of writing fast, idiomatic, Rust
code, we’ll devote this entire chapter to them.

---
> [!info] Source
> Nearly verbatim from [Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html) in *The Rust Programming Language*.

> [!tip] Related Notes
> - [[126_Redirecting Errors to Standard Error|Redirecting Errors to Standard Error]] — previous section
> - [[131_Closures|Closures]] — next section

---
← [[126_Redirecting Errors to Standard Error|Redirecting Errors to Standard Error]] · [[README|⌂ Classic Rust Index]] · [[131_Closures|Closures]] →
