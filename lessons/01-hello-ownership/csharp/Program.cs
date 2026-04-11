// Lesson 01: Hello, Ownership
// C# equivalent
//
// Goal: compute the sum and maximum of an integer array.
// Notice: we pass arrays around freely. C# arrays are reference types,
// assignment copies the reference, not the data.

using System;

class Program
{
    // Returns (sum, max) as a tuple.
    // The array is passed by reference (C# default for reference types).
    // No copy is made. The original array is accessible here and at the call site.
    static (long Sum, int Max) SumAndMax(int[] numbers)
    {
        if (numbers.Length == 0)
            throw new ArgumentException("Array cannot be empty");

        long sum = 0;
        int max = numbers[0];

        foreach (var n in numbers)
        {
            sum += n;
            if (n > max) max = n;
        }

        return (sum, max);
    }

    // Demonstrates: in C#, assigning an array copies the reference.
    // Both variables point to the same underlying data.
    static void DemonstrateReferenceSemantics()
    {
        int[] original = { 1, 2, 3, 4, 5 };
        int[] alias = original;   // NOT a copy, both point to the same array

        alias[0] = 99;

        Console.WriteLine($"original[0] = {original[0]}");  // 99, modified through alias!
        Console.WriteLine($"alias[0]    = {alias[0]}");      // 99
    }

    // Demonstrates: value types (int, struct) ARE copied.
    static void DemonstrateValueSemantics()
    {
        int a = 10;
        int b = a;   // b is an independent copy

        b = 99;

        Console.WriteLine($"a = {a}");   // 10, unchanged
        Console.WriteLine($"b = {b}");   // 99
    }

    static void Main()
    {
        Console.WriteLine("=== Reference Semantics ===");
        DemonstrateReferenceSemantics();

        Console.WriteLine("\n=== Value Semantics ===");
        DemonstrateValueSemantics();

        Console.WriteLine("\n=== Sum and Max ===");
        int[] data = { 3, 1, 4, 1, 5, 9, 2, 6, 5, 3 };
        var (sum, max) = SumAndMax(data);

        Console.WriteLine($"Data:  [{string.Join(", ", data)}]");
        Console.WriteLine($"Sum:   {sum}");
        Console.WriteLine($"Max:   {max}");

        // data is still fully usable here, passing it to SumAndMax
        // did not transfer or consume it in any way.
        Console.WriteLine($"data[0] after call: {data[0]}");  // 3, untouched
    }
}
