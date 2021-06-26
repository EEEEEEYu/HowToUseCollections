use std::cmp::Ordering::{Less, Greater};

#[allow(unused)]
pub fn vec_test() {
    /** Initialize */
    let mut vec1 = vec![0;5];
    let mut vec2:Vec<i32> = Vec::new();
    let mut vec3 = Vec::from(vec1.clone()); // This will move the original vector

    /** Add element */
    vec1.push(3);
    vec1.append(&mut vec3);
    println!("vec1:{:?}, len:{}", vec1, vec1.len());

    /** Pop tail element */
    vec1.pop();
    println!("vec1:{:?}, len:{}", vec1, vec1.len());

    /** Erase element in the middle */
    vec1.remove(0);
    vec1.remove(2);
    println!("vec1:{:?}, len:{}", vec1, vec1.len());

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

    /* Compare elements */
    vec3 = Vec::from(vec1.clone());
    println!("{}", vec1.eq(&vec3));
}