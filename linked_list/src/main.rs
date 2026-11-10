fn main() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }
    impl Node {
        fn new(value: i32) -> Self {
            Node {value, next: None}
        }
    }

    #[derive(Debug)]
    struct LinkedList {
        head: Option<Box<Node>>,
        len: usize,
    }
    impl LinkedList {
        fn new() -> Self {
            LinkedList { 
                head: None,
                len: 0 
            }
        }
        fn append(&mut self, value: i32) {
            let node = Box::new(Node::new(value));
            let mut curr = &mut self.head;
            while let Some(nxt) = curr {
                curr = &mut nxt.next;
            }
            *curr = Some(node);
            self.len += 1;

        }
        fn delete(&mut self, value: i32) {
            let mut curr = &mut self.head;
            loop {
                if curr.is_none() {
                    break;
                }
                if curr.as_ref().unwrap().value == value {
                    println!("Deleting node with value: {:?}", curr.as_mut().unwrap());
                    let mut del_node = &mut curr.as_mut().unwrap();
                    *curr = del_node.next.take();       // overwrite the current node with the next node
                    // *curr = &mut curr.as_mut().unwrap().next.take(); // single line version of the above two lines
                    self.len -= 1;
                    break;
                }

                // curr = &mut node.next;     // change WHERE curr points
                // *curr = node.next.take();  // change WHAT is stored there

                curr = &mut curr.as_mut().unwrap().next;
            }
        }
        // fn delete(&mut self, value: i32) {
        //     let mut curr = &mut self.head;
        //     while let Some(node) = curr {
        //         if node.value == value {
        //             *curr = node.next.take();
        //             self.len -= 1;
        //             return;
        //         }
        //         curr = &mut node.next;
        //     }
        // }

    }

    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);
    list.delete(2);
    println!("{:?}", list);
}
