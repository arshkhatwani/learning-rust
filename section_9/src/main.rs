fn main() {
    println!("Hello, world!");
    // panic!("something happened!"); // also occurs when something is accessed outside bound of vector
    println!("Hello, world!2");

    let a = vec![1, 2, 3];
    println!("{}", a[3]); // panic is expected here
}
