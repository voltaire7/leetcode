use std::env;

mod p1;
mod p2;

fn main() {
    if env::args().len() != 2 {
        println!("[ERROR] Invalid number of arguments. Please use this format: cargo run 1a");
        return;
    }
    let id = env::args().nth(1).unwrap();
    match id.as_str() {
        "1" => p1::test(),
        "2" => p2::test(),
        _ => println!("[ERROR] Project with id '{id}' does not exist."),
    }
}
