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

    // ---Clone--- -> Deep Copy
    // let x = vec!["hello".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("x: {:?}", x);
    // println!("y: {:?}", y);
    // println!("z: {:?}", z);

    // ---Copy--- -> Can be implemented on types which are already stored on Stack such as integer, bool, float etc.
    // let x = 10;
    // let y = x;
    // println!("x: {} y: {}", x, y);

    // let s: String = String::from("takes");
    // takes_ownership(s);
    // // println!("{}", s); // won't work

    // let val: i32 = 10;
    // make_copy(val);
    // println!("{}", val);

    // let t = give_ownership();
    // println!("{}", t);

    // let s2 = take_and_give("world".to_string());
    // println!("{}", s2);

    // ---References and Borrowing---
    let mut s = String::from("hello");
    change_str(&mut s);
    println!("{}", s);
    // Here we were able to change the value without taking ownership inside the function, if we had not passed by reference then 'change_str' would have taken ownership of s hence we would not be able to use 's' after function
}

fn change_str(s: &mut String) {
    s.push_str(", world");
}

// fn takes_ownership(s: String) {
//     let new_str = s;
//     println!("{}", new_str);
// }

// fn make_copy(val: i32) {
//     let cpy_val = val;
//     println!("{}", cpy_val);
// }

// fn give_ownership() -> String {
//     "hello".to_string()
// }

// fn take_and_give(s: String) -> String {
//     s
// }

// var is dropped, s is dropped
