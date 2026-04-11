# Rust for C# Developers

A practical, example-driven course for C# developers learning Rust, through the lens of **data structures and algorithms**.

Each lesson pairs a familiar DSA concept with the Rust features that make it interesting (or tricky). You'll always see the C# equivalent side-by-side.

---

## Why DSA?

Because ownership, borrowing, and lifetimes are easiest to understand when you're building something you already know. A linked list you've written ten times in C# becomes a completely different conversation in Rust, and that contrast is the lesson.

---

## Structure

```
rust-for-csharp-devs/
├── docs/
│   ├── en/
│       └── intro.md          ← Start here: big-picture 
└── lessons/
    ├── 01-hello-ownership/
    │   ├── README.md
    │   ├── rust/             ← Cargo project
    │   └── csharp/           ← Equivalent C# code
    ├── 02-borrowing-references/
    ...
```

Each lesson folder contains:
- `README.md`, concept explanation, C# vs Rust comparison, key takeaways
- `rust/`, a standalone Cargo project you can `cargo run`
- `csharp/`, equivalent C# for direct comparison

---

## Lessons

| # | Title | Rust Concept | DSA Topic |
|---|-------|-------------|-----------|
| 01 | [Hello, Ownership](lessons/01-hello-ownership/) | Ownership, Move, Copy | Variables & Primitives |
| 02 | [Borrowing & References](lessons/02-borrowing-references/) | `&T`, `&mut T`, Borrow Checker | Array Traversal & Linear Search |
| 03 | Structs & Enums | `struct`, `enum` with data | Stack with `Vec<T>` |
| 04 | Pattern Matching | `match`, `Option<T>` | Binary Search |
| 05 | Generics & Traits | `trait`, `impl<T>` | Generic Stack |
| 06 | Lifetime Annotations | `'a` | Slice-based Queue |
| 07 | Linked List I | `Box<T>` | Singly Linked List |
| 08 | Linked List II | `Rc<T>`, `RefCell<T>` | Doubly Linked List |
| 09 | HashMap & Hashing | `HashMap`, hash traits | Hash Table |
| 10 | Iterators & Closures | `Iterator` trait, lazy eval | Sliding Window |
| 11 | Error Handling | `Result<T,E>`, `?` operator | Validated Parsing |
| 12 | Recursion & Stack Safety | tail calls, stack limits | Merge Sort |
| 13 | Trees I | recursive enum, `Box` | Binary Search Tree |
| 14 | Trees II | `dyn Trait`, trait objects | Tree Traversals |
| 15 | Graphs | adjacency list | BFS / DFS |
| 16 | Sorting | `Ord`, `PartialOrd` | Quick Sort |
| 17 | Concurrency I | `Arc<Mutex<T>>` | Concurrent Queue |
| 18 | Concurrency II | `async/await`, tokio | Async BFS |
| 19 | Smart Pointers | `Box/Rc/Arc/Weak` | Memory-safe Graph |
| 20 | Final Project | everything combined | LRU Cache |

---

## Prerequisites

- Comfortable with C# (any version)
- Rust installed: [rustup.rs](https://rustup.rs)
- Basic familiarity with DSA concepts (arrays, linked lists, trees)

## Running a Lesson

```bash
cd lessons/01-hello-ownership/rust
cargo run
```

---