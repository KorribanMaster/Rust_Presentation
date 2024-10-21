---
marp: true
theme: baxter
class:
    - invert
paginate: true
---
<!-- _class: lead -->
<!-- paginate: skip -->

# Rust

![bg right:33% contain](./assets/Rust_programming_language_black_logo.svg)

**Introduction to Rust for C Developers**
 By: Eicke Hecht

<style>
  img {
    background-color: transparent;
  }
</style>

---

# Why Rust?

**Rust’s Popularity Explained**

- Memory safety without a garbage collector
- Concurrency without data races
- Modern syntax features in a system level language
- C like performance
- Growing popularity: Rust has topped StackOverflow’s “Most Loved” language list since 2016

---

# Common Ground Between C and Rust

**Familiar Concepts for C Developers**

- Systems-level control over memory and hardware
- No garbage collector, like C
- Compiled language

---

# Rust’s Memory Safety: Ownership and Borrowing

**C’s Manual Memory vs Rust’s Automated Approach**

- **C**: You must explicitly manage memory (`malloc`, `free`)
- **Rust**: Ownership and borrowing system ensures memory safety at compile-time

  ```c
  // C code example
  int *p = malloc(sizeof(int));
  *p = 10;
  free(p);
  ```

  ```rust
  {
    // Rust code example
    let x = Box::new(10); // No need to free manually
  }
  ```

---


# Borrowing in Rust

- **Ownership**: In Rust, each value has a unique owner.
- **Borrowing**: Allows temporarily "borrowing" a value without taking ownership.
  - **Immutable Borrowing** (`&T`):
    - Multiple references allowed.
    - No modification is allowed while borrowed.
  - Example:
  
    ```rust
    fn print_value(val: &i32) {
        println!("Value: {}", val);
    }
    
    let x = 10;
    print_value(&x); // Borrow `x`
    ```

---

# Mutable Borrowing

- **Mutable Borrowing** (`&mut T`):
  - Allows exclusive mutable access to a value.
  - Only one mutable reference is allowed at a time.
  - Ensures no data races and memory safety.
  - Example:

    ```rust
    fn increment(val: &mut i32) {
        *val += 1;
    }
    
    let mut x = 10;
    increment(&mut x); // Mutably borrow `x`
    println!("{}", x); // 11
    ```

---

# Borrowing Rules

- **Rules**:
  - You can have either **one mutable** reference or **many immutable** references, but not both at the same time.
  - Ensures data integrity and prevents race conditions.
- Example violation:

    ```rust
    let mut x = 10;
    let r1 = &x;
    let r2 = &mut x; // Error: Cannot borrow `x` as mutable because it’s already borrowed as immutable
    ```

- The borrow checker ensures these rules at compile time.

---

# Rust’s Memory Safety: Lifetimes

  ```rust
  fn main() {
      let r;

      {
          let x = 5;
          r = &x;
      }
      // x was dropped
      println!("r: {r}"); 
      // error x does not live long enough
  }
  ```

---

# Lifetimes explained

```text
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let x = 5;
  |             - binding `x` declared here
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {r}");
  |                  --- borrow later used here
  ```

  Rust ensures memory is freed automatically when the variable goes out of scope

---
# Lifetime problems

- The compiler sometimes needs help
- Example:

 ```rust
 fn main() {
  let string1 = String::from("abcd");
  let string2 = String::from("xyz");
  let result = longest(string1.as_str(), string2.as_str());
  println!("The longest string is {}", result);
 }

fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
 ```

---

# Lifetimes

- Fixed:

 ```rust
 fn main() {
  let string1 = String::from("abcd");
  let string2 = String::from("xyz");
  let result = longest(string1.as_str(), string2.as_str());
  println!("The longest string is {}", result);
 }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
 ```

 ---

# Concurrency in Rust vs C

**Concurrency Without Fear**

- **C**: Threads and locks, manual management prone to errors
- **Rust**: Concurrency managed via ownership and borrowing

  ```rust
  // Safe multithreading in Rust
  let handle = std::thread::spawn(|| {
      println!("Hello from a thread!");
  });
  handle.join().unwrap();
  ```

  Rust's compiler enforces data race prevention at compile-time

---

# Zero-Cost Abstractions

**Performance: Rust vs C**

