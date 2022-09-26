fn main() {
    'ten: for _ten in 1..=3 {
        '_uni: for unit in 1..=10 {
            if unit % 2 == 0 {
                continue 'ten;
            }
            if unit > 5 {
                continue;
            }
            let o = unit.to_string();
            println!("{}", o)
        }
    }
}
