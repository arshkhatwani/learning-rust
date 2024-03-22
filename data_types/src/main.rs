fn main() {
    let x: i8 = -10;
    println!("integer {}", x);

    let y: u8 = 10;
    println!("unsigned integer {}", y);

    let decimal = 2_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("decimal {}", decimal);
    println!("hex {}", hex);
    println!("octal {}", octal);
    println!("binary {}", binary);

    let byte = b'A';
    println!("byte {}", byte);

    let a = 2.0;
    let b: f32 = 1.0;
    println!("floats {} {}", a, b);

    let c: char = 'c';
    println!("char {}", c);

    let t: bool = true;
    println!("bool {}", t);

    let num1 = 10;
    let num2 = 20;
    println!("sum of 2 nums {}", num1 + num2);
}
