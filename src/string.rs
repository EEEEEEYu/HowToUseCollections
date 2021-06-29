
#[allow(unused)]
pub fn string_test() {
    /** Initialize */
    let mut str1 = String::new();
    let mut str2 = String::from("Hello");

    /* Add elements */
    str1.push('a');
    str1.push_str(&str2);

    /** Remove */
    str1.trim(); // Remove head and tail whitespace
    str1.pop().unwrap(); // Remove the last char
    str1.remove(0); // Remove 1 char at given index
    str1.replace("Hello", "hello"); // Replace substring
    str1.replace_range(1..3, "###"); // Replace range of substring.

    /** Access */
    str1.find("llo").unwrap(); // Return the index of the first matched
    str1.chars().nth(0).unwrap();
    str1.contains("llo");
    str1.contains(|c:char| { // If a char matches specific requirements
        if c < 'a' {
            return true;
        }
        return false;
    });

    /** Traverse */
    for char in str1.chars() {
        //
    }

    for (i, char) in str1.chars().enumerate() {

    }


    /** Sort */
    unsafe {
        str1.as_mut_vec().sort();
    }

    //* Merge elements */
    str1 += &str2;
    // str1.add(&str2); // This is the same action as above, consumes str1 itself
    str1.insert(0,'K');
    str1.insert_str(1, "%%%");

    /** Compare */
    str1.cmp(&str2);
    str1.eq(&str2);
}

