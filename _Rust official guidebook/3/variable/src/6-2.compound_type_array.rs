fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5]; //* with type annotation

    //* access with index
    let first = a[0];
    let second = a[1];

    //* invalid indicies will invoke compile-time panic:
    //* which is one of safety principle of rust
}
