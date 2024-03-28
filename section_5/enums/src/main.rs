enum Pet {
    cat,
    dog,
    fish,
}

impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::cat => "I am a cat",
            Pet::dog => "I am a dog",
            Pet::fish => "I am a fish",
        }
    }
}

enum IpAddrKind {
    V4(String),
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let pet = Pet::dog;
    println!("{}", pet.what_am_i());

    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        address: String::from("127.0.0.1"),
    };

    let office = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
