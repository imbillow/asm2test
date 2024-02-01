use hex::ToHex;

fn main() {
    let x = 0b11111011101000001111111111111111_u32.to_le_bytes();
    println!("adf\t{}", hex::encode(&x));

    let x = 0b11111000110111101111111111111111_u32.to_le_bytes();
    println!("bins\t{}", hex::encode(&x));

    let x = 0b11111011010000101111111111100000_u32.to_le_bytes();
    println!("bsh\t{}", hex::encode(&x));

    let x = 0b11111011010000001111111111100000_u32.to_le_bytes();
    println!("bsw\t{}", hex::encode(&x));

    let x = 0b0000001000111111_u16.to_le_bytes();
    println!("callt\t{}", hex::encode(&x));

    let x = 0b11111000111011101111111111111111_u32.to_le_bytes();
    println!("caxi\t{}", hex::encode(&x));

    let x = 0b11110001011000001111111111111111_u32.to_le_bytes();
    println!("d \"cll\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b00000001010001000000011111100000_u32.to_le_bytes();
    println!("d \"ctret\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111011001111101111111111111111_u32.to_le_bytes();
    println!("d \"cmov\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b11111011000111101111111111111111_u32.to_le_bytes();
    println!("d \"cmov\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b00000001010010000000011111100000_u32.to_le_bytes();
    println!("d \"eiret\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b00000001010010100000011111100000_u32.to_le_bytes();
    println!("d \"feret\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b0111100001000000_u16.to_le_bytes();
    println!("d \"fetrap\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111011010001101111111111100000_u32.to_le_bytes();
    println!("d \"hsh\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111011010001001111111111100000_u32.to_le_bytes();
    println!("d \"hsw\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111011011110000000011111111111_u32.to_le_bytes();
    println!("d \"ldl.w\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111011011110100000011111111111_u32.to_le_bytes();
    println!("d \"stc.w\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11110011110111101111111111111111_u32.to_le_bytes();
    println!("d \"mac\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11110011111111101111111111111111_u32.to_le_bytes();
    println!("d \"macu\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111001011000000100011111111111_u32.to_le_bytes();
    println!("d \"pushsp\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111001011000000110011111111111_u32.to_le_bytes();
    println!("d \"popsp\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b0000000001000000_u16.to_le_bytes();
    println!("d \"rie\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b00000000000000001111111111111111_u32.to_le_bytes();
    println!("d \"rie\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111000110001001111111111111111_u32.to_le_bytes();
    println!("d \"rotl\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111000110001101111111111111111_u32.to_le_bytes();
    println!("d \"rotl\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111011100111101111111111111111_u32.to_le_bytes();
    println!("d \"sbf\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b11111011011001001111111111100000_u32.to_le_bytes();
    println!("d \"sch0l\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b11111011011000001111111111100000_u32.to_le_bytes();
    println!("d \"sch0r\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b11111011011001101111111111100000_u32.to_le_bytes();
    println!("d \"sch1l\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b11111011011000101111111111100000_u32.to_le_bytes();
    println!("d \"sch1r\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b00000001001000000000111111100000_u32.to_le_bytes();
    println!("d \"snooze\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b0000000000011101_u16.to_le_bytes();
    println!("d \"synce\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b0000000000011100_u16.to_le_bytes();
    println!("d \"synci\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b0000000000011110_u16.to_le_bytes();
    println!("d \"syncm\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b0000000000011111_u16.to_le_bytes();
    println!("d \"syncp\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b0000000001011111_u16.to_le_bytes();
    println!("d \"switch\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b0000000010111111_u16.to_le_bytes();
    println!("d \"sxb\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b0000000011111111_u16.to_le_bytes();
    println!("d \"sxh\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b0000000010011111_u16.to_le_bytes();
    println!("d \"zxb\" {} 0x100000 ()", hex::encode(&x));
    let x = 0b0000000011011111_u16.to_le_bytes();
    println!("d \"zxh\" {} 0x100000 ()", hex::encode(&x));

    let x = 0b00111001011000001101011111111111_u32.to_le_bytes();
    println!("d \"syscall\" {} 0x100000 ()", hex::encode(&x));
}