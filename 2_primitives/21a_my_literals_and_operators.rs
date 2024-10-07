fn main() {
    //bitwise stuff im least familiar with
    println!("00110 AND 01011 is {:04b}", 0b00110u32 & 0b01011);
    println!("00000 OR 01111 is {:04b}", 0b00000u32 | 0b01111);
    println!("10101 XOR 01010 is {:04b}", 0b10101u32 ^ 0b01010);
    println!("2 << 5 is {}", 2u32 << 5); // this is left shift
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // this is right shift
}