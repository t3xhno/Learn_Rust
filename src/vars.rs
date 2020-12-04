// Variables hold primitive data or reference to data
// Vatiables are immutable by default
// Rust is a block-scoped language

pub fn run () {
    let name = "Marko";

    let mut age = 27;
    println!("My name is {}, and I am {} years old.", name, age);

    age = 28;
    println!("My name is {}, and I am {} years old.", name, age);

    // Define constant
    const ID: u32 = 001;
    println!("ID: {}.", ID);

    // Assign multiple variables at once
    let (my_name, my_age) = ("Pera", 25);
    println!("{} is {}.", my_name, my_age);
}