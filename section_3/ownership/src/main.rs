fn main() {
    let var1 = 10; // on the stack
    println!("{}", var1);

    let mut s = "hello".to_string(); // on the heap
    s.push_str(", world");
    println!("{}", s);
}

// var is dropped, s is dropped
