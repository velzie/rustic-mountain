// use std::{cell::RefCell, rc::Rc};

// fn main() {
//     let mut vect: Vec<Rc<RefCell<dyn Obj>>> = vec![];
//     vect.push(Rc::new(RefCell::new(ExampleObject { my_value: 1 })));
//     for i in 0..vect.len() {
//         let mut clonedrc = vect[i].clone();
//         clonedrc.borrow_mut().do_something(&mut vect);
//     }
// }
// struct ExampleObject {
//     pub my_value: i32,
// }
// impl Obj for ExampleObject {
//     fn do_something(&mut self, state: &mut Vec<Rc<RefCell<dyn Obj>>>) {
//         state.push(Rc::new(RefCell::new(ExampleObject { my_value: 1 })));
//     }
// }
// trait Obj {
//     fn do_something(&mut self, state: &mut Vec<Rc<RefCell<dyn Obj>>>);
// }
