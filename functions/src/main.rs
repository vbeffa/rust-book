fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    expressions();

    let x = five();

    println!("The value of x is: {}", x);

    let y = plus_one(five());

    println!("The value of y is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expressions() {
    let x = 5;

    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
