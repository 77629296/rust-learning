fn greet_world() {
    let germany_greeting = "Grüß Gott!";
    let chinese_greeting = "你好";
    let english_greeting = "Hello";
    let regions = [germany_greeting, chinese_greeting, english_greeting];
    for region in regions.iter() {
        println!("{}", region);
    }
}

fn main() {
    greet_world();
}
