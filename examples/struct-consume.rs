fn main() {
    let a = Bady {
        hand: "sa".to_owned(),
        foot: "dsa".to_owned(),
    };
    a.strip()
}

#[derive(Debug)]
struct Bady {
    pub hand: String,
    pub foot: String,
}

impl Bady {
    fn strip(self) {
        let c = self.foot;
        eatit(c);
        println!("{:?}", self);
    }
}

fn eatit(c: String) {}
