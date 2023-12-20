use std::io::{self, Read};

fn main() {
    println!("Hello, World");
    for i in io::stdin().bytes() {
        let c = i.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            break;
        }
    }
}
