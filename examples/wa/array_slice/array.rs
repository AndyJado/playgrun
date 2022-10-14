pub fn foo() {
    let mut ary = [8u8; 8];
    let mut vac: Vec<u8> = vec![8, 8, 8, 8, 8, 8, 8, 8];
    let sli = &mut vac;
    match sli.last_mut() {
        Some(la) => *la = 3u8,
        None => {}
    };
    // FIXME: ary implicitly coverts to a slice
    match ary.last_mut() {
        Some(la) => *la = 3u8,
        None => {}
    };
    assert_eq!(ary.as_slice(), sli)
}
