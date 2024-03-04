#![allow(dead_code)]

fn pattern_matching() {
    let tup = (1, 2, 3);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn visiting_tuple() {
    let tup = (1, 2, 3);
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn return_multiple_values() -> () {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn main() {
    // pattern_matching();
    // visiting_tuple();
    return_multiple_values();
}
