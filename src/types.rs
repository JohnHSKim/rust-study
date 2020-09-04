/*
Primitive Types
Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed lanauges, which means that it must know the types of all variables at compile time.
// However, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 3.14;

    // Add explicit type
    let z: i64 = 314;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}