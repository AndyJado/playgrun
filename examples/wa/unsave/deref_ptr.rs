pub fn foo() {
    let mut num = 5;
    // let r0 = &num;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // let r3 = &mut num;
    unsafe { println!("{},{},{},{}", *r1, *r1, *r2, *r2) }
}
