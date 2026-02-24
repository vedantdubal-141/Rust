fn main(){
    // Define a variable named 'life' and assign it the value 42.
    let life = 42;

    // Print "hello" to the terminal using the println! macro.
    // Note: This example shows a basic usage without variables in the string.
    println!("Hello"); // Output: Hello

    // Using formatted printing with a single variable (life).
    // The {:?} syntax is used for debugging/printing debug info of variables.
    // Here, it prints the value of 'life' in a debug format.
    println!("{:?}", life); // Output: 42

    // Using formatted printing with multiple variables and custom formatting.
    // The {:?} can be replaced with other formats like {:#?} for pretty-printing.
    // This demonstrates how to print 'life' twice in sequence.
    println!("Life is {}, and again it's {}", life, life); // Output: Life is 42, and again it's 42

    // Using a more descriptive string with the variable.
    // The {:?} ensures that the value of 'life' is printed correctly.
    println!("The meaning of life is {}.", life); // Output: The meaning of life is 42.

    /* Alternative Syntax (Kotlin-like) for clarity:
       - Using named placeholders like {life:?} to explicitly reference variables.
    */
    // Debug print with explicit variable name in the format string.
    // This is useful when you want to be more descriptive about what's being printed.
    println!("{life:?}", life); // Output: 42

    // Using a simple placeholder without debug formatting (e.g., for user-friendly output).
    // Note: This will not work directly with Rust’s `println!` macro in the same way as {:?}.
    // In practice, you'd use this more like in Kotlin's println("{}", life) if using a different language.
    println!("{life}", life); // Compile-time error (Kotlin-like syntax is not valid in Rust)

}