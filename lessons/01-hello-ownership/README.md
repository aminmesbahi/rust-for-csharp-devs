# Lesson 01, Hello, Ownership

**Rust concept:** Ownership, Move semantics, Copy types  
**DSA topic:** Variables, primitives, simple array operations

---

## The Problem

Write a function that takes an array of integers and returns the sum and the maximum value.

Simple. You've done this in C# a hundred times. In Rust, even this exposes something fundamental.

---

## Background: How C# Thinks About Values

In C#, values fall into two categories:

- **Value types** (`int`, `double`, `struct`), live on the stack, copied on assignment
- **Reference types** (`class`, `string`, `array`), live on the heap, assignment copies the reference

```csharp
int a = 5;
int b = a;     // b is a copy, a and b are independent
b = 10;
Console.WriteLine(a);  // 5, unchanged

var list1 = new List<int> { 1, 2, 3 };
var list2 = list1;     // list2 is NOT a copy, both point to the same list
list2.Add(4);
Console.WriteLine(list1.Count);  // 4, list1 was also modified
```

The GC tracks references and frees the heap object when nothing points to it anymore. You never think about *when* memory is freed.

---

## How Rust Thinks About Values: Ownership

Rust has one rule that governs everything:

> **Each value has exactly one owner. When the owner goes out of scope, the value is dropped (freed).**

There are no shared references by default. No GC needed.

```rust
let s1 = String::from("hello");
let s2 = s1;     // ownership MOVES to s2, s1 is no longer valid

println!("{}", s1);  // COMPILE ERROR: value moved
println!("{}", s2);  // fine
```

This is called a **move**. It's not a copy, the original variable is invalidated. Rust knows at compile time that `s1` no longer owns anything.

### Why? No double-free, ever.

If two variables owned the same heap data, both would try to free it when they go out of scope, a double-free bug. Rust prevents this structurally.

---

## Copy Types

Not everything is moved. Small, stack-only types implement the `Copy` trait and are duplicated instead of moved:

```rust
let a: i32 = 5;
let b = a;           // a is copied, both a and b are valid
println!("{}", a);   // fine
println!("{}", b);   // fine
```

Types that implement `Copy`: `i32`, `f64`, `bool`, `char`, tuples of Copy types, fixed-size arrays of Copy types.

Types that do **not** implement `Copy`: `String`, `Vec<T>`, `Box<T>`, anything that owns heap data.

---

## The DSA Example

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
| Memory model | GC tracks references | Single owner, deterministic drop |
| Assignment (value types) | Copy | Copy (for `Copy` types) |
| Assignment (heap types) | Copies the reference | Moves ownership |
| Null | Possible (reference types) | Doesn't exist |
| When is memory freed? | GC decides | End of owner's scope |

### Things that will surprise you coming from C#

1. **Assignment is not always a copy.** `let s2 = s1` for a `String` invalidates `s1`.
2. **Functions consume values.** Passing a `String` to a function moves it, you can't use it after.
3. **Dropping is explicit and deterministic.** No finalizers, no `~Destructor` timing surprises.

---

## Next Lesson

[Lesson 02, Borrowing & References](../02-borrowing-references/README.md): Moving ownership everywhere is too restrictive. Rust has a way to *lend* values without transferring ownership, and that's where the borrow checker enters.
