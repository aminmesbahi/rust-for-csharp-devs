// Lesson 01: Hello, Ownership
// Rust implementation
//
// Goal: compute the sum and maximum of an integer array.
// Along the way: understand ownership, move semantics, and Copy types.

// ── Helper: sum_and_max ──────────────────────────────────────────────────────
//
// We take a *slice* (&[i32]) rather than a Vec<i32>.
// A slice is a borrowed view into a sequence, it does not take ownership.
// (Borrowing is the topic of Lesson 02; just accept the & for now.)
//
// Returning two values: Rust functions can return tuples naturally.
// There is no "out parameter" pattern here.
fn sum_and_max(numbers: &[i32]) -> (i64, i32) {
    assert!(!numbers.is_empty(), "slice cannot be empty");

    let mut sum: i64 = 0;
    let mut max: i32 = numbers[0];

    for &n in numbers {
        sum += n as i64;
        if n > max {
            max = n;
        }
    }

    (sum, max) // no semicolon = this is the return value (expression syntax)
}

// ── Demonstrate: Move semantics ──────────────────────────────────────────────
//
// String owns heap-allocated data.
// Assigning a String to another variable MOVES ownership, the original is gone.
fn demonstrate_move_semantics() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves from s1 to s2

    // println!("{}", s1); // COMPILE ERROR: value moved to s2
    //                      // Uncomment to see the error message.
    println!("s2 = {s2}"); // fine, s2 is the owner now

    // s2 goes out of scope here: memory is freed. No GC. No leak. Guaranteed.
}

// ── Demonstrate: Copy semantics ──────────────────────────────────────────────
//
// i32, f64, bool, char, stack-only types, implement the Copy trait.
// Assignment duplicates the value. Both variables remain valid.
// This mirrors C# value-type behavior exactly.
fn demonstrate_copy_semantics() {
    let a: i32 = 10;
    let b = a; // a is COPIED, not moved, a remains valid

    println!("a = {a}"); // 10
    println!("b = {b}"); // 10

    // Modifying c does not affect a (they are independent stack slots).
    let mut c = a; // c is an independent copy of a
    c += 89;       // c = 99, a is untouched
    println!("a = {a}, c = {c}"); // a = 10, c = 99
}

// ── Demonstrate: Ownership through function calls ────────────────────────────
//
// In C#, passing a reference type to a function passes the reference, easy.
// In Rust, passing an owned type (String, Vec) MOVES it into the function.
// You cannot use it after the call (unless the function returns it back).
fn takes_ownership(s: String) {
    println!("Inside function: {s}");
    // s is dropped here, memory freed
}

fn gives_back_ownership(s: String) -> String {
    s // return the String, transferring ownership back to the caller
}

fn demonstrate_ownership_through_calls() {
    let s1 = String::from("world");

    takes_ownership(s1);
    // println!("{s1}"); // COMPILE ERROR: s1 was moved into takes_ownership

    let s2 = String::from("back");
    let s3 = gives_back_ownership(s2); // ownership moves: s2 → function → s3
    // s2 is no longer valid
    println!("s3 = {s3}"); // fine

    // --- Compare with Copy types ---
    let n1: i32 = 42;
    let _result = print_number(n1); // n1 is COPIED into the function
    println!("n1 = {n1}");          // still valid, Copy type is never moved
}

fn print_number(n: i32) -> i32 {
    println!("number = {n}");
    n
}

// ── main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Move Semantics ===");
    demonstrate_move_semantics();

    println!("\n=== Copy Semantics ===");
    demonstrate_copy_semantics();

    println!("\n=== Ownership Through Calls ===");
    demonstrate_ownership_through_calls();

    println!("\n=== Sum and Max ===");
    let data: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    // We pass &data, a borrowed reference to the Vec as a slice.
    // data still owns the memory. The function gets a read-only view.
    let (sum, max) = sum_and_max(&data);

    println!("Data:  {data:?}");
    println!("Sum:   {sum}");
    println!("Max:   {max}");

    // data is still valid here, we only lent it, didn't move it.
    println!("data[0] after call: {}", data[0]); // 3
}

// ── What you should take away ────────────────────────────────────────────────
//
// 1. Every value has one owner. When the owner's scope ends, the value is freed.
// 2. Assignment of owned heap types (String, Vec) = MOVE. Original is invalid.
// 3. Assignment of Copy types (i32, bool, etc.) = copy. Both remain valid.
// 4. Passing to a function follows the same rules: move or copy.
// 5. There is no GC. Memory management is resolved entirely at compile time.
//
// The & (borrow) in sum_and_max(&data) is the escape hatch for "I want to
// use this without taking ownership". That's Lesson 02.
