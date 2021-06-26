use std::collections::VecDeque;

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
    println!("queue1:{:?}", queue1);

    /** Remove element */
    queue1.pop_back();
    queue1.pop_front();
    println!("queue1:{:?}", queue1);

    /** Access element */
    queue1.front().unwrap();
    queue1.back().unwrap();
    queue1[1];

    /** Check if val exists */
    queue1.contains(&0);
    for val in &queue1 {
        //
    }

    for (idx, val) in queue1.iter().enumerate() {
        //
    }

    /** Compare */
    let mut queue2 = VecDeque::from(queue1.clone());
    println!("{}", queue1.eq(&queue2));

}