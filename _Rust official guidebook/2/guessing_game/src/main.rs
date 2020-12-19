use rand::Rng;
use std::any::type_name;
use std::cmp::Ordering;
use std::io; //* load io library
///* use random library

fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

fn main() {
  //* declare & implement main function
  println!("let the game begin!");
  println!("write the number you guessed");
  let secret_number = rand::thread_rng().gen_range(1, 101); //* use random seed from OS
  println!("answer is {}", secret_number);
  loop {
    let mut guess = String::new(); //* create mutable variable
                                   //* values from [)
    io::stdin()
      .read_line(&mut guess) //* Returns Result type
      //* Result is Union type of
      //* | Ok
      //* | Err
      .expect("failed to read value");
    //* program will run without line above, but will show an error

    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    //* shadowing is possible in rust, override name 'guess from here
    println!("input value {} {}", guess, type_of(&guess));
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("input value is smaller than it thought"),
      Ordering::Greater => println!("input value is larger than it thought"),
      Ordering::Equal => {
        println!("exactly!");
        break;
      }
    }
  }
}
