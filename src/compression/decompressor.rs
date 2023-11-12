use std::{process::{Command, Stdio}, io::{BufWriter, Write}, fs::File, path::PathBuf};

pub fn decompress_single_file(rom_content: &[u8], rom_file_path: String, rom_offset_start: usize, rom_offset_end: usize) {
    let compressed_bytes = &rom_content[rom_offset_start..rom_offset_end];

    let headered_compressed_bytes = add_gzip_header(compressed_bytes);
    let prepared_compressed_bytes = remove_bt_padding(headered_compressed_bytes);

    // Create ROM dump dir if it doesn't exist yet
    let mut rom_dump_path = PathBuf::from(rom_file_path);
    rom_dump_path.pop();
    rom_dump_path.push("dump");
    if !rom_dump_path.exists() && std::fs::create_dir(&rom_dump_path).is_err() {
        panic!("Cannot create directory '{}'", &rom_dump_path.display());
    }

    let new_file_name = format!("{}/{:X}.bin.decompressed", rom_dump_path.as_path().display(), rom_offset_start);
    let stdout_target_file = File::create(new_file_name).expect("failed to write file");

    // Build handle and writer for gzip command and its stdin input
    let gzip_command = Command::new("gzip")
        .args(["-9", "-c", "-f", "-d"])
        .stdout(stdout_target_file)
        .stdin(Stdio::piped())
        .stderr(Stdio::null()) // 2> /dev/null
        .spawn()
        .unwrap();

    let mut gzip_stdin = gzip_command.stdin.unwrap();
    let mut writer_in = BufWriter::new(&mut gzip_stdin);
    writer_in.write_all(&prepared_compressed_bytes[..]).unwrap();
}

fn add_gzip_header(compressed_bytes: &[u8]) -> Vec<u8> {
    let mut headered_compressed_bytes: Vec<u8> = vec![
        0x1F,
        0x8B,
        0x08,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x02,
        0x03
    ];

    let mut i = 2;
    for comp_byte in compressed_bytes {
        // skip first 2 bytes
        if i == 0 {
            headered_compressed_bytes.push(*comp_byte);
        } else {
            i -= 1;
        }
    }

    headered_compressed_bytes
}

fn remove_bt_padding(headered_compressed_bytes: Vec<u8>) -> Vec<u8> {
    let mut trimmed_compressed_bytes: Vec<u8> = Vec::new();
    let mut found_end_of_padding = false;

    for i in headered_compressed_bytes.into_iter().rev() {
        if !found_end_of_padding && i != 0xAA {
            found_end_of_padding = true;
        }

        if found_end_of_padding {
            trimmed_compressed_bytes.push(i);
        }
    }

    trimmed_compressed_bytes.reverse();
    trimmed_compressed_bytes
}
