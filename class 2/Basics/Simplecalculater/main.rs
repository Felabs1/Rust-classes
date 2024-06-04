
//a command-line calculator that can perform basic arithmetic operations like addition, subtraction, multiplication, and division.
use std::io;

fn main() {
    println!("Simple Calculator");
    
    loop {
        let mut input = String::new();
        
        println!("Enter an operation (e.g., 3 + 4) or 'quit' to exit:");
        
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if input.trim() == "quit" {
            break;
        }
        
        let result = calculate(&input);
        
        match result {
            Ok(value) => println!("Result: {}", value),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn calculate(input: &str) -> Result<f64, &str> {
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    
    if tokens.len() != 3 {
        return Err("Invalid input");
    }
    
    let left_operand: f64 = tokens[0].parse().map_err(|_| "Invalid number")?;
    let right_operand: f64 = tokens[2].parse().map_err(|_| "Invalid number")?;
    let operator = tokens[1];
    
    match operator {
        "+" => Ok(left_operand + right_operand),
        "-" => Ok(left_operand - right_operand),
        "*" => Ok(left_operand * right_operand),
        "/" => Ok(left_operand / right_operand),
        _ => Err("Unknown operator"),
    }
}
