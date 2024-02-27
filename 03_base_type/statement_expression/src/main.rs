fn _expression(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 5;
    // cannot end with ; because it will become a statement
    x + y
}

fn _return_unit_type() {
    let x = 1;
    let y = if x % 2 == 1 {
        1
    } else {
        2
    };
}

fn _cannot_assigned_let_statement() {
    // let x = 5;
    // let y = (let z = 6); // error: expected expression, found statement (`let`)
}

fn main() {
    // println!("The value of x is: {}", _expression(5, 6));
    // _cannot_assigned_let_statement();
    assert_eq!(_return_unit_type(), ());
}
