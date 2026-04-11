# Introduction: .NET and Rust, A Big-Picture Comparison

Before writing a single line of Rust, it's worth understanding *why* Rust feels the way it does, especially coming from C#. They solve some of the same problems very differently, and knowing the philosophy behind each helps you stop fighting Rust and start using it.

---

## The Core Difference: Who Manages Memory?

This is the most fundamental difference between the two languages.

| | C# / .NET | Rust |
|--|-----------|------|
| Memory management | Garbage Collector (GC) | Ownership system (compile-time) |
| When memory is freed | Non-deterministic (GC decides) | Deterministic (end of owner's scope) |
| Runtime overhead | GC pauses, heap pressure | Zero runtime overhead |
| You think about memory | Rarely | Always (but the compiler guides you) |

In C#, you write code and the CLR quietly manages memory behind the scenes. Objects live on the heap, the GC tracks references, and memory is reclaimed when nothing points to an object anymore.

In Rust, there is no GC. Memory is managed through a set of compile-time rules called **ownership**. The compiler tracks exactly who owns each value, when ownership transfers, and when memory should be freed, all without any runtime machinery.

This sounds restrictive. In practice, it means Rust programs have:
- No GC pauses
- No null pointer exceptions (nulls don't exist)
- No data races (by design)
- Fully predictable memory behavior

---

## Type Systems: Similarities and Differences

Both languages have strong, static type systems. But there are meaningful differences in expressiveness and safety.

### Nullability

```csharp
// C#: null is always possible unless you opt in to nullable reference types
string name = null;              // compiles (with warning in modern C#)
string? nullable = null;         // explicit nullable
string nonNull = "hello";        // still nullable at runtime in older code
```

```rust
// Rust: null does not exist. Absence is expressed with Option<T>
let name: Option<String> = None;
let name: Option<String> = Some(String::from("hello"));

// You cannot use the value without handling the None case
```

In Rust, `Option<T>` replaces null entirely. There is no way to have a value that "might be null" without the type system knowing about it.

### Error Handling

```csharp
// C#: exceptions propagate implicitly, can be caught anywhere
try {
    var result = DoSomething();
} catch (SomeException ex) {
    // handle
}
```

```rust
// Rust: errors are values, propagation is explicit
let result: Result<i32, MyError> = do_something();

match result {
    Ok(value) => println!("Got: {}", value),
    Err(e)    => println!("Failed: {}", e),
}

// Or propagate with the ? operator
let value = do_something()?;  // returns Err early if it fails
```

Rust functions that can fail return `Result<T, E>`. Errors are explicit in the function signature. There are no invisible exception paths.

### Enums

C# enums are just named integers. Rust enums are **algebraic data types**, each variant can carry data.

```csharp
// C#: enum is just a name for a number
enum Shape { Circle, Rectangle, Triangle }
// To attach data, you need a separate class hierarchy
```

```rust
// Rust: each variant can carry different data
enum Shape {
    Circle(f64),                    // radius
    Rectangle(f64, f64),            // width, height
    Triangle(f64, f64, f64),        // three sides
}
```

This makes Rust enums closer to F# discriminated unions or Kotlin sealed classes.

---

## Traits vs Interfaces

Rust's `trait` is the counterpart to C#'s `interface`, but with important differences.

```csharp
// C# interface
interface IComparable<T> {
    int CompareTo(T other);
}

class MyType : IComparable<MyType> {
    public int CompareTo(MyType other) { ... }
}
```

```rust
// Rust trait
trait Comparable {
    fn compare_to(&self, other: &Self) -> i32;
}

impl Comparable for MyType {
    fn compare_to(&self, other: &Self) -> i32 { ... }
}
```

Key differences:
- Rust traits can be implemented for **any type**, including types from other crates (with some restrictions)
- Rust has **default implementations** in traits (so does C# 8+, but it's less common in practice)
- Rust has **no inheritance**, composition through traits replaces class hierarchies entirely
- Rust's `dyn Trait` (dynamic dispatch) is opt-in; the default is static dispatch (monomorphization), which is more like C++ templates than C# generics

---

## The Borrow Checker: What C# Never Needed to Tell You

This is the most Rust-specific concept and the one that causes the most friction for newcomers.

In C#, you can freely share mutable references:

```csharp
var list = new List<int> { 1, 2, 3 };
var ref1 = list;
var ref2 = list;

ref1.Add(4);   // fine
ref2.Add(5);   // also fine, both point to the same list
```

This is safe in C# because the GC ensures the list isn't freed while references exist. But in multithreaded code, this is where data races happen.

Rust's borrow checker enforces two rules at compile time:

1. **You can have any number of immutable references** (`&T`) to a value, OR
2. **Exactly one mutable reference** (`&mut T`), never both at the same time

```rust
let mut v = vec![1, 2, 3];

let r1 = &v;      // immutable borrow
let r2 = &v;      // another immutable borrow, fine
// let r3 = &mut v; // ERROR: cannot borrow as mutable while immutable borrows exist

println!("{:?} {:?}", r1, r2);   // r1, r2 used here
// after this point, r1 and r2 are no longer active

let r3 = &mut v;  // now this is fine
r3.push(4);
```

This rule makes data races *impossible* at compile time. No runtime checks. No locks needed for single-threaded code. The compiler rejects unsafe patterns before your program ever runs.

---

## Concurrency

| | C# | Rust |
|--|-----|------|
| Threads | `Thread`, `Task`, `async/await` | `std::thread`, `async/await` (with tokio/async-std) |
| Shared state | `lock`, `Monitor`, `Mutex<T>` | `Arc<Mutex<T>>` |
| Data races | Possible at runtime | Impossible (prevented at compile time) |
| Send/Sync | Implicit | Explicit traits (`Send`, `Sync`) |

In Rust, whether a type can be sent between threads (`Send`) or shared between threads (`Sync`) is encoded in the type system. You cannot accidentally share a non-thread-safe type across threads, the compiler rejects it.

---

## Performance Model

| | C# | Rust |
|--|-----|------|
| Runtime | CLR (JIT compiled) | Native binary (AOT compiled) |
| GC overhead | Yes, pauses, heap pressure | None |
| Abstractions | Sometimes have runtime cost | Zero-cost abstractions |
| Startup time | Slow (JIT warmup) | Instant |
| Predictability | Variable (GC behavior) | Fully predictable |

"Zero-cost abstractions" is a core Rust promise: if you use iterators, closures, or generics, the compiled output is equivalent to what you'd write by hand. There's no boxing, no virtual dispatch unless you explicitly ask for it.

---

## Ecosystem Comparison

| Area | C# / .NET | Rust |
|------|-----------|------|
| Package manager | NuGet | Cargo |
| Build tool | MSBuild / dotnet CLI | Cargo (unified) |
| Web | ASP.NET Core | Axum, Actix, Rocket |
| Async runtime | Built-in (CLR) | External (tokio, async-std) |
| Testing | xUnit, NUnit, MSTest | Built-in (`#[test]`) |
| ORM | Entity Framework | Diesel, SQLx |
| Maturity | Very mature | Growing rapidly |

Cargo is notably more unified than the .NET ecosystem, one tool handles building, testing, documentation, publishing, and dependency management.

---

## What Rust Is Not

- **Not a replacement for C# in all contexts.** For business applications, web APIs, and rapid development, C# and .NET are often the better choice.
- **Not easy to learn quickly.** The borrow checker has a learning curve. Budget real time for it.
- **Not object-oriented.** Rust has structs and traits, not classes and inheritance. Thinking in terms of data and behavior separately takes adjustment.

---

## What to Expect in This Course

Each lesson introduces a Rust concept through a DSA problem you already know how to solve in C#. The goal is not to memorize Rust syntax, it's to understand *why* Rust works the way it does, and when the tradeoffs make sense.

Start with [Lesson 01, Hello, Ownership](../../lessons/01-hello-ownership/README.md).
