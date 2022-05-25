use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    next: Vec<Link>,
    pub value: u64,
}

type Link = Option<Rc<RefCell<Node>>>;
