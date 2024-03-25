struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn whats_my_width(&self) -> u32 {
        self.width
    }

    fn update_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let mut rec = Rectangle {
        height: 20,
        width: 30,
    };
    println!("area: {}", rec.area());
    println!("width: {}", rec.whats_my_width());

    rec.update_width(40);
    println!("new width: {}", rec.whats_my_width());
    println!("new area: {}", rec.area());
}
