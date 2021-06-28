mod vector;
mod map;
mod set;
mod queue;
mod priority_queue;
mod linked_list;
mod string;


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
}
