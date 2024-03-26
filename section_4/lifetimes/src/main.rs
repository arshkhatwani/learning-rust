// Specifying lifetime because Rust Compiler needs to ensure that the reference is valid for as long as the struct itself.
struct MyString<'a> {
    text: &'a str,
}

fn main() {
    let str1 = String::from("This is my string");
    let x = MyString {
        text: str1.as_str(),
    };
    let y: &'static str = "I have a static lifetime"; // Static Lifetime

    println!("x.text: {}", x.text);
    println!("y: {}", y);

    let a: &str = "This is a string";
    let b: &str = "This is a longer string";

    let result = longest(a, b);
    println!("Longest between a and b: {}", result);
}

/*
Would not work due to missing lifetime specifier
this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
*/
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Working function by using generic lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
