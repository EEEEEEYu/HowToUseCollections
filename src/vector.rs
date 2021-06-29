use std::cmp::Ordering::{Less, Greater};
use std::collections::{LinkedList, HashMap};

pub fn print_vec(vec: &Vec<i32>) {
    println!("@@@@@@@@@@@@@@@");
    println!("len:{} elements:{:?}", vec.len(), vec);
}

#[allow(unused)]
pub fn vec_test() {
    /** Initialize */
    let mut vec1 = vec![0; 5];
    let mut vec2: Vec<i32> = Vec::new();
    let mut vec3 = Vec::from(vec1.clone()); // This will move the elements in parenthesis
    let mut vec4 = Vec::from(&vec1[1..3]);  // Construct from slice

    /** Add element */
    vec1.push(3);
    vec1.insert(0, 2);
    // vec1.clear();
    print_vec(&vec1);

    /** Remove element */
    vec1.pop();
    vec1.remove(3);
    print_vec(&vec1);

    /** Access element */
    vec1[0]; // This will panic if index out of bound
    vec1.get(0).unwrap(); // This is safe, but with some more overhead

    /** Traverse element */
    for elem in &vec1 { // The '&' must be added otherwise this loop will consume vector
    }

    for (idx, elem) in vec1.iter().enumerate() {}

    /** Sort element */
    vec1.sort();
    vec1.sort_by(|n1, n2| {
        if n1 < n2 {
            Less
        } else {
            Greater
        }
    });
    print_vec(&vec1);

    /** Merge element */
    vec1.append(&mut vec2); // This will clear vec2
    vec1.extend(vec2.clone());
    print_vec(&vec1);

    /* Compare vectors */
    vec3 = Vec::from(vec1.clone());
    println!("{}", vec1.eq(&vec3));

    /* Functional Programming */
    // map function returns iterator which applies the closure on each element
    for elem in vec1.iter_mut().map(|x| {
        *x += 1;
        x
    }) {}

    // filter method returns elements that satisfy the predicate
    // If you use iter instead of into_iter, this will return a vector of references of original vector values
    // The double reference is due to iter() and filter(), they both output references type
    // See https://stackoverflow.com/questions/44662312/how-to-filter-a-vector-of-custom-structs-in-rust
    let vecx: Vec<&i32> = vec1.iter().filter(|x| { **x >= 0 }).collect();
    // Use into_iter to iterate over values and filter them. But careful that into_iter takes original ownership
    // See https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter?noredirect=1&lq=1
    let vecy: Vec<i32> = vec1.clone().into_iter().filter(|x| { *x > 1 }).collect();
    // Be careful that you must explicitly tell which container the collect() method will generate
    // Also remember that iterators are lazily evaluated. No values will be generated until collect()
    let vecx: LinkedList<i32> = vec1.clone().into_iter().filter(|x| { *x > 1 }).collect();

    // any() returns bool if at least one element satisfies the predicate
    vec1.iter().any(|x| { *x > 0 });
    // all() returns bool if all elements satisfy the predicate
    vec1.iter().all(|x| { *x >= 0 });

    // merge two vectors as a hashmap
    let map1: HashMap<i32, i32> = vec1.into_iter().zip(vec2.clone().into_iter()).collect();
}