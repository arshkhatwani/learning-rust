struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 10.2, y: 20.1 };
    let p3 = Point {
        x: "some string",
        y: 20,
    };

    println!("{} {}", p.x, p.y);
    println!("{} {}", p2.x, p2.y);
    println!("{} {}", p3.x, p3.y);
}
