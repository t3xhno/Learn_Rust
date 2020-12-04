// Create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct ColorTuple (u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new (first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get Full name
    fn full_name (&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name (&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    
    // Name to tuple
    fn to_tuple (self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run () {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    let mut ct = ColorTuple(255, 0, 0);

    ct.0 = 200;
    println!("Color Tuple: {} {} {}", ct.0, ct.1, ct.2);

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let p = Person::new("Marko", "Lazic");
    let mut p2 = Person::new("Marko", "Lazic");
    p2.set_last_name("Lol");
    println!("Person: {}", p.full_name());
    println!("Person 2: {}", p2.full_name());
    println!("Tuple name: {:?}", p2.to_tuple());
}