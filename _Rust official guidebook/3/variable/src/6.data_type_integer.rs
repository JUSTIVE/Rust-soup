fn main() {
    let i_8: i8 = 127;
    let i_16: i16 = 32_767;
    let i_32: i32 = 2_147_483_647; //* underscore can be used for digit seperation
    let i_64: i64 = 0; //* up to 2^63 - 1
    let i_size: isize = 0 //* regarded 32bit on 32bit system, 64bit on 64bit system

    let u_8: u8 = 255;let u_16:u16 = 65535;
    let u_16: u16 = 65535;
    let u_32: u32 = 4_294_967_295;
    let u_64: u64 = 0;//* up to 2_64 - 1
    let u_size:usize = 0;//* regarded 32bit on 32bit system, 64bit on 64bit system

    //* non-floating number literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
}
