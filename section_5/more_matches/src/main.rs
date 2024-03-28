fn main() {
    let x = 1;

    match x {
        0 | 1 => println!("Zero or One"),
        _ => println!("Not Zero nor One"),
    }

    let y = 5;

    match y {
        0..=5 => println!("Between 0 and 5"),
        _ => println!("Not between 0 and 5"),
    }

    // let a = Some(10);
    let a = Some(15);
    let b = 15;
    // let b = 5;

    match a {
        Some(10) => println!("10!"),
        Some(a) if a == b => println!("Matches!"),
        _ => println!("No match"),
    }
}
