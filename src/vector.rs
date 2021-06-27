use std::cmp::Ordering::{Less, Greater};

pub fn print_vec(vec: &Vec<i32>) {
    println!("@@@@@@@@@@@@@@@");
    println!("len:{} elements:{:?}", vec.len(), vec);
}

#[allow(unused)]
pub fn vec_test() {
    /** Initialize */
    let mut vec1 = vec![0;5];
    let mut vec2:Vec<i32> = Vec::new();
    let mut vec3 = Vec::from(vec1.clone()); // This will move the elements in parenthesis
    let mut vec4 = Vec::from(&vec1[1..3]);  // Construct from slice

    /** Add element */
    vec1.push(3);
    vec1.insert(0,2);
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

    for (idx, elem) in vec1.iter().enumerate() {

    }

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
}