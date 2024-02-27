fn _char_type() {
    let c = 'z';
    let z = 'ℤ';
    let zh = '中';
    let heart_eyed_cat = '😻';
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
    println!("字符 中 占用了: {}字节的内存大小", std::mem::size_of_val(&zh));
}

fn _bool_type() {
    let t = true;
    let f: bool = false;
    println!("t: {}", t);
    println!("f: {}", f);
}

fn main() {
    // _char_type();
    _bool_type();
}
