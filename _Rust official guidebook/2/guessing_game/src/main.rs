use std::io;//* load io library
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main(){//* declare & implement main function
    println!("let the game begin!");
    println!("write the number you guessed");

    let mut guess = String::new();//* create mutable variable 

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read value");

    println!("input value {} {}",guess,type_of(&guess));
}