fn main() {
    println!("Hello, world!");

    // string literal
    let mut s = "hello";

    println!("mutable string s initialized to string literal: {}", s);

    s = "bye";

    println!("s after mutation: {}", s);

    // String::from
    let mut t = String::from("hello");

    println!("mutable string t using String::from: {}", t);

    t.push_str(", world!");

    println!("t after push_str op: {}", t);

    // immutable s1 assigned to s2
    let s1 = "hello";

    let s2 = s1;

    println!("s1: {}, s2: {}", s1, s2);

    // String::from s3 assigned to s4
    let s3 = String::from("hello");

    // let s4 = s3; // error[E0382]: borrow of moved value: `s3`

    let s4 = s3.clone();

    println!("s3: {}, s4: {}", s3, s4);

    // borrowed reference
    let s5 = String::from("hello reference");

    let len = calculate_length(&s5);

    println!("The length of '{}' is {}.", s5, len);

    // mutable borrowed reference
    let mut s6 = String::from("hello");

    println!("s6 is: {}", s6);

    change(&mut s6);

    println!("s6 is now: {}", s6);

    // multiple mutable borrows

    let mut s7 = String::from("hello");

    let s8 = &mut s7;
    println!("{}", s8);

    let s9 = &mut s7;
    println!("{}", s9);

    // borrowed value
    foo(s7);

    // first_word_index
    let s10 = String::from("hello world");

    let idx = first_word_index(&s10);

    println!("index is: {}", idx);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn foo(s: String) -> usize {
    s.len()
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
