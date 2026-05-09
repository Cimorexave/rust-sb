// ============================================================
// Rust Basics: Variables, Functions, Control Flow, Ownership
// ============================================================

mod temp; // Import the temp module (defined in src/temp.rs)
mod fibo; // Import the fibo module (defined in src/fibo.rs)
// mod todo; // Import the todo module (defined in src/todo.rs)


fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Returns the larger of two integers.
fn max_of(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Takes ownership of a String and prints it.
/// After this call, the string is dropped (freed).
fn consume_string(s: String) {
    println!("  Consumed: {s}");
    // s is dropped here — no longer available to the caller
}

/// Takes a reference (&String) — borrows without taking ownership.
fn borrow_string(s: &String) {
    println!("  Borrowed (read-only): {s}");
    // s is NOT dropped; caller keeps ownership
}

/// Takes a mutable reference — can modify the original value.
fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

fn main() {
    println!("Choose an application to run:");
    println!("1. Temperature Converter");
    println!("2. Fibonacci Calculator");
    println!("3. TODO List Manager");
    
    let app_choice = loop {
        println!("Enter your choice (1, 2, or 3):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "1" | "2" | "3" => break input,
            _ => println!("Invalid choice! Please enter 1, 2, or 3."),
        }
    };
    match app_choice.trim() {
        "1" => {
            println!("Running Temperature Converter...");
            while temp::temp_convertor::run() {}
            return;
        },
        "2" => {
            println!("Running Fibonacci Calculator...");
            // Fibonacci is calculated below in the main function
            let mut fibo_index: u32 = 0;
            println!("Enter the Fibonacci index (non-negative integer):");
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                match input.trim().parse::<u32>() {
                    Ok(val) => {
                        fibo_index = val;
                        break;
                    },
                    Err(_) => println!("Invalid input! Please enter a non-negative integer."),
                }
            }
            print!("Fibonacci of {fibo_index} is {}", fibo::fibo::fib(fibo_index));
            return;
        },
        "3" => {
            println!("Running TODO List Manager...");
            // todo::run();
            return;
        },
        _ => {
            println!("Unexpected error: invalid choice");
            return;
        },
    }

    // --------------------------------------------------
    // 1. Variables — immutable by default
    // --------------------------------------------------
    println!("=== Variables ===");

    let x = 5; // immutable
    // x = 6;  // <-- would not compile

    let mut y = 10; // mutable with `mut`
    println!("  x = {x}, y = {y}");

    y = 20; // allowed because y is mutable
    println!("  y changed to {y}");

    // Type annotation
    let z: u32 = 42;
    println!("  z (explicit u32) = {z}");

    // --------------------------------------------------
    // 2. Functions
    // --------------------------------------------------
    println!("\n=== Functions ===");

    let sum = add(3, 7);
    println!("  3 + 7 = {sum}");

    let bigger = max_of(10, 4);
    println!("  max(10, 4) = {bigger}");

    // --------------------------------------------------
    // 3. Control Flow
    // --------------------------------------------------
    println!("\n=== Control Flow ===");

    // if / else
    let n = 7;
    if n % 2 == 0 {
        println!("  {n} is even");
    } else {
        println!("  {n} is odd");
    }

    // if used as an expression (returns a value)
    let category = if n < 5 { "small" } else { "large" };
    println!("  {n} is {category}");

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2; // break with a value
        }
    };
    println!("  loop result: {result}");

    // while
    let mut countdown = 3;
    while countdown > 0 {
        print!("  {countdown}...");
        countdown -= 1;
    }
    println!(" lift off!");

    // for (range)
    print!("  counting: ");
    for i in 0..5 {
        // 0, 1, 2, 3, 4
        print!("{i} ");
    }
    println!();

    // for (iterate over array)
    let numbers = [10, 20, 30];
    print!("  array: ");
    for num in numbers {
        print!("{num} ");
    }
    println!();

    // --------------------------------------------------
    // 4. Ownership & Borrowing
    // --------------------------------------------------
    println!("\n=== Ownership & Borrowing ===");

    // Ownership — String (heap-allocated)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2
    // println!("{s1}"); // <-- would not compile: s1 is no longer valid
    println!("  After move: s2 = {s2}");

    // Clone (deep copy) — both stay valid
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("  After clone: s3 = {s3}, s4 = {s4}");

    // Ownership transfer via function
    let s5 = String::from("owned");
    consume_string(s5);
    // println!("{s5}"); // <-- would not compile: s5 was moved into the function

    // Borrowing (references)
    let s6 = String::from("borrowed");
    borrow_string(&s6); // &s6 creates a reference — s6 is NOT moved
    println!("  s6 still available after borrow: {s6}");

    // Mutable borrow
    let mut s7 = String::from("hello");
    append_exclamation(&mut s7);
    println!("  After mutable borrow: {s7}");

    // --------------------------------------------------
    // 5. Common patterns: slices & iteration
    // --------------------------------------------------
    println!("\n=== Slices ===");

    let words = String::from("hello world rust");
    let first_word = &words[0..5]; // slice: "hello"
    println!("  First word: {first_word}");

    // Iterate over characters
    print!("  Chars: ");
    for ch in "Rust".chars() {
        print!("{ch} ");
    }
    println!();

    println!("\n✅ All examples completed!");
}
