use std::collections::{BinaryHeap, HashMap};

use rand::seq::SliceRandom;

fn main() {
    // let mut nums: Vec<i32> = vec![];

    // nums.push(21);
    // nums.push(3);
    // nums.push(32);
    // nums.push(55);
    // nums.push(14);
    // println!("{:?}", nums);

    // nums.pop();
    // println!("{:?}", nums);

    // nums.insert(1, 100);
    // println!("{:?}", nums);

    // nums.sort();
    // println!("After sort: {:?}", nums);

    // nums.reverse();
    // println!("After reverse: {:?}", nums);

    // nums.shuffle(&mut rand::thread_rng());
    // println!("After shuffle: {:?}", nums);

    // let mut bheap = BinaryHeap::new();

    // bheap.push(1);
    // bheap.push(2);
    // bheap.push(4);
    // bheap.push(5);
    // bheap.push(1);

    // println!("{:?}", bheap);

    // let el = bheap.peek();
    // println!("{:?}", el);

    let mut hm = HashMap::new();

    hm.insert(2, 3);
    hm.insert(3, 6);
    let old = hm.insert(2, 5);
    hm.insert(10, 8);

    println!("{:?}", hm);
    println!("old val: {:?}", old);

    println!("has key? {}", hm.contains_key(&10));
    println!("has key? {}", hm.contains_key(&100));
    println!("find {:?}", hm.get(&10));
    println!("find {:?}", hm.get(&100));

    let one = hm.remove(&2);
    println!("removed: {:?}", one);

    let two = hm.remove_entry(&10);
    println!("removed: {:?}", two);

    hm.clear();
    println!("is empty? {}", hm.is_empty());
}
