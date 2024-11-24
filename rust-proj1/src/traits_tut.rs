pub trait Summary {
    // Default implementation
    fn summarize(&self) -> String {
        return String::from("Hii there");
    }
}

pub trait Fix {
    // Default implementation
    fn fix(&self) -> String {
        return String::from("Hii there from fix ");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("Name is {}, age is {}", self.name, self.age);
    }
}

impl Fix for User {}

pub fn trt() {
    let user = User {
        name: String::from("Harsh"),
        age: 67,
    };
    notify(&user);
    node(&user);
    println!("Call the function {}", user.summarize());
}

// Anything can be implementated as summary trait can be use the traits function
pub fn notify(u: &impl Summary) {
    println!("{}", u.summarize())
}

// Generic traits
pub fn node<T: Summary + Fix>(u: &T) {
    println!("{}", u.fix())
}
