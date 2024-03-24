fn main() {
    // ---Ownership---
    // let var1 = 10; // on the stack
    // println!("{}", var1);

    // let mut s = "hello".to_string(); // on the heap
    // s.push_str(", world");
    // println!("{}", s);

    // ---Move---
    // let x = vec!["hello".to_string()];
    // let y = x; // value of x is moved

    // // println!("x: {:?}", x); // won't work since value of x is moved, see compiler error for more details
    // println!("y: {:?}", y);

    // // now similar would happen in case of y
    // let z = y;
    // // println!("y: {:?}", y); // won't work
    // println!("z: {:?}", z);

    // ---Clone---
    let x = vec!["hello".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
}

// var is dropped, s is dropped
