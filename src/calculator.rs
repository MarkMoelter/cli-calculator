use std::str::FromStr;

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(()),
        }
    }
}

pub fn calculate(num_1: f32, num_2: f32, operation: Operator) -> f32 {
    match operation {
        Operator::Add => num_1 + num_2,
        Operator::Subtract => num_1 - num_2,
        Operator::Multiply => num_1 * num_2,
        Operator::Divide => num_1 / num_2,
    }
}
