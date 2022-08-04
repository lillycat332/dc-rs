use config::Config;
use dc_ock::eval;
use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs;
use std::{
    collections::VecDeque,
    io::{self, stdin, stdout},
    process::exit,
};

fn get_config() -> HashMap<String, String> {
    // Construct the path to the configuration folder
    let binding = ProjectDirs::from("com", "lc332", "dc_rs").unwrap();
    let config_dir = binding.config_dir().to_str().unwrap_or(".");
    let config_file = format!("{}/{}", config_dir, "dc_rs.toml");

    // Ensure the actual file and folder exists. We discard the error because
    // it's pretty likely they'll already exist, and handling the error is sort
    // of pointless.
    fs::create_dir_all(config_dir).ok();
    fs::File::create(&config_file).ok();

    // then load the config and return it.
    let config = Config::builder()
        .add_source(config::File::with_name(&config_file))
        .build()
        .unwrap();

    config.try_deserialize::<HashMap<String, String>>().unwrap()
}

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
