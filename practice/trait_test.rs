trait Hello {
    fn hello(&self) -> String;
}

struct Person {
    name: String,
}

impl Hello for Person {
    fn hello(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

fn main() {
    let person = Person { name: String::from("Alice") };
    println!("{}", person.hello());
}
