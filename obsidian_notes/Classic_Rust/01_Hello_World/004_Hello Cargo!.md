Cargo seems like the pip for python or maven for java. It is a build system and packet manager. 

Its claimed to be handing the build the code, downloading libraries and dependencies and preparing for you. ,

Check the cargo version by:
```
$ cargo --version
```

If you want to generate a package with the cargo, you need to write:
```
$ cargo new hello_cargo
$ cd hello_cargo
```

Interesting think is, it creates a complete repository for you. a ".git", ".gitignore", ".toml" for packet management and so on. 

They wrote you can override the git behavior via: 
```
cargo new --vcs=git
```

I think the note they put is detailed enough about the topic: 
> [!note]
> ```
> Note: Git is a common version control system. You can change `cargo new` to use a different version control system or no version control system by using the `--vcs` flag. Run `cargo new --help` to see the available options.
> ```

This file is in the [_TOML_](https://toml.io) (_Tom’s Obvious, Minimal Language_) format, which is Cargo’s configuration format.

The first line, `[package]`, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use.

The last line, `[dependencies]`, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as _crates_. We won’t need any other crates for this project.

If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. Move the project code into the _src_ directory and create an appropriate _Cargo.toml_ file. One easy way to get that _Cargo.toml_ file is to run `cargo init`, which will create it for you automatically.

## Related files 
- hello_cargo_02