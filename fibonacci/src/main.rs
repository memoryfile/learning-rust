// Code following the tutorials of...
// https://benjaminbrandt.com/fibonacci-in-rust/
// https://dev.to/kelvinkirima014/generating-the-nth-fibonacci-in-rust-238k

use std::io;

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        2 => 1,
        _ => (fib(n - 1)) + (fib(n - 2)),
    }
}

fn to15() {
    for int in 0..15 {
        println!("fib ({}) => {}", int, fib(int)); // Does fib sequence 1 -> 15
    }
}

fn main() {
    println!("Fibonnaci generator");
    println!("Type \"quit\" to exit, or \"to15\" to receive a 1 -> 15 fibonacci sequence!");

    loop {
        let mut n = String::new();

        println!("\nEnter your input:");

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.trim() == "quit" {
            break;
        }

        if n.trim() == "to15" {
            to15();
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fib(n));
    }
}
