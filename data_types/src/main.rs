fn main() {
    // let x: i8 = -10;
    // println!("integer {}", x);

    // let y: u8 = 10;
    // println!("unsigned integer {}", y);

    // let decimal = 2_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;

    // println!("decimal {}", decimal);
    // println!("hex {}", hex);
    // println!("octal {}", octal);
    // println!("binary {}", binary);

    // let byte = b'A';
    // println!("byte {}", byte);

    // let a = 2.0; // f64 by default
    // let b: f32 = 1.0;
    // println!("floats {} {}", a, b);

    // let c: char = 'c';
    // println!("char {}", c);

    // let t: bool = true;
    // println!("bool {}", t);

    // let num1 = 10;
    // let num2 = 20;
    // println!("sum of 2 nums {}", num1 + num2);

    // let tup = (500, "sandy", true);
    // println!("{:?}", tup); // print
    // println!("{:#?}", tup); // pretty print
    // println!("{} {} {}", tup.0, tup.1, tup.2);

    // let (x, y, z) = tup;
    // println!("{} {} {}", x, y, z);

    let array = [1, 2, 3];
    println!("{:?}", array);
    println!("{}", array[1]);

    let mut array: [i32; 4] = [10, 20, 30, 40];
    println!("{}", array[2]);
    array[2] = 15;
    println!("{}", array[2]);
}
