// Sha1 checksum calc
use hex_literal::hex;
use sha1::{Sha1, Digest};

// OS file access
use std::env;
use std::fs;

const BT_US_10_ROM_SHA1: [u8; 20] = hex!("af1a89e12b638b8d82cc4c085c8e01d4cba03fb3");

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_file_path = &args[1];

    if check_rom_checksum(rom_file_path.clone()) {
        println!("Yay");
    } else {
        println!("Nay");
    }
}

fn check_rom_checksum(file_path: String) -> bool {
    let rom_content = fs::read(file_path)
        .expect("Could not read provided file");

    let mut hasher = Sha1::new();

    hasher.update(rom_content);

    let result = hasher.finalize();

    result[..] == BT_US_10_ROM_SHA1
}
