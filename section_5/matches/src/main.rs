fn main() {
    let x: Option<i32> = Some(5);

    let res1 = plus_one(x);
    let res2 = plus_one(None);

    println!("{:?}", res1);
    println!("{:?}", res2);

    what_pet("dog");
    what_pet("fish");
    what_pet("cat");
    what_pet("cow");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn what_pet(pet: &str) {
    match pet {
        "cat" => println!("This is a cat"),
        "dog" => println!("This is a dog"),
        "fish" => println!("This is a fish"),
        _ => println!("I am not sure what pet this is"),
    }
}
