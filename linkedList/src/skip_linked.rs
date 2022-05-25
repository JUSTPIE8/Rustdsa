use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

type Link = Option<Rc<RefCell<Node>>>;

struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

#[derive(Clone)]

pub struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl BestTransactionLog {
    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level //use the max level for the first node
        } else {
            self.get_level()
        };
        let new = Node::new(vec![None; level], offset, value);
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            if self.head.is_none() {
                self.head = Some(new.clone());
            }
            self.length += 1;
        }
    }
}
