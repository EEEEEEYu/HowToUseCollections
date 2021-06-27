use std::collections::BinaryHeap;

pub fn print_queue(queue: &BinaryHeap<i32>) {
    println!("^^^^^^^^^^^^^^^^^^^^^");
    println!("len:{} elements:{:?}", queue.len(), queue);
}

#[allow(unused)]
pub fn priority_queue_test() {
    /** Initialize */
    let mut queue1 = BinaryHeap::new(); // This is max-heap by default

    /** Add elements */
    queue1.push(1);
    queue1.push(3);
    queue1.push(2);
    queue1.push(5);
    print_queue(&queue1);

    /** Remove elements */
    queue1.pop();
    print_queue(&queue1);

    /** Access elements */
    queue1.peek().unwrap();

    /** Traverse elements */
    for elem in &queue1 {
        //
    }

    /** Merge elements */
    let mut queue2 = queue1.clone();
    queue1.append(&mut queue2.clone());
    queue1.extend(&queue2);
    print_queue(&queue1);
}