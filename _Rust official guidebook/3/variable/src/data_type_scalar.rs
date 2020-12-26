fn main() {
    //* from guessing_game
    let guess: u32 = "42".parse().expect("is not valid number");
    //* without type specification, will invoke error, due to uninferrable type
}
