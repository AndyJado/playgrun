fn main() {
    // ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
    let a = <[()]>::len(&[
        (),
        (),
        {
            let a = 3;
        },
        println!("wala"),
    ]);
    let b = <[Option<_>]>::len(&[Some(8)]);
    println!("{a:?}");
}

fn smart_pants() {
    let (the, guardian, stands, resolute) = ("the", "Turbofish", "remains", "undefeated");
    let wa: (bool, bool) = (the < guardian, stands > (resolute));
    println!("{wa:?}");
}
