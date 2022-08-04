mod internal;

use dc_ock::safe_eval_with_stack;
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

    let mut stk: VecDeque<f64> = VecDeque::new();

    loop {
        print!("{}", prompt);
        io::Write::flush(&mut stdout()).expect("Flush error");
        let mut in_str = String::new();

        match stdin().read_line(&mut in_str) {
            Ok(0) => exit(0),
            Ok(_) => (),
            Err(_) => panic!("Read error"),
        }

        match safe_eval_with_stack(&in_str.trim(), stk.clone()) {
            Ok(x) => stk = x,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}
