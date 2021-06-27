use std::collections::HashMap;

pub fn print_map(map: &HashMap<i32, i32>) {
    println!("###############");
    println!("size: {} elements: {:?}", map.len(), map);
}

#[allow(unused)]
pub fn map_test() {
    /** Initialize */
    let mut map1:HashMap<i32,i32> = HashMap::new();
    let mut map2 = HashMap::from(map1.clone());

    /** Add element */
    map1.insert(0,1);
    map1.insert(1,2);
    map1.insert(2,3);
    map1.insert(0,4);   // The old value 1 is returned
    print_map(&map1);

    /** Remove element */
    map1.remove(&9).unwrap_or(3); // This returns the value to the removed key
    map1.remove(&0).unwrap();
    print_map(&map1);

    /** Access element */
    map1.get(&1).unwrap();
    map1.get(&2).unwrap();

    /** Check if key exists */
    map1.contains_key(&0);

    /** Check if val exists */
    for entry in &map1 {
        // entry.1
    }

    for val in map1.values() {

    }

    /** Merge maps */
    // This moves all elments from map2 to map1
    map1.extend(map2.clone());


    /** Compare maps */
    let mut map3 = HashMap::from(map1.clone());
    println!("{}",map1.eq(&map3));
}