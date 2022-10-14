use std::collections::HashMap;

fn main() {
    let mut map = HashMap::<i32, String>::new();
    let key = 3;
    match map.get_mut(&key) {
        Some(val) => {
            val;
        }
        None => {
            map.insert(key, String::new());
        }
    }
}
