trait Bob<T: Default> {
    fn bob(&mut self) -> LaVec<T>;
}

impl<T: Default> Bob<T> for Vec<T> {
    fn bob(&mut self) -> LaVec<T> {
        LaVec::default()
    }
}

#[derive(Default)]
pub enum LaVec<T: Default> {
    Da(Vec<T>),
    Wa(T),
    #[default]
    Na,
}

impl<T: Default> LaVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        use LaVec::*;
        match vec.last() {
            Some(_) => Da(vec),
            None => Na,
        }
    }
    pub fn push(e: &mut Self, dish: T) {
        use LaVec::*;
        *e = match e {
            Da(vec) => {
                vec.push(dish);
                *e
            }
            Wa(e) => todo!(),
            Na => todo!(),
        }
    }
}

fn main() {}
