use rand::seq::SliceRandom;

fn main() {
    let mut nums: Vec<i32> = vec![];

    nums.push(21);
    nums.push(3);
    nums.push(32);
    nums.push(55);
    nums.push(14);
    println!("{:?}", nums);

    nums.pop();
    println!("{:?}", nums);

    nums.insert(1, 100);
    println!("{:?}", nums);

    nums.sort();
    println!("After sort: {:?}", nums);

    nums.reverse();
    println!("After reverse: {:?}", nums);

    nums.shuffle(&mut rand::thread_rng());
    println!("After shuffle: {:?}", nums);
}
