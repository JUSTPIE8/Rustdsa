use std::rc::Rc;

use std::cell::RefCell;
#[derive(Debug)]
pub struct Node {
    value: String,
    next: Link,
    prev: Link,
}
impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }
}
type Link = Option<Rc<RefCell<Node>>>;
#[derive(Debug)]
pub struct Transaction {
    pub length: u8,
    head: Link,
    tail: Link,
}
impl Transaction {
    pub fn empty_first() -> Transaction {
        Transaction {
            length: 0,
            head: None,
            tail: None,
        }
    }
    pub fn append(&mut self, value: String) {
        let node = Node::new(value);
        match self.tail.take() {
            Some(old) => {
                //   if (self.length <= 0) {
                //     old.borrow_mut().prev = None
                // } else {
                old.borrow_mut().prev = Some(old.clone());
                // }

                old.borrow_mut().next = Some(node.clone())
            }
            None => self.head = Some(node.clone()),
        }
        self.length += 1;
        self.tail = Some(node);
        println!("{:?}", self);
    }

    pub fn pop(&mut self) -> String {
        let head = self.head.clone();
        match self.head.take() {
            Some(old) => {
                self.head = old.borrow_mut().next.clone();
            }
            None => {}
        }
        head.unwrap().borrow_mut().value.to_owned()
    }
}
