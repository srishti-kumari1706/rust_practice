use std::rc::Rc;

struct Node {
    element: i32,
    next: Option<Rc<Node>>,
}

pub struct Stack {
    top: Option<Rc<Node>>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { top: None }
    }
    // push and pop look identical to the Box implementation!
    pub fn push(&mut self, value: i32) {
        self.top = Some(Rc::new(Node {
            element: value,
            next: self.top.take(),
        }));
    }
}
