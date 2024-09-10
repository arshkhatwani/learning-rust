use std::fs::File;

fn main() {
    // println!("Hello, world!");
    // // panic!("something happened!"); // also occurs when something is accessed outside bound of vector
    // println!("Hello, world!2");

    // let a = vec![1, 2, 3];
    // println!("{}", a[3]); // panic is expected here

    // let file = File::open("error.txt");
    // the above should return a 'Result'
}

// Result looks something like this
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }