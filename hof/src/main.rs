fn twice<A>(f: impl Fn(A)->A) -> impl Fn(A)-> A{
    move |a| f(f(a))
}

fn add_three(x:i32)->i32{
    x+3
}

fn main() {
    println!("{}",twice(add_three)(3));
}
