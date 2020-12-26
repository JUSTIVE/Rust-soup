fn main() {
    let x = 5;
    println!("x의 값: {}", x);
    x = 6; //* cannot assign twice to immutable variable, it won't be run
    println!("x의 값: {}", x);
}
