fn main() {
    say_hello("John");

    let result = add(2, 3);
    println!("{}", result);

    let gcd_num = gcd(36, 42);
    println!("{}", gcd_num);

    println!(
        "{} {}",
        multiple_return_values(true),
        multiple_return_values(false)
    );
}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}

fn add(a: i8, b: i8) -> i8 {
    a + b
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}
