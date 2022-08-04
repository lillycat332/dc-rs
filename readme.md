# dc.rs

dc.rs is a reverse Polish notation (postfix) calculator, roughly akin to the unix "dc" command, written in Rust.
It lacks many of the more advanced features of dc, but still has core mathematical operations needed for most simple usage.

In addition to the command-line utility, dc.rs provides library components for other programs to use, including the RPN evaluator, the types and utility functions for said types.

## Examples

```rust
use dc-ock::str_to_calc_type;

fn main() {
  let x: CalcType = str_to_calc_type("1").unwrap(); // returns Val(1.)
  let y: CalcType = str_to_calc_type("+").unwrap(); // returns Addition
  let z: CalcType = str_to_calc_type("*").unwrap(); // returns Multiplication
  // and so on.
}
```
