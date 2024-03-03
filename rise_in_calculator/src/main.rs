use std::io;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    println!("Enter an operation (Add, Subtract, Multiply, Divide):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let operation_type = match input.trim() {
        "Add" => Operation::Add,
        "Subtract" => Operation::Subtract,
        "Multiply" => Operation::Multiply,
        "Divide" => Operation::Divide,
        _ => {
            println!("Invalid operation chosen, defaulting to Add.");
            Operation::Add
        }
    };

    let operation_result = calculate(operation_type);
    println!("This is the operation chosen result: {}", operation_result);
}

fn calculate(operation_type: Operation) -> f64 {
    match operation_type {
        Operation::Add => 1.0 + 1.0,
        Operation::Subtract => 1.0 - 1.0,
        Operation::Multiply => 1.0 * 1.0,
        Operation::Divide => 1.0 / 1.0,
    }
}
