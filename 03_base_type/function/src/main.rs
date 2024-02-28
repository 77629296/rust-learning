fn _function_parameter(x: i32) {
    println!("x = {}", x);
}

fn _function_statement_and_return(x: i32) -> i32 {
    if x > 10 {
        return x - 5;
    }
    x + 5
}

fn _not_have_return_value_implicit() {
    println!("This function not have return value implicit");
}

fn _not_have_return_value_explicit() -> () {
    println!("This function not have return value explicit");
}

fn _diverge_function() -> ! {
    panic!("This function diverge");
}

fn main() {
    // _function_parameter(10);
    // println!("x = {}", _function_statement_and_return(11));
    // println!("x = {}", _function_statement_and_return(10));
    // _not_have_return_value_implicit();
    // _not_have_return_value_explicit();
    _diverge_function();
}
