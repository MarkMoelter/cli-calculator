mod calculator;
use std::io;
use std::str::FromStr;

fn main() {
    let mut num_1_string = String::new();
    let mut num_2_string = String::new();
    let mut oper = String::new();
    println!("Cli Calculator");
    println!("Please input your first number:");

    io::stdin()
        .read_line(&mut num_1_string)
        .expect("Failed to read line");

    println!("Please input your second number:");

    io::stdin()
        .read_line(&mut num_2_string)
        .expect("Failed to read line");

    println!("Please input your operator: (+, -, *, /)");

    io::stdin()
        .read_line(&mut oper)
        .expect("Failed to read line");

    let calc = calculator::calculate(
        parse_number(&num_1_string),
        parse_number(&num_2_string),
        parse_operator(&oper),
    );

    println!("Your calculation: {}", calc);
}

fn parse_number(input: &String) -> f32 {
    input.trim().parse::<f32>().unwrap()
}

fn parse_operator(input: &String) -> calculator::Operator {
    calculator::Operator::from_str(input.trim()).unwrap()
}
