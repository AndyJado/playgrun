use std::{rc::Rc,};
use std::fmt::Debug;
use uuid::Uuid;

use ang::Angle;

#[derive(Default,Debug)]
struct Expression {
    intention: Intention,
    bias: Vec<Option<Angle>>,
    expressed: Vec<Box<dyn Observable>>,
}

impl Expression {
    fn new() -> Expression {
        Expression { intention: Intention::new(), bias: vec![], expressed: vec![] }
    }

    fn new_from(itn:Intention) -> Expression {
        Expression { intention: Intention::new_from(itn), bias: vec![], expressed: vec![] }
    }

    fn express(&mut self,sth: Box<dyn Observable>) {
        self.bias.push(None);
        self.expressed.push(sth);
    }

    fn biased(&mut self, deg: Angle) {
        let last = self.bias.pop();
        match last {
            Some(cta) => {
                let new = cta.unwrap_or_default();
                self.bias.push(Some(new + deg));
            },
            None => (),
        };
    }

    fn key(&self) -> String {
        let ref int = self.intention;        
        int.key().to_string()
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
struct Intention {
    id:Uuid,
    origin: Option<Rc<Intention>>
}

impl Intention {

    fn new() -> Self {
        Intention { id: Uuid::new_v4(), origin: None }
    }

    fn new_from(self) -> Self {
        let new_id = Uuid::new_v4();
        
        Intention { 
            id: new_id,
            origin: Some(Rc::new(self)), 
            }  
    }

    fn another(&self) -> Self {
        Intention {
             id: Uuid::new_v4(), 
             origin: Some(Rc::clone(&self.origin.as_ref().unwrap())) 
        }
    }

    fn forget(&mut self) {
        self.origin = None
    }

    fn key(&self) -> Uuid {
        let ori = self.origin.as_ref();
        match ori {
            Some(itn) => itn.key(),
            None => self.id,
        }
    }
}

#[derive(Debug)]
struct Url(String);

impl Observable for Url { }

trait Observable:Debug { }

fn main() {
    let zero = Intention::new();
    let mut one = zero.new_from();
    one.forget();
    let two = one.new_from();

    // let cnt = Rc::strong_count(&two.origin.as_ref().unwrap());
    let two2 = two.another();
    println!("{}",Rc::strong_count(&two.origin.as_ref().unwrap()));
    let mut say = Expression::new_from(two);

    say.express(Box::new(Url("sadL".to_string())));

    let deg = Angle::Degrees(3099.into());
    say.biased(deg.normalized());
    println!("{:?}", say);

    say.express(Box::new(Url("2nd URL".to_string())));
    let deg = Angle::Degrees(50.into());
    say.biased(deg);
    let mut say2 = Expression::new();
    say2.biased(deg);
    println!("{:?}", say.key());
    println!("{:?}", say2)


}



