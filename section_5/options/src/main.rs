fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y;
    // println!("sum: {}", sum); // would not work

    println!("y: {:?}", y);
}
