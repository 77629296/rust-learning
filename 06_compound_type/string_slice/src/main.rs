#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
fn greet_string(name: String) {
    println!("greet_string, {}", name);
}

fn greet_str(name: &str) {
    println!("greet_str, {}", name);
}

fn slice_string() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("{} {}", hello, world);
}

fn slice_chinese() {
    let s = String::from("你好，世界！");
    let hello = &s[0..3];
    // byte index 4 is not a char boundary; it is inside '好' (bytes 3..6) of `你好，世界！`
    let world = &s[3..6];
    println!("{} {}", hello, world);
}

fn slice_mut_error() {
    let mut s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..12];
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.clear();
    println!("{} {}", hello, world);
}

fn string_to_str() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..12];
    greet_str(hello);
    greet_str(world);
    greet_str(s.as_str());
}

fn string_index() {
    let s = String::from("Hello, world!");
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // let h = s[0];
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // let w = s[7];
    // println!("{} {}", h, w);
}

fn string_push() {
    let mut s = String::from("Hello, ");
    // push_str(&mut self, string: &str)
    s.push_str("world");
    // push(&mut self, ch: char)
    s.push('!');
    println!("{}", s);
}

fn string_insert() {
    let mut s = String::from("Hello world!");
    // insert(&mut self, idx: usize, ch: char)
    s.insert(5, ',');
    // insert_str(&mut self, idx: usize, string: &str)
    s.insert_str(6, " by_insert_str ");
    println!("{}", s);
}

fn string_replace() {
    let mut s = String::from("Hello, world!");
    // replace(&mut self, range: Range<usize>, new: &str) -> String
    let new_s = s.replace("world", "世界");
    println!("{}", new_s);
}

fn string_replacen() {
    let mut s = String::from("Hello, world!");
    // replacen(&mut self, pat: &str, rep: &str, count: usize) -> String
    let new_s = s.replacen("o", "O", 1);
    println!("{}", new_s);
}

fn string_replace_range() {
    let mut s = String::from("Hello, world!");
    // replace_range(&mut self, range: Range<usize>, new: &str)
    s.replace_range(7..13, "世界");
    println!("{}", s);
}

fn string_pop() {
    let mut s = String::from("Hello, world!");
    // pop(&mut self) -> Option<char>
    let c = s.pop();
    // println!("{}", c.unwrap());
    dbg!(c);
    dbg!(s);
}

fn string_remove() {
    let mut s = String::from("Hello, world!");
    // remove(&mut self, idx: usize) -> char
    let c = s.remove(5);
    println!("{}", c);
    println!("{}", s);
}

fn string_truncate() {
    let mut s = String::from("Hello, world!");
    // truncate(&mut self, new_len: usize)
    s.truncate(5);
    println!("{}", s);
}

fn string_clear() {
    let mut s = String::from("Hello, world!");
    // clear(&mut self)
    s.clear();
    dbg!(s);
}

fn string_add() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // add(self, other: &str) -> String
    let s3 = s1 + &s2;
    println!("{}", s3);
}

fn string_format() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // format_args!(args: Arguments) -> String
    let s3 = format!("{}{}", s1, s2);
    println!("{}", s3);
}

fn string_escape() {
    let s = String::from("Hello, \"world\"!");
    // escape_debug(&self) -> String
    let s1 = s.escape_debug();
    println!("{}", s1);
    // escape_default(&self) -> String
    let s2 = s.escape_default();
    println!("{}", s2);
    // escape_unicode(&self) -> String
    let s3 = s.escape_unicode();
    println!("{}", s3);
}

fn main() {
    // greet_string("Hello, world!");
    // greet_str("Hello, world!");
    // slice_string();
    // slice_chinese();
    // slice_mut_error();
    // string_to_str();
    // string_index();
    // string_push();
    // string_insert();
    // string_replace();
    // string_replacen();
    // string_replace_range();
    // string_pop();
    // string_remove();
    // string_truncate();
    // string_clear();
    // string_add();
    // string_format();
    string_escape();
}
