fn main() {
    // Question 1
    // ques_1();

    // Question 2
    // ques_2();

    // Question 3
    // ques_3();

    // Question 4
    ques_4();
}

fn ques_1() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{}", ans);
}

fn ques_2() {
    let mut v = vec![2, 4, 6, 8, 10];
    println!("{:?}", v);
    v.pop();
    v.push(12);
    println!("{:?}", v);
}

fn ques_3() {
    let s = String::from("Hello");
    let result = concat_string(s);
    println!("{}", result);
}

fn concat_string(mut s: String) -> String {
    s.push_str(" World");
    s
}

fn ques_4() {
    control_flow(1);
    control_flow(53);
    control_flow(21);
    control_flow(27);
}

fn control_flow(x: i8) {
    if x == 1 {
        println!("The value is one");
    } else if x > 50 {
        println!("The value is greater than 50");
    } else if x < 25 {
        println!("The value is less than 25");
    } else if x > 25 && x < 50 {
        println!("The value is greater than 25 but less than 50");
    }
}
