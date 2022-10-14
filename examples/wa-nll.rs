fn last_or_push<'b, 'a: 'b>(vec: &'a mut Vec<String>) -> &'a String {
    match vec.last() {
        Some(s) => {
            let as = &'b *s;
        },
        None => todo!(),
    }
    if let Some(s) = vec.last() {
        // borrows vec
        // returning s here forces vec to be borrowed
        // for the rest of the function, even though it
        // shouldn't have to be
        return s;
    }
    // Because vec is borrowed, this call to vec.push gives
    // an error!
    vec.push("".to_string()); // ERROR
    vec.last().unwrap()
}

fn last_o_push<'a>(vec: &'a mut Vec<String>) -> &'a String {
    match vec.pop() {
        Some(corn) => vec.push(corn),
        None => vec.push("".to_string()),
    }
    vec.last().unwrap()
}

fn main() {
    let mut a = vec!["sajljl".to_string()];
    last_or_push(&mut a);
}
