use std::io;

fn main() {
    println!("Select an operator: ");

    println!("\n\tAddition: + ");
    println!("\tSubtraction: -");
    println!("\tMultiplication: *");
    println!("\tDivision: /");

    println!("\nEnter an operator: ");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read line");

    println!("\nEnter a number: ");
    let mut num1_str = String::new();
    io::stdin().read_line(&mut num1_str).expect("Failed to read line");
    let num1: i32 = num1_str.trim().parse().expect("Invalid input");

    println!("\nEnter a second number: ");
    let mut num2_str = String::new();
    io::stdin().read_line(&mut num2_str).expect("Failed to read line");
    let num2: i32 = num2_str.trim().parse().expect("Invalid input");

    match operator.trim() {
        "+" => println!("\nAnswer: \n{}", add(num1, num2)),
        "-" => println!("\nAnswer: \n{}", subtract(num1, num2)),
        "*" => println!("\nAnswer: \n{}", multiply(num1, num2)),
        "/" => println!("\nAnswer: \n{}", divide(num1, num2)),
        _ => println!("\nInvalid operator entered."),

    };
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b
}

fn divide(a: i32, b: i32) -> i32 {
    return a / b
}