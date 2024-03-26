fn main() {
    let mut ferrari = Car {
        mpg: 100,
        color: String::from("red"),
        top_speed: 250.5,
    };
    ferrari.get_details();

    ferrari.set_mpg(200);
    ferrari.set_color("black".to_string());
    ferrari.set_top_speed(300.10);

    ferrari.get_details();
}

struct Car {
    mpg: u32,
    color: String,
    top_speed: f32,
}

impl Car {
    fn set_mpg(&mut self, mpg: u32) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: f32) {
        self.top_speed = top_speed;
    }

    fn get_details(&self) {
        println!(
            "Car details: mpg: {}, color: {}, top speed: {}",
            self.mpg, self.color, self.top_speed
        );
    }
}
