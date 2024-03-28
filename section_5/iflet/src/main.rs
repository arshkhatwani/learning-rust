enum Pet {
    Cat,
    Dog,
    Fish,
}

fn main() {
    let dog1 = Some(Pet::Dog);
    let cat1 = Some(Pet::Cat);
    let fish1 = Some(Pet::Fish);

    what_am_i(dog1);
    what_am_i(cat1);
    what_am_i(fish1);

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{:?}", top);
    }
}

fn what_am_i(x: Option<Pet>) {
    if let Some(Pet::Cat) = x {
        println!("The animal is a cat");
    } else if let Some(Pet::Dog) = x {
        println!("The animal is a dog")
    } else {
        println!("Unknown animal")
    }
}
