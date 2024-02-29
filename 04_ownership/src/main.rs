// base type don't have ownership
fn _base_type_ownership() {
    let x = 5;
    let _y = x;
}

// compound type have ownership
fn _compound_type_ownership() {
    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{}, world!", s1); // error: value borrowed here after move
}

fn _clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn _takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn _makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn _function_ownership() {
    let s = String::from("hello");
    _takes_ownership(s);
    // println!("{}", s); // error: value borrowed here after move
    
    let x = 5;
    _makes_copy(x);
    println!("{}", x);
}

fn _gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn _takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn _function_return_ownership() {
    let _s1 = _gives_ownership();
    let s2 = String::from("hello");
    let _s3 = _takes_and_gives_back(s2);
}

fn main() {
    // _base_type_ownership();
    // _compound_type_ownership();
    // _function_ownership();
    _function_return_ownership();
}
