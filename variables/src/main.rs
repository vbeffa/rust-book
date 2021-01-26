fn main() {
    let _guess = "42".parse::<u32>().expect("Not a number!");
    
    let x = 5;

    let x = x + 1;

    let x = x * 2;
    
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of (x, y, z) is: ({}, {}, {})", x, y, z);

    let five_hundred = tup.0;

    println!("The value of x is: {}", five_hundred);

    let six_point_four = tup.1;

    println!("The value of y is: {}", six_point_four);

    let one = tup.2;

    println!("The value of z is: {}", one);

    let a: [i8; 5] = [1, 2, 3, 4, 5];

    println!("The value of a[2] is: {}", a[2]);

    let seven_sevens = [7; 7];

    println!("The value of seven_sevens[3] is: {}", seven_sevens[3]);

    let foo = 3 + 4;
    
    println!("Danger, Will Robinson, danger! {}", seven_sevens[foo]); // compile error, not runtime error
}
