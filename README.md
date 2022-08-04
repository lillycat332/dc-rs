# dc.rs

dc.rs (also dc-ock, for the Open Computing Kit) is a reverse Polish notation (postfix) calculator, roughly akin to the unix "dc" command, written in Rust.
It lacks many of the more advanced features of dc, but still has core mathematical operations needed for most simple usage.

In addition to the command-line utility, dc.rs provides library components for other programs to use, including the RPN evaluator, the types and utility functions for said types.

## Examples

Conversion from string to CalcType:

```rust
use dc-ock::str_to_calc_type;

fn main() {
  let x: CalcType = str_to_calc_type("1").unwrap(); // returns Val(1.)
  let y: CalcType = str_to_calc_type("+").unwrap(); // returns Addition
  let z: CalcType = str_to_calc_type("*").unwrap(); // returns Multiplication
  // and so on.
}
```

Evaluating expressions safely:

```rust
use dc-ock::safe_eval;
fn main() {
    let expr = "1 2 +";
    match safe_eval(expr) {
        Ok(x) => println!("{:?}", x), // prints [3.0]
        Err(e) => println!("{}", e),  // prints an error message
    }
}  
```
