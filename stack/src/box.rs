#[derive(Debug)]
struct Node {
    element: i32,
    next: Option<Box<Node>>,
}

pub struct Stack {
    top: Option<Box<Node>>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { top: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            element: value,
            next: self.top.take(), // Cleanly moves ownership into the box
        });
        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.top.take().map(|node| {
            println!("Popping node: {:?}", node);
            self.top = node.next; // Moves the next node to the top
            node.element
        })
    }
}