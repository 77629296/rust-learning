fn _immutable() {
    let x = 5;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable
    // x = 6;
    // println!("The value of x is: {}", x);
}

fn _mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn _destructuring() {
    let (a, mut b): (bool, bool) = (true, false);
    // {:?} provides more detailed output, suitable for debugging and error reporting, while {} provides more concise output, suitable for display to the end user.
    // println!("a = {}, b = {}", a, b);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
}

struct _Struct {
    e: i32,
}
fn _destructuring_struct() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ represents matching a value, but we don't care about the specific value, so instead of using a variable name, we use _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    _Struct { e, .. } = _Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn _constants() {
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn _shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn main() {
    // _immutable();
    // _mutable();
    // _destructuring();
    // _destructuring_struct();
    // _constants();
    _shadowing();
}
