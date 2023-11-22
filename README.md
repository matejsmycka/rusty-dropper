# Rusty Dropper

Rusty dropper is a simple malware loader written in Rust.

Shellcode is encrypted in compile time using XOR. After running, the shellcode is decrypted and moved directly into memory.
Another thread is created, which executes the code in memory. However, both are done inside one process, which is not persistent.

This code serves as POC. I do not hold any responsibility for malicious misuse.

The `shellcode.exe` included in the repo is a script that runs a calculator generated by mfsvenom.

Command: `msfvenom -p windows/exec cmd=calc.exe -f c -e x86/alpha_mixed`

## Detection

The final binary is indetectable with Windows Defender. However, more advanced EDR can raise alerts.

Here is a comparison between the original shellcode and the shellcode bundled via dropper.
For a comparison, I used [VirusTotal](https://www.virustotal.com/).

**Original:**

![Original binary](https://i.imgur.com/rkBdF8Z.png)

**New:**

![New binary](https://i.imgur.com/3NRilpC.png)

## Usage

1. Copy your shellcode to root directory of this project.
2. Rename it to `shellcode.exe`
3. `cargo build --release`
4. Run `target/release/rusty-sheller.exe`
5. Rename binary into something less suspicious
6. Deploy
