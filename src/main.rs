use compression::decompressor;
// Sha1 checksum calc
use hex_literal::hex;
use sha1::{Sha1, Digest};

// OS file access
use std::env;
use std::fs;

// Our modules
mod compression;

const BT_US_10_ROM_SHA1: [u8; 20] = hex!("af1a89e12b638b8d82cc4c085c8e01d4cba03fb3");

fn main() {
    // Get call arguments to acquire file path to ROM
    let arg1 = parse_arguments();
    let rom_file_path = match arg1 {
        Result::Ok(x) => x,
        Result::Err(x) => panic!("{}", x)
    };

    // Load provided file and check if it's actually BT US 1.0
    let rom_content = read_rom_content(&rom_file_path);
    let rom_content = match rom_content {
        Result::Ok(x) => x,
        Result::Err(x) => panic!("{}", x)
    };

    decompressor::decompress_single_file(&rom_content, rom_file_path, 0xC359FC, 0xC35A3C);
}

fn parse_arguments() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("No filepath to BT ROM provided".to_owned())
    } else {
        Ok((args[1]).clone())
    }
}

fn read_rom_content(file_path: &String) -> Result<Vec<u8>, String> {
    println!("Read provided file ...");

    let rom_content = fs::read(file_path)
        .expect("Could not read provided file");

    if check_rom_checksum(&rom_content) {
        println!("ROM ok");
        Ok(rom_content)
    } else {
        Err("Provided file does not match expected BT US 1.0 ROM checksum".to_owned())
    }
}

fn check_rom_checksum(rom_content: &Vec<u8>) -> bool {
    println!("Check provided file checksum ...");

    let mut hasher = Sha1::new();
    hasher.update(rom_content);

    let result = hasher.finalize();

    result[..] == BT_US_10_ROM_SHA1
}
