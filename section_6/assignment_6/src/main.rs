trait Details {
    fn set_top_speed(&mut self, top_speed: f32) {}
    fn set_mpg(&mut self, mpg: u32) {}
    fn set_color(&mut self, color: String) {}
}

#[derive(Debug)]
struct Car {
    mpg: u32,
    color: String,
    top_speed: f32,
}

impl Details for Car {
    fn set_top_speed(&mut self, top_speed: f32) {
        self.top_speed = top_speed;
    }
    fn set_mpg(&mut self, mpg: u32) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color
    }
}

#[derive(Debug)]
struct Bike {
    mpg: u32,
    color: String,
    top_speed: f32,
}

impl Details for Bike {
    fn set_top_speed(&mut self, top_speed: f32) {
        self.top_speed = top_speed;
    }
    fn set_mpg(&mut self, mpg: u32) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color
    }
}

fn print_val<T>(item: T)
where
    T: std::fmt::Debug,
{
    println!("Printing item...");
    println!("{:?}", item);
}

fn main() {
    let mut ferrari = Car {
        mpg: 100,
        color: String::from("red"),
        top_speed: 250.5,
    };

    ferrari.set_top_speed(300.10);
    print_val(ferrari);

    let mut ducati = Bike {
        mpg: 100,
        color: String::from("red"),
        top_speed: 250.5,
    };

    ducati.set_top_speed(220.0);
    ducati.set_color(String::from("blue"));
    print_val(ducati);
}
