mod internal;

use dc_ock::eval;
use internal::util::get_config;
use std::{
    collections::VecDeque,
    io::{self, stdin, stdout},
    process::exit,
};

#[allow(unused_assignments)]
fn main() {
    let config = get_config();

    let prompt: String = {
        let this = config.get("prompt");
        match this {
            Some(x) => x,
            None => ">>> ",
        }
    }
    .to_string();

    loop {
        print!("{}", prompt);
        io::Write::flush(&mut stdout()).expect("Flush error");
        let mut stk: VecDeque<f64> = VecDeque::new();
        let mut in_str = String::new();

        let mut clos_mutate = |x: &str| eval(x.strip_suffix('\n').unwrap_or(x), &mut stk);

        match stdin().read_line(&mut in_str) {
            Ok(0) => {
                exit(0);
            }
            Ok(_) => clos_mutate(&in_str),
            Err(e) => println!("Couldn't parse input: {}", e),
        };
    }
}
