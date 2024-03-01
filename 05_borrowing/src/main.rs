fn _borrowing() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x: {}", x);
    // The println! macro automatically dereferences references when encountered, printing the actual value pointed to by the reference
    println!("y: {}", y);
    // You can use the {:p} format specifier to print the pointer
    println!("y: {:p}", y);
}

fn _not_mut_borrowing() {
    let x = 5;
    let y = &x;
    // *y += 1; // error[E0594]: cannot assign to `*y` because it is borrowed
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn _mut_borrowing_error() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    // assert_eq!(6, x);
    assert_eq!(6, *y);
}

/**
 * Changing the order of assert_eq!(6, *y); and assert_eq!(6, x);
 * can avoid violating Rust's borrowing rules because Rust's borrowing rules impose strict requirements on the order of reference usage.
 * ensuring that the mutable reference *y is used before the immutable reference x, it does not violate the borrowing rules.
 * The scope of a reference begins from its creation and lasts until the last usage of it
 * which is different from the scope of a variable, which lasts from its creation to a certain set of braces.
 */
fn _mut_borrowing_success() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    assert_eq!(6, *y);
    assert_eq!(6, x);
}

fn _mut_borrowing_once() {
    let mut s = String::from("hello");
    let _r1 = &mut s;
    // let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", _r1, r2);
}

fn main() {
    // _borrowing();
    // _not_mut_borrowing();
    // _mut_borrowing_error();
    // _mut_borrowing_success();
    // _mut_borrowing_once();
}
