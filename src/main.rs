use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(dead_code)]
struct Node{
    data: i32,
    child: Option<Weak<RefCell<Node>>>,
}

fn main(){
    loop {
        let node1 = Rc::new(RefCell::new(Node {
            data: 1,
            child: None,
        }));
        let node2 = Rc::new(RefCell::new(Node {
            data: 2,
            child: None,
        }));

        node1.borrow_mut().child = Some(Rc::downgrade(&node2));
        node2.borrow_mut().child = Some(Rc::downgrade(&node1));
    }
}