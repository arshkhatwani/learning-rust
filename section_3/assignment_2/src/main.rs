// fn main() {
//     let mut v = vec![1, 3, 5, 7];
//     let res = func(&v);
//     println!("res: {}", res);
//     v.push(15);
//     println!("{:?}", v);
// }

// fn func(val: &Vec<i32>) -> bool {
//     if val[0] == 1 {
//         true
//     } else {
//         false
//     }
// }

fn main() {
    let mut val: i8 = 10;
    val = add_two(val);
    println!("val: {}", val);
}

fn add_two(val: i8) -> i8 {
    val + 2
}
