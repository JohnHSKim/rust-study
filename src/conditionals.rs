pub fn run() {
    let age: u8 = 19;
    let check_id: bool = false;

    if age >= 20 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 20{
        println!("Bartender: Sorry, you have to leave");
    }

    let is_of_age = if age >= 20 { true } else { false };
    println!("Is of Age: {}", is_of_age);
}