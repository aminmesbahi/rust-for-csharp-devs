// Lesson 02: Borrowing & References
// C# equivalent
//
// Goal:
//   1. Linear search, find the index of a target in an unsorted array
//   2. Double every element in-place
//
// C# passes reference types by reference automatically.
// There is no explicit borrow system.

using System;

class Program
{
    // ── Linear Search ────────────────────────────────────────────────────────
    //
    // Returns the index of the first occurrence of `target`, or -1.
    // `numbers` is a reference type, no copy is made when passed.
    // The array is read-only by convention; C# doesn't enforce this at compile time.
    static int LinearSearch(int[] numbers, int target)
    {
        for (int i = 0; i < numbers.Length; i++)
        {
            if (numbers[i] == target)
                return i;
        }
        return -1;
    }

    // ReadOnlySpan<T> is C#'s closest analog to Rust's &[T]:
    // a read-only, non-owning view into a contiguous sequence.
    // Available since C# 7.2 / .NET Core 2.1.
    static int LinearSearchSpan(ReadOnlySpan<int> numbers, int target)
    {
        for (int i = 0; i < numbers.Length; i++)
        {
            if (numbers[i] == target)
                return i;
        }
        return -1;
    }

    // ── In-place Double ──────────────────────────────────────────────────────
    //
    // Doubles every element in the array in-place.
    // In C#, `int[]` is a reference type, mutations here affect the caller's array.
    // There is no "mutable reference" concept; all reference-type parameters
    // are implicitly mutable (you can modify what they point to).
    static void DoubleInPlace(int[] numbers)
    {
        for (int i = 0; i < numbers.Length; i++)
        {
            numbers[i] *= 2;
        }
    }

    // Span<T> is the mutable version, like Rust's &mut [T].
    static void DoubleInPlaceSpan(Span<int> numbers)
    {
        for (int i = 0; i < numbers.Length; i++)
        {
            numbers[i] *= 2;
        }
    }

    // ── Aliasing demonstration ───────────────────────────────────────────────
    //
    // In C#, you can freely have multiple "references" to the same array.
    // Reading through one while writing through another is allowed.
    // This is safe in single-threaded code; in multi-threaded code,
    // it's a source of data races.
    static void DemonstrateAliasing()
    {
        int[] data = { 1, 2, 3, 4, 5 };

        int[] reader = data;   // both point to the same array
        int[] writer = data;

        writer[0] = 99;                          // mutate through writer
        Console.WriteLine($"reader[0] = {reader[0]}");  // 99, reads the mutation

        // In Rust, this pattern is rejected at compile time:
        // you cannot have a &mut reference while another reference exists.
    }

    // ── Main ─────────────────────────────────────────────────────────────────

    static void Main()
    {
        int[] data = { 3, 1, 4, 1, 5, 9, 2, 6, 5, 3 };

        Console.WriteLine("=== Linear Search ===");
        int idx = LinearSearch(data, 9);
        Console.WriteLine($"Found 9 at index: {idx}");   // 5

        int notFound = LinearSearch(data, 42);
        Console.WriteLine($"Found 42 at index: {notFound}");  // -1

        // Using ReadOnlySpan, shows C# moving toward Rust-like semantics
        int idxSpan = LinearSearchSpan(data, 5);
        Console.WriteLine($"Found 5 at index (Span): {idxSpan}");  // 4

        Console.WriteLine("\n=== Double In-Place ===");
        Console.WriteLine($"Before: [{string.Join(", ", data)}]");
        DoubleInPlace(data);
        Console.WriteLine($"After:  [{string.Join(", ", data)}]");

        Console.WriteLine("\n=== Aliasing ===");
        DemonstrateAliasing();
    }
}
