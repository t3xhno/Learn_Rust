// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure

pub fn run () {
    let hello = "Hello"; // str
    let mut hello1 = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    hello1.push('W'); // Push char
    hello1.push_str("orld!"); // Push string

    // Capacity in bytes
    println!("Capacity: {}", hello1.capacity());
    println!("Is empty: {}", hello1.is_empty());

    // Contains
    println!("Contains 'World': {}", hello1.contains("World"));

    // Replace
    println!("Replace: {}", hello1.replace("World", "There"));

    // Loop through strings by whitespace
    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}