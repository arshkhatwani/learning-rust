use std::fs::{rename, File};
use std::io::{Error, ErrorKind};

fn main() {
    // println!("Hello, world!");
    // // panic!("something happened!"); // also occurs when something is accessed outside bound of vector
    // println!("Hello, world!2");

    // let a = vec![1, 2, 3];
    // println!("{}", a[3]); // panic is expected here

    // let file = File::open("error.txt");
    // the above should return a 'Result'

    // Catching errors using 'match' statement
    // let file = File::open("error.txt");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("error.txt") {
    //             Ok(file_created) => file_created,
    //             Err(error) => panic!("could not create file due to {}", error),
    //         },
    //         _ => panic!("_"),
    //     },
    // };
    // println!("{:?}", file);

    // Using unwrap to catch errors
    // let file = File::open("error.txt").unwrap();

    // Using expect to catch errors
    // let file = File::open("error.txt").expect("Error opening the file");

    // Error propagation
    // let file = open_file();
    // file.unwrap();

    rename_file().unwrap();
}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    Ok(file)
}

fn rename_file() -> Result<(), Error> {
    let file = rename("error.txt", "renamed.txt")?;
    Ok(file)
}

// Result looks something like this
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }
