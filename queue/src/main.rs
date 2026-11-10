mod rc;

fn main() {
    let mut my_queue= rc::Queue::new();
    // let mut my_queue= rcell::Stack::new();
    
    my_queue.print_addrs();
    my_queue.enqueue(10);
    my_queue.print_addrs();
    my_queue.enqueue(20);
    my_queue.print_addrs();
    
    println!("Queue is: {:?}", my_queue);
    
    my_queue.dequeue();
    my_queue.print_addrs();
    println!("Queue is: {:?}", my_queue);
}
