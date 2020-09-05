pub fn run() {
    let mut count = 0;

    loop {
        count +=1 ;
        println!("Counter: {}", count);

        if count == 20 {
            break;
        }
    }
    
    count = 0;
    while count <= 100 {
        if  count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("Number: {}", count);
        }

        count += 1;
    }

    for count in 0..100 {
        if  count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("Number: {}", count);
        }
    }
}