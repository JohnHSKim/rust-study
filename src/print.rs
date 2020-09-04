pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "John", "South Korea");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "John", "South Korea", "code");

    // Named Arguments
    println!("{name} likes to eat {fruit}", name="Ted", fruit="apple");

    // Placeholder traits
    println!("Binary: {0:b} Hex: {0:x} Octal: {number:o}", 10, number=10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hi"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}