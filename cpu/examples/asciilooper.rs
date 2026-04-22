use cpu::Hart;


fn main() {
    let mut hart = Hart::new();
    let mut ram = [0u8; 65536];
    ram[0] = 0o231; // lui x3, 128
    ram[1] = 0;
    ram[2] = 0o022; // li x2, 31
    ram[3] = 0x7c;
    ram[4] = 0o011; // lui x1, 256
    ram[5] = 0x1;
    ram[6] = 0o022; // 1: addi x2, x2, 1
    ram[7] = 0x5;
    ram[8] = 0o204; // sb x2, x1
    ram[9] = 0x8;
    ram[10] = 0o163; // blt x2, x3, 1b
    ram[11] = 0xed;
    ram[12] = 7; // jalr zero, zero, 0
    ram[13] = 0;
    while hart.step(&mut |addr, value, we| {
        if we {
            ram[addr as usize] = value;
            if addr == 0x100 {
                print!("{}", value as char);
            }
            value
        } else {
            ram[addr as usize]
        }
    }) {}
}
