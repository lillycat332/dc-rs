use std::{collections::VecDeque, fmt};
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
#[derive(Debug, Clone)]
pub struct EvaluationError {
    pub message: String,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed evaluation with reason: {}", self.message)
    }
}

/// str_to_calc_type converts a string to an optional `CalcType`.
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

/// `eval` takes a `&str` and a `&mut VecDeque<f64>`, evaluates the expression,
/// and prints the result, pushing the results onto the stack of type
/// VecDeque<f64> provided as a parameter.
///
/// It does not return an error, but will instead silently fail if the expression is invalid.
/// Additionally, it mutates the stack parameter, rather than returning a new one.
/// For this reason, eval is less safe, and safe_eval is recommended over it.
///
pub fn eval(input: &str, stack: &mut VecDeque<f64>) {
    //// Create a mutable copy of the inputted stack
    //// let mut stack = stack_in.clone();

    // Split the input into tokens.
    let toks = input.split(' ').collect::<Vec<&str>>();
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

/// `safe_eval` takes a `&str` evaluates the expression, returning the
/// resulting stack as a Result if the expression is valid, or an Err if the
/// expression is invalid or otherwise cannot be evaluated.
///
/// safe_eval is useful for testing the validity of an expression, and is safer
/// than eval, as it does not mutate any inputted values.
///
/// # Examples  
///
/// ```
/// use dc-ock::safe_eval;
///
/// fn main() {
///     let expr = "1 2 +";
///     match safe_eval(expr) {
///         Ok(x) => println!("{:?}", x), // prints [3.0]
///         Err(e) => println!("{}", e),  // prints an error message
///     }
/// }  
/// ```
pub fn safe_eval(input: &str) -> Result<VecDeque<f64>, EvaluationError> {
    // Initialise the stack
    let mut stack: VecDeque<f64> = VecDeque::new();

    // Split the input into tokens.
    let toks = input.split(' ').collect::<Vec<&str>>();
    let mut ops: VecDeque<CalcType> = VecDeque::new();

    for tok in &toks {
        let x: CalcType = match str_to_calc_type(tok) {
            Some(x) => x,
            None => {
                return Err(EvaluationError {
                    message: format!("Invalid token: {}", tok),
                })
            }
        };

        match x {
            Add | Divide | Multiply | Power | Subtract | Print => ops.push_back(x),

            Val(x_) => stack.push_back(x_),
        }
    }

    for op in &ops {
        match op {
            Add => {
                let y = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                let x = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                &stack.push_back(x + y)
            }
            Subtract => {
                let y = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                let x = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                &stack.push_back(x - y)
            }
            Multiply => {
                let y = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                let x = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                &stack.push_back(x * y)
            }
            Divide => {
                let y = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                let x = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                &stack.push_back(x / y)
            }
            Power => {
                let y = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;
                let x = &stack.pop_back().ok_or(EvaluationError {
                    message: "Stack is empty!".to_string(),
                })?;

                let result = x.powf(*y);
                &stack.push_back(result)
            }
            Print => &{ println!("{:#?}", stack.iter().last()) },
            Val(_) => panic!("Unexpected value in the operator stack!"),
        };
    }

    Ok(stack)
}
