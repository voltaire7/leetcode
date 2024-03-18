use std::env;

mod problem1;

fn main() {
    if env::args().len() != 2 {
        println!(
            "[ERROR] Invalid number of arguments. Please use the following format: cargo run 1a"
        );
        return;
    }
    let id = env::args().nth(1).unwrap();
    match id.as_str() {
        "1" => problem1::call(),
        _ => println!("[ERROR] Project with id '{id}' does not exist."),
    }
}
