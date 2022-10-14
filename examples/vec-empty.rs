#[derive(Debug)]
pub enum LaVec<T> {
    Y(Vec<T>),
    N(Option<T>),
}

impl<T: Default> LaVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        match vec.is_empty() {
            true => LaVec::N(None),
            false => LaVec::Y(vec),
        }
    }

    pub fn upgrage(&mut self) {
        match self {
            LaVec::Y(_) => {}
            LaVec::N(_) => todo!(),
        }
    }

    pub fn push(&mut self, value: T) {
        match self {
            LaVec::Y(v) => v.push(value),
            LaVec::N(el) => match el {
                Some(w) => todo!(),
                None => todo!(),
            },
        }
    }
    pub fn last_or_push<'a, 'b: 'a>(&'b mut self) -> &'a T {
        match self {
            LaVec::Y(ve) => ve.last().unwrap(),
            LaVec::N(w) => {
                *w = Some(T::default());
                w.as_ref().unwrap()
            }
        }
    }
    pub fn check(self) -> Self {
        match self {
            LaVec::Y(_) => self,
            LaVec::N(e) => match e {
                Some(w) => Self::new(vec![*w]),
                None => self,
            },
        }
    }
}

fn main() {
    let a = vec![1, 2, 3, 4];
    let b = LaVec::new(a);
}
