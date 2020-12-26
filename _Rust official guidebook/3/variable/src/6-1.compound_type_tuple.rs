fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("value of y:{}", y);
    println!("value of y:{}", tup.1); //* access with 0-based index
}
