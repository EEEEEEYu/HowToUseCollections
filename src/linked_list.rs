use std::collections::LinkedList;

pub fn print_linked_list(list:&LinkedList<i32>) {
    println!("&&&&&&&&&&&&&&&&&&&&&");
    println!("len:{} elements:{:?}", list.len(), list);
}

#[allow(unused)]
pub fn linked_list_test() {
    /** Initialize */
    let mut linked_list1 = LinkedList::new();

    /** Add elements */
    linked_list1.push_back(0);
    linked_list1.push_front(1);
    linked_list1.push_back(2);
    linked_list1.push_back(3);
    print_linked_list(&linked_list1);

    /** Remove elements */
    linked_list1.pop_back();
    linked_list1.pop_front();
    print_linked_list(&linked_list1);

    /** Access elements */
    linked_list1.front().unwrap();
    linked_list1.back().unwrap();

    /** Traverse elements */
    for elem in &linked_list1 {
        //
    }

    /** Merge elements */
    let mut linked_list2 = linked_list1.clone();
    linked_list1.append(&mut linked_list2.clone());
    linked_list1.extend(linked_list2.clone());
    print_linked_list(&linked_list1);

    /** Compare elements */
    linked_list1.eq(&linked_list2);
}