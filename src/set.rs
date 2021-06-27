use std::collections::HashSet;

pub fn print_set(set: &HashSet<i32>) {
    println!("$$$$$$$$$$$$$$$$$");
    println!("size: {} elements: {:?}", set.len(), set);
}

#[allow(unused)]
pub fn set_test() {
    /** Initialize */
    let mut set1:HashSet<i32> = HashSet::new();
    let mut set2= HashSet::from(set1.clone());

    /** Add element */
    set1.insert(0);
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    print_set(&set1);

    /** Remove element */
    set1.remove(&9); // If value does not exists, return false
    set1.remove(&0); // Pass in reference can reduce copy overhead
    print_set(&set1);

    /** Access element */
    set1.get(&1).unwrap(); // There is only Option return
    set1.get(&2).unwrap(); // Pass in reference reduces copy overhead

    /** Check if val exists */
    set1.contains(&0);

    /** Traverse set*/
    for elem in &set1 {

    }

    /** Merge set */
    set1.extend(set2.clone());

    /** Compare map */
    let mut map3 = HashSet::from(set1.clone());
    println!("{}",set1.eq(&map3));
}