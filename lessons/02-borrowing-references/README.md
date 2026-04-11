# Lesson 02, Borrowing & References

**Rust concept:** `&T` (immutable borrow), `&mut T` (mutable borrow), the Borrow Checker  
**DSA topic:** Array traversal, Linear search, In-place operations

---

## The Problem

1. Find the index of a target value in an unsorted array (linear search)
2. Double every element in an array in-place

Both are trivial in C#. In Rust, they illustrate exactly what the borrow checker is enforcing and why.

---

## Why Borrowing Exists

In Lesson 01, we saw that assigning or passing a `String` or `Vec` *moves* it. That's too restrictive for everyday use. You can't pass a vector to a function and have it disappear.

**Borrowing** is Rust's solution: you can *lend* a value without transferring ownership.

```rust
fn print_vec(v: &Vec<i32>) {      // borrows v, does not take ownership
    println!("{v:?}");
}

let data = vec![1, 2, 3];
print_vec(&data);                  // lend data to the function
println!("{data:?}");              // data still valid, we only lent it
```

The `&` before the type is a **reference**. The function gets read-only access to the data; ownership stays with `data`.

---

## Two Kinds of Borrows

### Immutable borrow, `&T`

- Read-only access
- Multiple immutable borrows can coexist
- The owner cannot be mutated while immutable borrows are active

```rust
let data = vec![1, 2, 3];

let r1 = &data;   // borrow 1
let r2 = &data;   // borrow 2, fine, both are read-only

println!("{r1:?} {r2:?}");   // both used here, borrows end after this line
```

### Mutable borrow, `&mut T`

- Read-write access
- **At most one** mutable borrow at a time
- No immutable borrows may exist simultaneously

```rust
let mut data = vec![1, 2, 3];

let r = &mut data;   // mutable borrow
r.push(4);           // allowed, we have exclusive access
                     // no other borrow of data can exist here

println!("{data:?}"); // [1, 2, 3, 4]
```

---

## The Borrow Checker Rules (simplified)

At any point in time, for a given value, you may have **either**:

- Any number of `&T` (immutable references), or
- Exactly one `&mut T` (mutable reference)

**Never both at the same time.**

This rule makes data races *structurally impossible* in single-threaded code, and is the foundation for Rust's thread-safety guarantees.

### How C# compares

C# does not have these restrictions for single-threaded code:

```csharp
var list = new List<int> { 1, 2, 3 };

// Fine in C#, both references coexist, one mutates while the other "reads"
var reader = list;
list.Add(4);       // mutate through original
Console.WriteLine(reader.Count);  // reads through alias, 4
```

This is safe in C# because there's one object on the heap and the GC manages it. But in multi-threaded code, this pattern leads to race conditions. Rust eliminates the possibility at compile time.

---

## Slices

A **slice** is a borrowed view into a contiguous sequence, no ownership, no allocation.

```rust
let data = vec![1, 2, 3, 4, 5];
let slice: &[i32] = &data[1..4];   // [2, 3, 4], a view, not a copy
```

Functions that accept `&[T]` instead of `&Vec<T>` are more flexible, they work with vectors, arrays, and any other contiguous sequence.

```rust
fn sum(numbers: &[i32]) -> i64 { ... }   // works with Vec, array, slice
```

This is idiomatic Rust. Prefer `&[T]` over `&Vec<T>` in function signatures.

---

## The DSA Examples

### C# version

```csharp
// See csharp/Program.cs
```

### Rust version

```rust
// See rust/src/main.rs
```

---

## Key Takeaways

| | C# | Rust |
|--|-----|------|
| Read-only reference | Default (no keyword) | `&T` |
| Mutable reference | `ref` in some contexts | `&mut T` |
| Multiple mutable references | Allowed | Compile error |
| Simultaneous read + write | Allowed | Compile error |
| Slice type | `Span<T>` / `ReadOnlySpan<T>` | `&[T]` / `&mut [T]` |
| Borrow enforcement | Runtime (data races possible) | Compile time (impossible) |

### Things that will surprise you coming from C#

1. **You must declare mutability explicitly.** `let x = 5` is immutable. `let mut x = 5` is mutable.
2. **A `&mut` reference is exclusive.** No other reference to the same data can exist while it's alive.
3. **The borrow checker knows *where* borrows end.** It's not just scope, it's the last *use* (Non-Lexical Lifetimes).
4. **`&Vec<T>` vs `&[T]`**, prefer slices in function signatures. `&Vec<T>` coerces to `&[T]` automatically.

---

## Next Lesson

[Lesson 03, Structs & Enums](../03-structs-enums/README.md): Building custom types. `struct` replaces `class` (without inheritance), and `enum` becomes powerful enough to replace entire class hierarchies.
