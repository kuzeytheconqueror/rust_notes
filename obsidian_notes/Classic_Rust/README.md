# Classic Rust

> [!important] Original stopping point
> All files that already existed in `Classic_Rust` were preserved byte-for-byte. The original notes stop in [[062_The Match Control Flow Construct|062 The Match Control Flow Construct]], immediately before the code for Listing 6-5. The copied material resumes in [[062B_The Match Control Flow Construct - Continuation|062B The Match Control Flow Construct - Continuation]].

## Source and scope

The generated continuation follows the [stable Rust 1.90 edition of *The Rust Programming Language*](https://doc.rust-lang.org/stable/book/title-page.html), pinned to official source revision [`05d114287b7d`](https://github.com/rust-lang/book/commit/05d114287b7d6f6c9253d5242540f00fbd6172ab). It covers the remainder of section 6.2, section 6.3, Chapters 7–21, and Appendices A–G.

The generated text is a near-verbatim Markdown conversion performed with AI assistance. Existing handwritten notes were not rewritten. Copyright © 2010 The Rust Project Developers; this copy uses the repository’s MIT licensing option. See [LICENSE-RUST-BOOK-MIT](LICENSE-RUST-BOOK-MIT).

## Table of Contents

> [!abstract] Hello World
> - [[001_Creating the enviorment|001 Creating the enviorment]]
> - [[002_Hello World|002 Hello World]]
> - [[003_  Anatomy of a Rust Program|003 Anatomy of a Rust Program]]
> - [[004_Hello Cargo!|004 Hello Cargo!]]
> - [[005_Building and Running a Cargo Project|005 Building and Running a Cargo Project]]
>
> [!abstract] Programing a guessing game
> - [[021_ Setting up a New Project|021 Setting up a New Project]]
> - [[022_Processing a Guess|022 Processing a Guess]]
> - [[023_Storing Values with Variables|023 Storing Values with Variables]]
> - [[024_Receiving User Input|024 Receiving User Input]]
> - [[025_Handling Potential Failure with Result|025 Handling Potential Failure with Result]]
> - [[026_Printing Values with println! Placeholders|026 Printing Values with println! Placeholders]]
> - [[027_Generating a secret Number|027 Generating a secret Number]]
> - [[028_Comparing the guess to the secret Number|028 Comparing the guess to the secret Number]]
> - [[029_Allowing Multiple Guesses with looping|029 Allowing Multiple Guesses with looping]]
>
> [!abstract] Common Programing Language Concepts
> - [[030_Common Programing Concepts|030 Common Programing Concepts]]
> - [[031_ Variables and Mutability|031 Variables and Mutability]]
> - [[032_Data Types|032 Data Types]]
> - [[033_Functions|033 Functions]]
> - [[034_Comments|034 Comments]]
> - [[035_Contol Flow|035 Contol Flow]]
>
> [!abstract] Understanding Ownership
> - [[040_Understanding Ownership|040 Understanding Ownership]]
> - [[041_What is ownership|041 What is ownership]]
> - [[042_References and Borrowing|042 References and Borrowing]]
> - [[043_ The Slice Type|043 The Slice Type]]
>
> [!abstract] Using Structs to Structure Related Data
> - [[050_Using Structs to Structure Related Data|050 Using Structs to Structure Related Data]]
> - [[051_Defining and Instantiating Structs|051 Defining and Instantiating Structs]]
> - [[052_An example program Using Structs|052 An example program Using Structs]]
> - [[053_Methods|053 Methods]]
>
> [!abstract] Enums and Pattern Matching
> - [[060_Enums and Pattern Matching|060 Enums and Pattern Matching]]
> - [[061_Defining an Enum|061 Defining an Enum]]
> - [[062_The Match Control Flow Construct|062 The Match Control Flow Construct]]
> - [[062B_The Match Control Flow Construct - Continuation|062B The Match Control Flow Construct - Continuation]]
> - [[063_Concise Control Flow with if let and let...else|063 Concise Control Flow with if let and let...else]]
>
> [!abstract] Packages, Crates, and Modules
> - [[070_Packages, Crates, and Modules|070 Packages, Crates, and Modules]]
> - [[071_Packages and Crates|071 Packages and Crates]]
> - [[072_Control Scope and Privacy with Modules|072 Control Scope and Privacy with Modules]]
> - [[073_Paths for Referring to an Item in the Module Tree|073 Paths for Referring to an Item in the Module Tree]]
> - [[074_Bringing Paths Into Scope with the use Keyword|074 Bringing Paths Into Scope with the use Keyword]]
> - [[075_Separating Modules into Different Files|075 Separating Modules into Different Files]]
>
> [!abstract] Common Collections
> - [[080_Common Collections|080 Common Collections]]
> - [[081_Storing Lists of Values with Vectors|081 Storing Lists of Values with Vectors]]
> - [[082_Storing UTF-8 Encoded Text with Strings|082 Storing UTF-8 Encoded Text with Strings]]
> - [[083_Storing Keys with Associated Values in Hash Maps|083 Storing Keys with Associated Values in Hash Maps]]
>
> [!abstract] Error Handling
> - [[090_Error Handling|090 Error Handling]]
> - [[091_Unrecoverable Errors with panic!|091 Unrecoverable Errors with panic!]]
> - [[092_Recoverable Errors with Result|092 Recoverable Errors with Result]]
> - [[093_To panic! or Not to panic!|093 To panic! or Not to panic!]]
>
> [!abstract] Generic Types, Traits, and Lifetimes
> - [[100_Generic Types, Traits, and Lifetimes|100 Generic Types, Traits, and Lifetimes]]
> - [[101_Generic Data Types|101 Generic Data Types]]
> - [[102_Defining Shared Behavior with Traits|102 Defining Shared Behavior with Traits]]
> - [[103_Validating References with Lifetimes|103 Validating References with Lifetimes]]
>
> [!abstract] Writing Automated Tests
> - [[110_Writing Automated Tests|110 Writing Automated Tests]]
> - [[111_How to Write Tests|111 How to Write Tests]]
> - [[112_Controlling How Tests Are Run|112 Controlling How Tests Are Run]]
> - [[113_Test Organization|113 Test Organization]]
>
> [!abstract] An I and O Project - Building a Command Line Program
> - [[120_An I and O Project - Building a Command Line Program|120 An I and O Project - Building a Command Line Program]]
> - [[121_Accepting Command Line Arguments|121 Accepting Command Line Arguments]]
> - [[122_Reading a File|122 Reading a File]]
> - [[123_Refactoring to Improve Modularity and Error Handling|123 Refactoring to Improve Modularity and Error Handling]]
> - [[124_Adding Functionality with Test Driven Development|124 Adding Functionality with Test Driven Development]]
> - [[125_Working with Environment Variables|125 Working with Environment Variables]]
> - [[126_Redirecting Errors to Standard Error|126 Redirecting Errors to Standard Error]]
>
> [!abstract] Functional Language Features - Iterators and Closures
> - [[130_Functional Language Features - Iterators and Closures|130 Functional Language Features - Iterators and Closures]]
> - [[131_Closures|131 Closures]]
> - [[132_Processing a Series of Items with Iterators|132 Processing a Series of Items with Iterators]]
> - [[133_Improving Our I and O Project|133 Improving Our I and O Project]]
> - [[134_Performance in Loops vs. Iterators|134 Performance in Loops vs. Iterators]]
>
> [!abstract] More about Cargo and Crates.io
> - [[140_More about Cargo and Crates.io|140 More about Cargo and Crates.io]]
> - [[141_Customizing Builds with Release Profiles|141 Customizing Builds with Release Profiles]]
> - [[142_Publishing a Crate to Crates.io|142 Publishing a Crate to Crates.io]]
> - [[143_Cargo Workspaces|143 Cargo Workspaces]]
> - [[144_Installing Binaries with cargo install|144 Installing Binaries with cargo install]]
> - [[145_Extending Cargo with Custom Commands|145 Extending Cargo with Custom Commands]]
>
> [!abstract] Smart Pointers
> - [[150_Smart Pointers|150 Smart Pointers]]
> - [[151_Using Box(T) to Point to Data on the Heap|151 Using Box(T) to Point to Data on the Heap]]
> - [[152_Treating Smart Pointers Like Regular References|152 Treating Smart Pointers Like Regular References]]
> - [[153_Running Code on Cleanup with the Drop Trait|153 Running Code on Cleanup with the Drop Trait]]
> - [[154_Rc(T), the Reference Counted Smart Pointer|154 Rc(T), the Reference Counted Smart Pointer]]
> - [[155_RefCell(T) and the Interior Mutability Pattern|155 RefCell(T) and the Interior Mutability Pattern]]
> - [[156_Reference Cycles Can Leak Memory|156 Reference Cycles Can Leak Memory]]
>
> [!abstract] Fearless Concurrency
> - [[160_Fearless Concurrency|160 Fearless Concurrency]]
> - [[161_Using Threads to Run Code Simultaneously|161 Using Threads to Run Code Simultaneously]]
> - [[162_Transfer Data Between Threads with Message Passing|162 Transfer Data Between Threads with Message Passing]]
> - [[163_Shared-State Concurrency|163 Shared-State Concurrency]]
> - [[164_Extensible Concurrency with Send and Sync|164 Extensible Concurrency with Send and Sync]]
>
> [!abstract] Fundamentals of Asynchronous Programming - Async, Await, Futures, and Streams
> - [[170_Fundamentals of Asynchronous Programming - Async, Await, Futures, and Streams|170 Fundamentals of Asynchronous Programming - Async, Await, Futures, and Streams]]
> - [[171_Futures and the Async Syntax|171 Futures and the Async Syntax]]
> - [[172_Applying Concurrency with Async|172 Applying Concurrency with Async]]
> - [[173_Working With Any Number of Futures|173 Working With Any Number of Futures]]
> - [[174_Streams - Futures in Sequence|174 Streams - Futures in Sequence]]
> - [[175_A Closer Look at the Traits for Async|175 A Closer Look at the Traits for Async]]
> - [[176_Futures, Tasks, and Threads|176 Futures, Tasks, and Threads]]
>
> [!abstract] Object Oriented Programming Features
> - [[180_Object Oriented Programming Features|180 Object Oriented Programming Features]]
> - [[181_Characteristics of Object-Oriented Languages|181 Characteristics of Object-Oriented Languages]]
> - [[182_Using Trait Objects to Abstract over Shared Behavior|182 Using Trait Objects to Abstract over Shared Behavior]]
> - [[183_Implementing an Object-Oriented Design Pattern|183 Implementing an Object-Oriented Design Pattern]]
>
> [!abstract] Patterns and Matching
> - [[190_Patterns and Matching|190 Patterns and Matching]]
> - [[191_All the Places Patterns Can Be Used|191 All the Places Patterns Can Be Used]]
> - [[192_Refutability - Whether a Pattern Might Fail to Match|192 Refutability - Whether a Pattern Might Fail to Match]]
> - [[193_Pattern Syntax|193 Pattern Syntax]]
>
> [!abstract] Advanced Features
> - [[200_Advanced Features|200 Advanced Features]]
> - [[201_Unsafe Rust|201 Unsafe Rust]]
> - [[202_Advanced Traits|202 Advanced Traits]]
> - [[203_Advanced Types|203 Advanced Types]]
> - [[204_Advanced Functions and Closures|204 Advanced Functions and Closures]]
> - [[205_Macros|205 Macros]]
>
> [!abstract] Final Project - Building a Multithreaded Web Server
> - [[210_Final Project - Building a Multithreaded Web Server|210 Final Project - Building a Multithreaded Web Server]]
> - [[211_Building a Single-Threaded Web Server|211 Building a Single-Threaded Web Server]]
> - [[212_From Single-Threaded to Multithreaded Server|212 From Single-Threaded to Multithreaded Server]]
> - [[213_Graceful Shutdown and Cleanup|213 Graceful Shutdown and Cleanup]]
>
> [!abstract] Appendix
> - [[A00_Appendix|A00 Appendix]]
> - [[A01_A - Keywords|A01 A - Keywords]]
> - [[A02_B - Operators and Symbols|A02 B - Operators and Symbols]]
> - [[A03_C - Derivable Traits|A03 C - Derivable Traits]]
> - [[A04_D - Useful Development Tools|A04 D - Useful Development Tools]]
> - [[A05_E - Editions|A05 E - Editions]]
> - [[A06_F - Translations of the Book|A06 F - Translations of the Book]]
> - [[A07_G - How Rust is Made and Nightly Rust|A07 G - How Rust is Made and Nightly Rust]]
>
