fn main() {
    let spaces = "     ";
    let spaces = spaces.len(); //* shadowing with different type, valid

    let mut space = "     ";
    space = space.len(); //* assigning with different type, invalid
//* ^^^^^ expected &str, found usize
}
