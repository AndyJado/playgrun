

fn main() {
    let kaz = |c:u8| Dum(c);
    let k_kaz = |d:Dum| Gum(d);
    da(kaz(3),k_kaz);
}

pub struct Dum(pub u8);

pub struct Gum(pub Dum);

fn da(dum: Dum, cb: impl Fn(Dum) -> Gum) {
    let a = cb(dum);
    println!("{}");
}