pub fn run() {
    let is_succeed = greeting("Hi", "Hoon");
    if is_succeed == 0 {
        println!("greeting success");
    } else {
        println!("There is a problem");
    }

    // Closure
    let n3: i32 = 10;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_num(3, 4));
}

fn greeting(greet: &str, name: &str) -> i8 {
    println!("{} {}, nice to meet you!", greet, name);
    0
}