- Both languages offer low-level control and efficiency
- **Rust**: Zero-cost abstractions, meaning high-level constructs like iterators compile down to efficient code

  ```rust
  // Rust zero-cost abstraction example
  let squares: Vec<_> = (0..10).map(|i| i * i).collect();
  ```

  This abstraction performs just as well as if you wrote the loop manually

---

# Error Handling in Rust

**Safe and Structured Error Handling**

- **C**: Relies on error codes or global variables (`errno`)
- **Rust**: Uses `Result` and `Option` types for explicit error handling

  ```rust
  fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
      if b == 0 {
          Err("Division by zero")
      } else {
          Ok(a / b)
      }
  }
  ```

---

# Rust’s Detailed Error Reporting

- **rustc** provides very informative error messages, making it easier to debug issues.
- Helps developers fix issues **faster** and **safely**.

---

## Example: Error in Rust vs C

### C Error

```c
int main() {
    int* x = NULL;
    *x = 10; // Segmentation fault
    return 0;
}
```

- Fails at runtime
- **Error**: `Segmentation fault (core dumped)`
- Minimal information, doesn't explain why or where it failed.

---

## Rust Error Example

```rust
fn main() {
    let x = 10;
    let r1 = &x;
    let r2 = &mut x; // ERROR: cannot borrow `x` as mutable
}
```

---

### Rust's `rustc` Error

```text
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:14
  |
3 |     let r1 = &x;
  |               -- immutable borrow occurs here
4 |     let r2 = &mut x;
  |              ^^^^^^ mutable borrow occurs here
5 |     println!("{}", r1);
  |                    -- immutable borrow later used here
```

- Error is caught at compile time
- Provides **line numbers** and clear explanations of **why** the error occurred.
- Suggests a solution and directs to `rustc --explain [ERROR_CODE]` for more details.

---

### Tooling and Ecosystem

**Rust’s Tooling: Cargo**

- **C**: Uses `make` and manual dependency management
- **Rust**: `Cargo` for build automation, dependency management, testing, and documentation
  - Integrated tools for benchmarking, fuzzing, and formatting
  - Rich ecosystem via `crates.io`
  
---

# Typical Rust Project Structure

A typical Rust project structure generated by Cargo looks like this:

```
my_project/
│
├── Cargo.toml    # Project metadata and dependencies
├── Cargo.lock    # Versions of dependencies (auto-generated)
└── src/
    ├── main.rs   # The main entry point of the program
    └── lib.rs    # Code for reusable libraries (optional)
```

- **Cargo.toml**: Specifies project details like dependencies.
- **Cargo.lock**: Locks specific versions of dependencies.
- **src/**: Contains Rust source code (`main.rs` for binaries, `lib.rs` for libraries).

---

# Cargo: Rust’s Build and Package Manager

**Cargo** is Rust's integrated tool for managing:

- **Building**: Compiles your code (`cargo build`).
- **Testing**: Runs tests (`cargo test`).
- **Documentation**: Generates documentation (`cargo doc`).
- **Dependencies**: Manages third-party libraries (`crates.io`).

- Key Cargo commands:
  - `cargo new my_project`: Creates a new project.
  - `cargo run`: Compiles and runs the project.
  - `cargo build --release`: Builds optimized code for release.

---

# Cargo.toml Explained

The **Cargo.toml** file is at the core of every Rust project. It contains:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"  # Example dependency
```

- **[package]**: Contains project metadata (name, version, etc.).
- **[dependencies]**: Lists external libraries (crates) with version numbers from [crates.io](https://crates.io).
- Cargo makes managing dependencies straightforward and handles fetching, compiling, and updating them.

---

# Why Cargo matters?

- **All-in-one tool**: From building to testing, Cargo simplifies Rust development.
- **Efficient dependency management**: Cargo handles versioning and builds with ease.
- **Community-driven ecosystem**: Crates.io hosts a wide range of packages for use in Rust projects.

---

# Conclusion

**Why Rust is Worth Exploring for C Developers**

- Memory safety without garbage collection
- Competitive performance, even in systems programming
- Increasingly adopted in industry for reliability and concurrency
  - Rust offers solutions to longstanding challenges in C development like memory safety and concurrency bugs.
- Rich ecosystem

---
<!-- _class: lead -->
<!-- paginate: skip -->

# Thank you for your attention

![bg right:33% contain](./assets/Rust_programming_language_black_logo.svg)
