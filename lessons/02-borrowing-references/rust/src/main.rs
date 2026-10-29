// Lesson 02: Borrowing & References
// Rust implementation
//
// Goal:
//   1. Linear search, find the index of a target in an unsorted array
//   2. Double every element in-place
//
// New concepts: &T (immutable borrow), &mut T (mutable borrow), &[T] slices

// ── Linear Search ─────────────────────────────────────────────────────────────
//
// Parameter type: &[i32]
//   - & = borrowed reference (we do not take ownership)
//   - [i32] = a slice, a view into a contiguous sequence of i32 values
//
// This is the idiomatic way to accept an array/vector for reading.
// It works with Vec<i32>, [i32; N], and sub-slices, all coerce to &[i32].
//
// Return type: Option<usize>
//   - Some(index) if found
//   - None if not found
//   No magic -1. The type system enforces that callers handle both cases.
fn linear_search(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in numbers.iter().enumerate() {
        // `.iter()` on a `&[i32]` yields `&i32`, so the `&value` pattern
        // destructures that reference into a plain `i32`.
        // Equivalent to: let value = *item;
        if value == target {
            return Some(index);
        }
    }
    None
}

// ── Double In-Place ──────────────────────────────────────────────────────────
//
// Parameter type: &mut [i32]
//   - &mut = mutable borrow (we can read and write, but not own)
//   - [i32] = a slice
//
// The caller must have declared the Vec as `mut` and passes `&mut data`.
// While this borrow is active, no other reference to the data can exist.
//
// `iter_mut()` is spelled out rather than using `for value in numbers`
// because it makes the borrow explicit, which is the point of the lesson.
#[allow(clippy::explicit_iter_loop)]
fn double_in_place(numbers: &mut [i32]) {
    for value in numbers.iter_mut() {
        *value *= 2;   // dereference to write through the mutable reference
    }
}

// ── Demonstrate: Borrow Checker Rules ────────────────────────────────────────
fn demonstrate_borrow_rules() {
    let mut data = vec![1, 2, 3, 4, 5];

    // RULE 1: multiple immutable borrows are fine
    {
        let r1 = &data;
        let r2 = &data;
        println!("r1: {r1:?}");
        println!("r2: {r2:?}");
        // r1 and r2 go out of scope (their last use was the println above)
    }

    // RULE 2: one mutable borrow, exclusive access
    {
        let r3 = &mut data;
        r3.push(6);
        // println!("{data:?}");  // COMPILE ERROR: data is mutably borrowed by r3
        //                         // Uncomment to see the error.
        println!("After push: {r3:?}");
        // r3's last use was above, mutable borrow ends here
    }

    // Now data is accessible again
    println!("data: {data:?}");

    // RULE 3: Cannot have &mut and & at the same time
    // Uncomment the block below to see the compiler error:
    //
    // let r_immut = &data;
    // let r_mut   = &mut data;   // ERROR: cannot borrow as mutable because
    //                             //        it's also borrowed as immutable
    // println!("{r_immut:?} {r_mut:?}");
}

// ── Demonstrate: Slices ───────────────────────────────────────────────────────
//
// &[T] is a "fat pointer": a pointer + length.
// It does not own the data and carries no heap allocation.
// Equivalent to C#'s ReadOnlySpan<T>.
fn demonstrate_slices() {
    let data = vec![10, 20, 30, 40, 50];

    // A slice of the whole Vec
    let all: &[i32] = &data;

    // A slice of part of the Vec, no copy, just a view
    let middle: &[i32] = &data[1..4];   // [20, 30, 40]

    println!("all:    {all:?}");
    println!("middle: {middle:?}");

    // The sum function works on any &[i32], Vec, array, or sub-slice
    fn sum(s: &[i32]) -> i64 {
        s.iter().map(|&x| x as i64).sum()
    }

    let arr = [1, 2, 3, 4, 5];   // fixed-size array, lives on the stack
    println!("sum of vec:   {}", sum(&data));
    println!("sum of array: {}", sum(&arr));
    println!("sum of slice: {}", sum(middle));
}

// ── Demonstrate: Option<T> return from search ─────────────────────────────────
//
// In C# you'd return -1 for "not found". Rust uses Option<usize>.
// The caller is forced by the type system to handle the None case.
fn demonstrate_option_handling() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // Pattern match, exhaustive, must handle both cases
    match linear_search(&data, 9) {
        Some(idx) => println!("Found 9 at index {idx}"),
        None      => println!("9 not found"),
    }

    // if let, match only the Some case, ignore None
    if let Some(idx) = linear_search(&data, 5) {
        println!("Found 5 at index {idx}");
    }

    // unwrap_or, provide a default if None
    let idx = linear_search(&data, 42).unwrap_or(usize::MAX);
    println!("Index of 42: {idx}");   // usize::MAX, sentinel without -1

    // map, transform the value inside Some without unwrapping
    let doubled_idx = linear_search(&data, 5).map(|i| i * 2);
    println!("Doubled index of 5: {doubled_idx:?}");   // Some(8)
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Linear Search ===");
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    // &data coerces Vec<i32> → &[i32] automatically
    println!("Search for 9:  {:?}", linear_search(&data, 9));    // Some(5)
    println!("Search for 42: {:?}", linear_search(&data, 42));   // None

    println!("\n=== Double In-Place ===");
    let mut mutable_data = vec![1, 2, 3, 4, 5];
    println!("Before: {mutable_data:?}");

    // &mut mutable_data coerces Vec<i32> → &mut [i32]
    double_in_place(&mut mutable_data);
    println!("After:  {mutable_data:?}");

    println!("\n=== Borrow Rules ===");
    demonstrate_borrow_rules();

    println!("\n=== Slices ===");
    demonstrate_slices();

    println!("\n=== Option<T> Handling ===");
    demonstrate_option_handling();
}

// ── What you should take away ─────────────────────────────────────────────────
//
// 1. &T is a read-only loan. Multiple can coexist. Owner is unaffected.
// 2. &mut T is an exclusive, read-write loan. Only one may exist at a time.
// 3. You CANNOT have &T and &mut T to the same value simultaneously.
//    This rule is what makes data races impossible in safe Rust.
// 4. &[T] is a slice, a borrowed view into a sequence, with no ownership.
//    Prefer it over &Vec<T> in function signatures (more flexible).
// 5. Option<T> replaces -1 / null sentinels. The compiler forces you to
//    handle the None case. No runtime null-reference surprises.
//
// Next: Structs & Enums, building your own types.
