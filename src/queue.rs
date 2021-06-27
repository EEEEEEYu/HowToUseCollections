use std::collections::VecDeque;

pub fn print_queue(queue: &VecDeque<i32>) {
    println!("%%%%%%%%%%%%%%%%%%");
    println!("len:{} elements:{:?}", queue.len(), queue);
}

#[allow(unused)]
pub fn queue_test() {
    /** Initialize */
    let mut queue1 = VecDeque::new();

    /** Add element */
    queue1.push_back(2);
    queue1.push_back(3);
    queue1.push_front(1);
    queue1.push_front(5);
    queue1.push_back(44);
    print_queue(&queue1);

    /** Remove element */
    queue1.pop_back();
    queue1.pop_front();
    print_queue(&queue1);

    /** Access element */
    queue1.front().unwrap();
    queue1.back().unwrap();
    queue1[1]; // Can access by index

    /** Check if val exists */
    queue1.contains(&0);

    /** Traverse */
    for val in &queue1 {
        //
    }

    for (idx, val) in queue1.iter().enumerate() {
        //
    }

    /** Merge queues */
    let mut queue2 = queue1.clone();
    queue1.append(&mut queue2.clone());
    print_queue(&queue1);

    /** Compare */
    let mut queue2 = queue1.clone();
    println!("{}", queue1.eq(&queue2));
}