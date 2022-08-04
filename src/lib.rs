use std::collections::VecDeque;

use CalcType::{Add, Divide, Multiply, Power, Print, Subtract, Val};
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
/// enum CalcType is a type containing operations used in the calculator.
pub enum CalcType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Print,
    Val(f64),
}

pub fn str_to_calc_type(string: &str) -> Option<CalcType> {
    let as_int = string.parse::<f64>();
    let result = match as_int {
        Ok(x) => Some(Val(x)),
        Err(_) => None,
    };

    if result.is_some() {
        return result;
    }

    match string {
        "+" => Some(Add),
        "-" => Some(Subtract),
        "*" => Some(Multiply),
        "/" => Some(Divide),
        "^" => Some(Power),
        "p" => Some(Print),

        _ => None,
    }
}

pub fn eval(input: &str, stack: &mut VecDeque<f64>) {
    //// Create a mutable copy of the inputted stack
    //// let mut stack = stack_in.clone();

    // Split the input into tokens.
    let toks = input.split(" ").collect::<Vec<&str>>();
    let mut ops: VecDeque<CalcType> = VecDeque::new();

    for tok in &toks {
        let x: CalcType = str_to_calc_type(tok).unwrap();

        match x {
            Add | Divide | Multiply | Power | Subtract | Print => ops.push_back(x),

            Val(x_) => stack.push_back(x_),
        }
    }

    for op in &ops {
        match op {
            Add => {
                let y = &stack.pop_back().unwrap_or(0.0);
                let x = &stack.pop_back().unwrap_or(0.0);
                &stack.push_back(x + y)
            }
            Subtract => {
                let y = &stack.pop_back().unwrap_or(0.0);
                let x = &stack.pop_back().unwrap_or(0.0);
                &stack.push_back(x - y)
            }
            Multiply => {
                let y = &stack.pop_back().unwrap_or(0.0);
                let x = &stack.pop_back().unwrap_or(0.0);
                &stack.push_back(x * y)
            }
            Divide => {
                let y = &stack.pop_back().unwrap_or(0.0);
                let x = &stack.pop_back().unwrap_or(0.0);
                &stack.push_back(x / y)
            }
            Power => {
                let y = &stack.pop_back().unwrap_or(0.0);
                let x = &stack.pop_back().unwrap_or(0.0);

                let result = x.powf(*y);
                &stack.push_back(result)
            }
            Print => &{ println!("{:#?}", stack.iter().last()) },
            Val(_) => panic!("Unexpected value in the operator stack!"),
        };
    }

    println!("{}", stack.iter().last().unwrap_or(&0.0));
}
