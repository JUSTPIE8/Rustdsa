use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: String,
    next: Link,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}
type Link = Option<Rc<RefCell<Node>>>;
struct Transaction {
    length: u8,
    head: Link,
    tail: Link,
}
impl Transaction {
    fn empty() -> Transaction {
        Transaction {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new)
    }
}

fn main() {}
