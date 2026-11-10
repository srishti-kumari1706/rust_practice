// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: Option<RefCell<Rc<Node>>>,
// }

// pub struct Stack {
//     top: Option<RefCell<Rc<Node>>>,
// }

// impl Stack {
//     pub fn new() -> Self {
//         Stack { top: None }
//     }

//     pub fn push(&mut self, value: i32) {
//         let new_node = RefCell::new(Rc::new(Node { element: value, next: self.top.take(), }));
//         self.top = Some(new_node);
//     }

//     pub fn pop(&mut self) -> Option<i32> {
//         self.top.take().map(|node_ref| {
//             println!("Popping node: {:?}", node_ref.borrow());
//             let node = node_ref.borrow();
//             self.top = node.next.clone();
//             node.element
//         })
//     }

//     pub fn peek(&self) -> Option<i32> {
//         self.top.as_ref().map(|node_ref| node_ref.borrow().element)
//     }
// }


use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    element: i32,
    next: Option<Rc<RefCell<Node>>>,
}

pub struct Stack {
    top: Option<Rc<RefCell<Node>>>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { top: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node { element: value, next: self.top.take(), }));
        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.top.take().map(|node| {
            println!("Popping node: {:?}", node.borrow());
            // let next_node = node.borrow_mut().next.take();
            self.top = node.next;
            println!("New top node is: {:?}", node.next);
            node.element
        })
    }

    pub fn peek(&self) -> Option<i32> {
        self.top.as_ref().map(|node_ref| node_ref.borrow().element)
    }
}
