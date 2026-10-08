use std::fs;
use std::io;

fn main() {
    let mut path: String = String::new();
    io::stdin().read_line(&mut path).unwrap();

    match fs::read(path.trim()) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}

