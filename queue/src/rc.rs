use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    element: i32,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct Queue {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue{
            head: None,
            tail: None,
        }
    }
    pub fn enqueue(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node{element: value, next: None}));
        match &self.tail {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }
        self.tail = Some(new_node);
    }
    pub fn dequeue(&mut self) {
        // self.head.take().map(|old_head| {
        //     let next_node = old_head.borrow().next.clone();
        //     self.head = next_node.clone();
        //     next_node
        // });

        self.head.take().map(|old_head| {
            let next_node = old_head.borrow().next.clone();
            self.head = next_node;
            old_head
        });
    }

    pub fn print_addrs(&self) {
        println!("Stack address of head {:?}", &self.head as *const _);
        println!("Stack address of tail {:?}", &self.tail as *const _);

        match &self.head {
            Some(rc_head) => println!("Heap address of head node {:p}", Rc::as_ptr(rc_head)),
            None => println!("Heap address of head node is None"),
        }
        match &self.tail {
            Some(rc_tail) => println!("Heap address of tail node {:p}", Rc::as_ptr(rc_tail)),
            None => println!("Heap address of tail node is None"),
        }
    }
}