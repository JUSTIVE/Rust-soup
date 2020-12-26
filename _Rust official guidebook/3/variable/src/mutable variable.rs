fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);
    x = 6; //* x is declared as mutable variable, it is valid assignment
    println!("x의 값: {}", x);
}

/*
const vs let
immutable(forever) / mutable(immutable by default)
must define its type / type can be inferred from context
can assign const expression / can assign any expression
*/
