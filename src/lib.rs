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
