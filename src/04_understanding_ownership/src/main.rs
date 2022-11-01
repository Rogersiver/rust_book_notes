fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s is: {s}");
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//finds index of first word
fn first_word(s: &String) -> &str {
    //convert to array of bytes
    let bytes = s.as_bytes();
    // create an iterator over the array of bytes using the iter method
    for  (i, &item) in bytes.iter().enumerate() {
        //use byte literal syntax to find a space to return the position of space
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
