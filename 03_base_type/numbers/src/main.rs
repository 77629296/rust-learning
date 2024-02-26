fn _wrapping_add() {
    let a: u8 = 255;
    let b: u8 = 1;
    let c = a.wrapping_add(b);
    println!("a: {}, b: {}, c: {}", a, b, c);

}

fn _checked_add() {
    let a: u8 = 255;
    let b: u8 = 1;
    let c = a.checked_add(b);
    match c {
        Some(v) => println!("a: {}, b: {}, c: {}", a, b, v),
        None => println!("a: {}, b: {}, c: {:?} is overflow", a, b, c),
    }
}

fn _overflowing_add() {
    let a: u8 = 255;
    let b: u8 = 1;
    let c = a.overflowing_add(b);
    println!("a: {}, b: {}, c: {:?}", a, b, c);
}

fn _saturating_add() {
    let a: u8 = 255;
    let b: u8 = 1;
    let c = a.saturating_add(b);
    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn _float_compare() {
    let a: f32 = 1.0;
    let b: f32 = 1.0;
    let c: f32 = 1.0000001;
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("a == b: {}", a == b);
    println!("a == c: {}", a == c);
    println!("a < c: {}", a < c);
    println!("a > c: {}", a > c);
    // panic
    // assert!(0.1 + 0.2 == 0.3);
}

fn _cannot_compare_nan() {
    let a: f32 = 0.0 / 0.0;
    let b: f32 = 0.0 / 0.0;
    println!("a: {}, b: {}", a, b);
    println!("a == b: {}", a == b);
    println!("a < b: {}", a < b);
    println!("a > b: {}", a > b);
    // panic
    assert_eq!(a, b);
}

fn _perform_calculation() {
    // The compiler will perform automatic type inference, giving 'twenty' the type i32.
    let twenty = 20;
    // Type annotation
    let twenty_one: i32 = 21;
    // Type annotation using type suffix: 22 is of type i32
    let twenty_two = 22i32;

    // Only the same types can be operated on
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // For longer numbers, you can use underscores to improve readability
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // Define an f32 array, where 42.0 will automatically be inferred as f32 type
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // Print the first value in the array, and control the decimal places to 2 digits
    println!("{:.2}", forty_twos[0]);
}

// & Bitwise AND	1 if both bits are 1, otherwise 0
// | Bitwise OR	1 if at least one bit is 1, otherwise 0
// ^ Bitwise XOR	1 if bits are different, otherwise 0
// ! Bitwise NOT	Flip each bit (0 becomes 1, 1 becomes 0)
// << Left Shift	Shifts all bits to the left by a specified number, filling in with 0 on the right
// >> Right Shift	Shifts all bits to the right by a specified number, preserving the sign (0 for positive numbers, 1 for negative numbers)
fn _bitwise_operations() {
    // Binary representation is 00000010
    let a: i32 = 2;
    // Binary representation is 00000011
    let b: i32 = 3;

    // 00000010 = 2
    println!("(a & b) value is {}", a & b);
    // 00000011 = 3
    println!("(a | b) value is {}", a | b);
    // 00000001 = 1
    println!("(a ^ b) value is {}", a ^ b);
    // 11111100
    // 00000011 Bitwise NOT + 1
    // 00000100 = 4 with the addition of a negative sign
    println!("(!b) value is {} ", !b);
    // 00010000 16 8 4 2 1
    println!("(a << b) value is {}", a << b);
    // 00000000 = 0
    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // Note that these arithmetic operators can all be combined with '=' for assignment (except '!=' which is used for inequality comparison)
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn _range() {
    // A range is a sequence of numbers with a start, end and a step
    // It is defined using the .. and ... syntax
    let inclusive_range = 1..=10;
    let exclusive_range = 1..10;

    for i in inclusive_range {
        println!("inclusive_range: {}", i);
    }

    for i in exclusive_range {
        println!("exclusive_range: {}", i);
    }

    for i in 'a'..='z' {
        println!("{} ", i);
    }
}

fn main() {
    // _wrapping_add();
    // _checked_add();
    // _overflowing_add();
    // _saturating_add();
    // _float_compare();
    // _cannot_compare_nan();
    // _perform_calculation();
    // _bitwise_operations();
    _range();
}
