use playgrun::Makro;

#[derive(Makro)]
struct TestDerive;

fn main() {
    let a = TestDerive;
    a.maka();
}
