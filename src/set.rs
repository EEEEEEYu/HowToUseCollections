use std::collections::HashSet;

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
    println!("set1{:?}", set1);

    /** Remove element */
    set1.remove(&9);
    set1.remove(&0);
    println!("set1{:?}",set1);

    /** Access element */
    set1.get(&1).unwrap();
    set1.get(&2).unwrap();

    /** Check if key exists */
    set1.contains(&0);

    /** Check if val exists */
    for entry in &set1 {

    }

    for (idx, enrtry) in set1.iter().enumerate() {
        // check entry.0

    }

    /** Compare maps */
    let mut map3 = HashSet::from(set1.clone());
    println!("{}",set1.eq(&map3));
}