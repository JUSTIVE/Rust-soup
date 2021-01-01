fn main() {
    another_function(5);
    another_function2(5, 6);
}

fn another_function(x: i32) {
    println!("x의 값: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}
