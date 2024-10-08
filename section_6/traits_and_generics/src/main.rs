use std::ops::Add;

// Generics
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

trait Overview {
    fn overview(&self) -> String {
        String::from("This is a default course")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("Dropping {}", self.author);
    }
}

impl Overview for Course {
    // if we comment this out then default trait implementation will be executed
    // fn overview(&self) -> String {
    //     String::from("This is just a Course")
    // }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        String::from("This is another course")
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    // let p = Point { x: 5, y: 10 };
    // let p2 = Point { x: 10.2, y: 20.1 };
    // let p3 = Point {
    //     x: "some string",
    //     y: 20,
    // };

    // println!("{} {}", p.x, p.y);
    // println!("{} {}", p2.x, p2.y);
    // println!("{} {}", p3.x, p3.y);

    let course1 = Course {
        headline: String::from("RUSTTT"),
        author: String::from("Some person"),
    };
    let course2 = AnotherCourse {
        headline: String::from("Another RUSTTT"),
        author: String::from("Some another person"),
    };

    // println!("{:?}", course1.overview());
    // println!("{:?}", course2.overview());

    call_overview(&course1);
    call_overview(&course2);

    // Even if we not add this, we will still get the dropping log because Rust is automatically dropping it on scope ending
    // We can manually drop and free our resource by adding this line
    // drop(course1);

    // Operator overloading
    let coord1 = Point { x: 10, y: 20 };
    let coord2 = Point { x: 25, y: 30 };

    let result = coord1 + coord2;
    println!("{:?}", result);
}

// fn call_overview(item: &dyn Overview) {
// Another method
// fn call_overview<T: Overview>(item: &T) {
// Another method
fn call_overview(item: &impl Overview) {
    println!("{:?}", item.overview());
}

// Clone
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}
