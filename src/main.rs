mod vector;
mod map;
mod set;
mod queue;
mod priority_queue;
mod linked_list;


use vector::vec_test;
use map::map_test;
use set::set_test;
use queue::queue_test;
use linked_list::linked_list_test;
use priority_queue::priority_queue_test;


fn main() {
    vec_test();
    map_test();
    queue_test();
    set_test();
    linked_list_test();
    priority_queue_test();

    let mut a = vec![6,5,4,3,2,1];
    let mut b = a.iter_mut().enumerate().for_each(|(idx, elem)| {
        if idx < 3 {
            *elem += 100;
        }
    });
    println!("{:?}", a);
}
