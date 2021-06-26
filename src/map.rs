use std::collections::HashMap;

#[allow(unused)]
pub fn map_test() {
    /** Initialize */
    let mut map1:HashMap<i32,i32> = HashMap::new();
    let mut map2 = HashMap::from(map1.clone());

    /** Add element */
    map1.insert(0,1);
    map1.insert(1,2);
    map1.insert(2,3);
    map1.insert(0,4);
    println!("map1{:?}", map1);

    /** Remove element */
    map1.remove(&9).unwrap_or(3); // This returns the value to the removed key
    map1.remove(&0).unwrap();
    println!("map1{:?}",map1);

    /** Access element */
    map1.get(&1).unwrap();
    map1.get(&2).unwrap();

    /** Check if key exists */
    map1.contains_key(&0);

    /** Check if val exists */
    for entry in &map1 {

    }

    for (idx, enrtry) in map1.iter().enumerate() {
        // check entry.0

    }

    /** Compare maps */
    let mut map3 = HashMap::from(map1.clone());
    println!("{}",map1.eq(&map3));
}