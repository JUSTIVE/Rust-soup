fn main() {
    let x = 5; //* is shadowed by line 3 declaration
    let x = x + 1; //* is shadowed by line 4 declaration
    let x = x * 2;
    println!("x의 값: {}", x);
}
