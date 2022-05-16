use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct Node {
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
pub struct Transaction {
    pub length: u8,
    head: Link,
    pub tail: Link,
}
impl Transaction {
    pub fn empty() -> Transaction {
        Transaction {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new)
    }

    pub fn pop(&mut self) -> String {
        let head = self.head.clone();
        match self.head.take() {
            Some(value) => self.head = value.borrow_mut().next.take(),
            None => {}
        }
        self.length -= 1;
        // println!(" head {:?}", &head);
        head.unwrap().borrow_mut().value.to_owned()

        /* Another way of implementation
         * Rc::try_unwrap(head.unwrap())
            .ok()
            .expect("something terrible papened")
            .into_inner()
            .value

            self.head.take().map(|head| {
                if let Some(next) = head.borrow_mut().next.take() {
                    self.head = Some(next);
                } else {
                    self.tail.take();
                }
                self.length -= 1;
                Rc::try_unwrap(head)
                    .ok()
                    .expect("Something is terribly wrong")
                    .into_inner()
                    .value
            })
        */
    }
}
