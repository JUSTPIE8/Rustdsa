use std::rc::Rc;

use std::cell::RefCell;
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)] //Transaction for performing operation from head or tail
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
                old.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(node.clone()),
        }
        self.length += 1;
        self.tail = Some(node);
        //   println!("{:?}", self);
    }

    pub fn pop(&mut self) -> Link {
        let head = self.head.clone();
        match self.head.take() {
            Some(old) => {
                self.head = old.borrow_mut().next.clone();
            }
            None => {}
        }
        // println!("{:?}", head.as_ref().unwrap().borrow_mut().value);
        //   head.unwrap().borrow_mut().value.to_owned()
        head
    }
}
#[derive(Debug)]
pub struct ListIterator {
    current: Link,
}
impl ListIterator {
    pub fn new(start_at: Link) -> ListIterator {
        ListIterator { current: start_at }
    }
}
impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some( current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}
impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        //  println!("{:?}", &self.current.as_ref().unwrap().borrow_mut().prev);
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}
