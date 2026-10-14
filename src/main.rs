use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("{}", file_path);

    let contents = fs::read(file_path).expect("Read file");

    dbg!("{}", contents);
}
