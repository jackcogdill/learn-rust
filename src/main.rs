use std::fmt;

/// A human being is represented here
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.age)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let person = Person { name, age };
    println!("display: {}", person);
    println!("debug: {:?}", person);
    println!("pretty debug: {:#?}", person);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    println!("{:?}", v);
}
