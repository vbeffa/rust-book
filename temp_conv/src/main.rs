use std::io;

fn main() {
    println!("Temperature converter");

    loop {
        println!("Enter temperature in Fahrenheit:");

        let mut fahr = String::new();

        io::stdin()
            .read_line(&mut fahr)
            .expect("Failed to read line");

        let fahr: f32 = match fahr.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let cels = (fahr - 32.0) * 5.0 / 9.0;

        println!("Celsius: {:.2}", cels);
    }
}
