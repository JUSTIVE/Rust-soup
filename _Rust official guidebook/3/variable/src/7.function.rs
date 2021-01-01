fn main() {
    println!("main function");

    another_function();
}
//* rust uses snake_case for function names.

//* declaration order of a function is not important, only matters where it declared
fn another_function() {
    println!("another function");
}
