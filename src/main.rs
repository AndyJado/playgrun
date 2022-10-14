use playgrun::AnswerFn;

#[derive(AnswerFn)]
struct TestDerive;

fn main() {
    let a = TestDerive;
    a.answer();
}
