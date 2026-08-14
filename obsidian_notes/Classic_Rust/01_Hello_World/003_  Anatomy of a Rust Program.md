When we start to look "Hello world" code, we can start to analyze how it works:

```
fn main() {

}
```

**Main** is a special function. It is always the **first code that runs** in every executable. 

> [!note]
> There is a note that written at Rust documentation. I am not sure that did I understand. But still want to put because it seems like they care about it:
> ```
> Note: If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called `rustfmt` to format your code in a particular style (more on `rustfmt` in [Appendix D](https://doc.rust-lang.org/stable/book/appendix-04-useful-development-tools.html)). The Rust team has included this tool with the standard Rust distribution, as `rustc` is, so it should already be installed on your computer!
> ```

So basically if the line has "**!**", it means it calls a RUST macro. If "**println!**" was "**println**" than it means it calls a function instead.  

We talked about it but before running, tou need to compile it with:
```
$ rustc main.rs
```

Then you need to run it via:
```
$ ./main # or .\main on Windows
```

Rust is not like JS or Python. Its not a dynamic programming language in that way. 

Rust is an **ahead-of-time compiled language**, meaning you can compile a program and give the executable to someone else, and they can run it even without having rust installed. 

For Python or JS, you can run with one command. At the end it seems like it is a trade-off. 
