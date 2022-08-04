use dc_ock::{
    str_to_calc_type, CalcType,
    CalcType::{Add, Divide, Multiply, Power, Print, Subtract, Val},
};
use std::{
    collections::VecDeque,
    io::{self, stdin, stdout},
    process::exit,
};

#[allow(unused_assignments)]
fn main() {
    loop {
        print!(">> ");
        io::Write::flush(&mut stdout()).expect("Flush error");
        let mut stk: VecDeque<f64> = VecDeque::new();
        let mut in_str = String::new();

        let mut clos_mutate = |x: &str| eval(x.strip_suffix("\n").unwrap_or(&x), &mut stk);

        match stdin().read_line(&mut in_str) {
            Ok(0) => {
                exit(0);
            }
            Ok(_) => clos_mutate(&in_str),
            Err(e) => println!("Couldn't parse input: {}", e),
        };
    }
}

fn eval(input: &str, stack: &mut VecDeque<f64>) {
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
                let y = stack.pop_back().unwrap_or(0.0);
                let x = stack.pop_back().unwrap_or(0.0);

                let result = x.powf(y);
                &stack.push_back(result)
            }
            Print => &{ println!("{:#?}", stack.iter().last()) },
            Val(_) => panic!("Unexpected value in the operator stack!"),
        };
    }

    println!("{}", stack.iter().last().unwrap_or(&0.0));
}
