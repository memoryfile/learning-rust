use std::io;

fn main() {
    println!("Input a number.");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number2 = match number.parse::<u32>() {
        Ok(number2) => number2,
        Err(_e) => 1,
    };

    fib(number2);
}

fn fib(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => fib(num - 1) + fib(num - 2),
    }
    // println!("{}", fib(num));
}
