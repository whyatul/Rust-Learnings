# Guessing Game in Rust

A scommand‑line based guessing game. It started as a simple exercise and became a handy playground to understand mutability, shadowing, pattern matching, and using external crates.

## What I've Learned

### 1. Rust Syntax and Structure

- Brought items into scope with `use` (e.g. `use std::cmp::Ordering;`)
- Defined the entry point with `fn main()`
- Declared variables with `let` (immutable) and `let mut` (mutable)
- Noticed how explicit most things are (types, error handling), which helps avoid surprises

### 2. Variables, Mutability, and Shadowing

- Immutable by default: `let x = 5;` cannot be reassigned
- Mutable with `mut`: `let mut guess = String::new();` so input can be appended
- Shadowing: `let guess: u32 = ...` creates a *new* binding with the same name after parsing
  - Mutation = same binding, different value
  - Shadowing = new binding, possibly new type or mutability
- Why shadow here: keeps the name `guess` readable while converting from `String` to `u32`

### 3. Data Types

- `String`: growable text buffer (`String::new()`)
- `u32`: unsigned 32‑bit integer for the guess
- Parsing: `guess.trim().parse::<u32>()` -> `Result<u32, _>` handled with `match`

### 4. Input / Output and References

- Read input: `io::stdin().read_line(&mut guess)` (mutable reference so the buffer can be changed)
- Trim: remove newline before parsing
- Output: `println!` with inline formatting: `println!("You guessed: {guess}");`

### 5. Error Handling and Control Flow

- `match` on parse result to recover from invalid input
- `continue` to retry on bad input instead of crashing
- Infinite `loop {}` plus `break` on success

### 6. Random Number Generation (External Crate)

- Dependency: `rand = "0.9.2"`
- Secret number: `rand::rng().random_range(1..=100)` (newer API style)
- Trait import brings the method into scope

### 7. Comparison and Ordering

- `cmp` returns `Ordering` which is matched for feedback

### 8. Ownership, Borrowing, Safety (Light Intro)

- `&mut guess` vs `&secret_number`
- Compiler enforces safe mutation and borrowing rules automatically

### 9. Organization and Comments

- Comments focus on *why* (shadowing, parse choices) not just *what*

## Program Flow Summary

1. Greet user
2. Generate secret number (1–100)
3. Loop: read input
4. Validate / parse
5. Compare & hint
6. Exit on correct guess



## Core Snippet

```rust
let secret_number = rand::rng().random_range(1..=100);
    
    loop {
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Equal => {
                println!("You win! {guess} is correct!");
                println!("The secret number was {secret_number}");
                break;
            },
            Ordering::Greater=> println!("{guess} is too big!"),
        }
    }
```

