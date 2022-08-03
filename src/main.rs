
struct Test<'a> {
    v: &'a i32,
}

trait MyTestTrait {
    type Output;
    fn borrow_mut(self) -> Self::Output;
}

trait Trait2 {
    type Output;
    fn borrow_mut2(self) -> Self::Output;
}

type Ptr_Test<'a> = &'a mut Test<'a>;

type Ptr2<'a,'b> = &'b mut Test<'a>;

impl<'a> MyTestTrait for Ptr_Test<'a> {
    type Output = i32;
    fn borrow_mut(self) -> i32 {
       0
    }
}

impl Trait2 for Test<'_> {
    type Output = i32;
    fn borrow_mut2(self) -> i32 {
       0
    }
}

fn main(){
   let i = 0;
   let mut m = Test { v: &i };

   let a = &mut m;
   { a.borrow_mut(); }

//    let i = m.v;
}