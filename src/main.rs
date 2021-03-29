use std::fs;

fn main() {
    println!("Hello, world!");

    // let args = std::env::args();
    // println!("{:?}", args);
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];
    println!("{}:", arg);

    let bytes: Vec<u8> = fs::read(arg).unwrap();
    for b in bytes {
        print!("{:x} ", b);
    }
    println!();
}
