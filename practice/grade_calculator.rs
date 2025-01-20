// match
use std::io;

fn grade_calculator(mark: &str) -> Option<&str> {
    match mark.to_uppercase().as_str() {
        "A" => Some("Excellent!"),
        "B" => Some("Good job!"),
        "C" => Some("You passed."),
        "D" => Some("Need improvement."),
        "F" => Some("Failed."),
        _ => None,
    }
}

fn main() -> () {
    let mut input = String::new();
    println!("Enter your grade (A-F): ");

    io::stdin().read_line(&mut input)
               .expect("Failed to read input");

    let grade = input.trim();

    match grade_calculator(grade) {
        Some(message) => println!("{}", message),
        None => println!("Invalid grade."),
    }
}