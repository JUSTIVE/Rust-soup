fn fact(x:i32)->i32 {
    match x{
        1=>1,
        a=>return a * fact(a-1)
    }
}
fn main() {
    println!("{}",fact(5))
}
