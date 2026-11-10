// mod rcell;
// mod rc;
mod r#box;

fn main() {
    let mut my_stack = r#box::Stack::new();
    // let mut my_stack = rcell::Stack::new();
    
    my_stack.push(10);
    my_stack.push(20);
    
    // println!("Top node is: {:?}", my_stack.peek()); // Outputs: Some(20)
    
    my_stack.pop();
    // println!("New top node is: {:?}", my_stack.peek()); // Outputs: Some(10)
}
