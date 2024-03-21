fn main() {
    // let x = 5; // immutable variable

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The new value of x is {}", x);

    const SECONDS: i8 = 23;
    println!("The new value of seconds is {}", SECONDS);
}
