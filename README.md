# Rust Book Code & Progress

The Rust Programming Language

   - [x] Getting Started
    - [x] Installation
    - [x] Hello, World!
    - [x] Hello, Cargo!

  - [x] Programming a Guessing Game

  - [x] Common Programming Concepts
    - [x] Variables and Mutability
    - [x] Data Types
    - [x] How Functions Work
    - [x] Comments
    - [x] Control Flow

  - [ ] Understanding Ownership
    - [ ] What is Ownership?
    - [ ] References & Borrowing
    - [ ] Slices

  - [ ] Using Structs to Structure Related Data
    - [ ] Defining and Instantiating Structs
    - [ ] An Example Program Using Structs
    - [ ] Method Syntax

  - [ ] Enums and Pattern Matching
    - [ ] Defining an Enum
    - [ ] The match Control Flow Operator
    - [ ] Concise Control Flow with if let

- Basic Rust Literacy

  - Modules
    - mod and the Filesystem
    - Controlling Visibility with pub
    - Referring to Names in Different Modules

  - Common Collections
    - Vectors
    - Strings
    - Hash Maps

  - Error Handling
    - Unrecoverable Errors with panic!
    - Recoverable Errors with Result
    - To panic! or Not to panic!

  - Generic Types, Traits, and Lifetimes
    - Generic Data Types
    - Traits: Defining Shared Behavior
    - Validating References with Lifetimes

  - Testing
    - Writing tests
    - Running tests
    - Test Organization

  - An I/O Project: Building a Command Line Program
    - Accepting Command Line Arguments
    - Reading a File
    - Refactoring to Improve Modularity and Error Handling
    - Developing the Library’s Functionality with Test Driven Development
    - Working with Environment Variables
    - Writing Error Messages to Standard Error Instead of Standard Output

- Thinking in Rust

  - Functional Language Features: Iterators and Closures
    - Closures: Anonymous Functions that Can Capture Their Environment
    - Processing a Series of Items with Iterators
    - Improving Our I/O Project
    - Comparing Performance: Loops vs. Iterators

  - More about Cargo and Crates.io
    - Customizing Builds with Release Profiles
    - Publishing a Crate to Crates.io
    - Cargo Workspaces
    - Installing Binaries from Crates.io with cargo install
    - Extending Cargo with Custom Commands

  - Smart Pointers
    - Box<T> Points to Data on the Heap and Has a Known Size
    - The Deref Trait Allows Access to the Data Through a Reference
    - The Drop Trait Runs Code on Cleanup
    - Rc<T>, the Reference Counted Smart Pointer
    - RefCell<T> and the Interior Mutability Pattern
    - Creating Reference Cycles and Leaking Memory is Safe

  - Fearless Concurrency
    - Threads
    - Message Passing
    - Shared State
    - Extensible Concurrency: Sync and Send

  - Object Oriented Programming Features of Rust
    - Characteristics of Object-Oriented Languages
    - Using Trait Objects that Allow for Values of Different Types
    - Implementing an Object-Oriented Design Pattern

- Advanced Topics

  - Patterns Match the Structure of Values
    - All the Places Patterns May be Used
    - Refutability: Whether a Pattern Might Fail to Match
    - All the Pattern Syntax

  - Advanced Features
    - Unsafe Rust
    - Advanced Lifetimes
    - Advanced Traits
    - Advanced Types
    - Advanced Functions & Closures

  - Final Project: Building a Multithreaded Web Server
    - A Single Threaded Web Server
    - Turning our Single Threaded Server into a Multithreaded Server
    - Graceful Shutdown and Cleanup
