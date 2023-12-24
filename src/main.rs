use std::fmt;

/// A human being is represented here
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}: {1}", self.name, self.age)
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let person = Person { name, age };
    println!("display: {}", person);
    println!("debug: {:?}", person);
    println!("pretty debug: {:#?}", person);
}
