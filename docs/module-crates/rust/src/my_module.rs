use crate::bots::hello_bot::BOT_NAME;

pub fn greet() {
    println!("Hi I am hello bot")
    println!("Hi! I am {}", BOT_NAME);
}

pub struct Person {
    pub name: String,
    age: i32,
}

impl Person {
    pub fn new(name: &str, age: i32) -> Self {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    pub fn get_older(&mut self, year: i32) {
        self.age += year;
    }
}
