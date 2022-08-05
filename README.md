# dc.rs

dc.rs (also dc-ock, for the Open Computing Kit) is a reverse Polish notation (postfix) calculator library,
providing backend functionality roughly akin to the unix "dc" command, written in Rust.
It lacks many of the more advanced features of dc, but still has core mathematical operations needed for most simple usage.

There is a [`command line utility`](https://crates.io/crates/dc-cli) available built on this library.

## Building dc.rs

...is a little bit awkward, but it's not too bad.

To build dc.rs, you'll need `cargo`.You can build the library with `cargo build --lib`,
or you can build the executable with `cargo build --features bbin --bin dc-cli`.

It's a bit weird because there are binary-only dependencies, but rust doesn't support them yet,
so we hack around it by using the `--features` flag. This stops massive downloads of the binary dependencies
when you build the library.

## Examples

Conversion from string to CalcType:

```rust
fn main() {
  let x: CalcType = str_to_calc_type("1").unwrap(); // returns Val(1.)
  let y: CalcType = str_to_calc_type("+").unwrap(); // returns Addition
  let z: CalcType = str_to_calc_type("*").unwrap(); // returns Multiplication
  // and so on.
}
```

Evaluating expressions safely:

```rust
fn main() {
    let expr = "1 2 +";
    match safe_eval(expr) {
        Ok(x) => println!("{:?}", x), // prints [3.0]
        Err(e) => println!("{}", e),  // prints an error message
    }
}  
```

Evaluating expressions with stack persistence:

```rust
fn main() {
    let mut stack: VecDeque<f64> = VecDeque::new();
    stack.push_back(2.);
    stack.push_back(7.5);
    stack.push_back(3.5);

    stack = safe_eval_with_stack("+ +", stack).unwrap();
    println!("{:?}", stack);
}
```
