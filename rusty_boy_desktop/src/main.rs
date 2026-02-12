fn main() {
    let bytes: u8 = 0b11111111;
    println!("{:0b}", (bytes as u16) << 8);
}
