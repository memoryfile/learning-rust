use std::io;

fn main() {
    println!("How do you want to convert your temperature?");
    println!("Input `f2c` for Fahrenheit to Celsius, `c2f` for Celsius to Fahrenheit.");

    let mut choice = String::new();

    let f2c: String = String::from("f2c");
    let c2f: String = String::from("c2f");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: &str = choice.trim();

    if choice.eq(&f2c) {
        loop {
            println!("What number would you like to convert from Fahrenheit to Celsius?");
            let mut input_temp = String::new();

            io::stdin()
                .read_line(&mut input_temp)
                .expect("Failed to read line");

            let input_temp: f64 = match input_temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // °C = (°F − 32) x 5/9
            let output_temp: f64 = (input_temp - 32.0) * 5.0 / 9.0;

            break println!("The temperature in Celsius is: {output_temp}");
        }
    }

    if choice.eq(&c2f) {
        loop {
            println!("What number would you like to convert from Celsius to Fahrenheit?");

            let mut input_temp = String::new();

            io::stdin()
                .read_line(&mut input_temp)
                .expect("Failed to read line");

            let input_temp: f64 = match input_temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            //  °F = (°C × 9/5) + 32
            let output_temp: f64 = (input_temp * 9.0 / 5.0) + 32.0;

            break println!("The temperature in Fahrenheit is: {output_temp}");
        }
    }
}
