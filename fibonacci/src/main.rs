use std::collections::HashMap;
use std::io;

fn main() {
    println!("Fibonacci calculator");

    let mut fibs: HashMap<u32, u64> = HashMap::new();
    fibs.insert(1, 1);
    fibs.insert(2, 1);

    loop {
        println!("Enter the value of N:");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 0 || n > 93 {
	    println!("N must be between 1 and 93.");
            continue;
        }

        for i in 3..n + 1 {
            if !fibs.contains_key(&i) {
                fibs.insert(
                    i,
                    fibs.get(&(i - 1)).unwrap() + fibs.get(&(i - 2)).unwrap(),
                );
            }
        }

        println!("Fib(N) = {}", fibs.get(&n).unwrap());
    }
}
