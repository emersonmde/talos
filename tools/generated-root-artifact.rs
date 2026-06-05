use std::{env, fs, process};

const MAGIC: &[u8; 16] = b"TALOSROOTV1\0\0\0\0\0";
const HEADER_LEN: usize = 40;
const ENTRY_HEADER_LEN: usize = 12;
const DIGEST_OFFSET: usize = 32;
const VERSION: u32 = 1;
const ENTRY_FILE: u8 = 1;
const FLAG_EXECUTABLE: u8 = 1;
const EXEC_ELF_LEN: usize = 0x204;
const TEXT_OFFSET: usize = 0x100;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("usage: {} <a|b|malformed> <artifact-path>", args[0]);
        process::exit(2);
    }

    let artifact = match args[1].as_str() {
        "a" => build_artifact(b"Talos generated-root external artifact A\n", 7),
        "b" => build_artifact(b"Talos generated-root external artifact B\n", 9),
        "malformed" => {
            let mut artifact = build_artifact(b"Talos generated-root malformed artifact\n", 11);
            artifact[DIGEST_OFFSET] ^= 0xff;
            artifact
        }
        other => {
            eprintln!("unsupported generated-root artifact variant: {other}");
            process::exit(2);
        }
    };

    fs::write(&args[2], artifact).expect("write generated-root artifact");
}

fn build_artifact(file_contents: &[u8], exit_status: u64) -> Vec<u8> {
    let exec = build_exit_elf(exit_status);
    let mut bytes = Vec::new();
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    bytes.extend_from_slice(&(HEADER_LEN as u32).to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&2u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    push_entry(&mut bytes, b"/generated/manifest.txt", file_contents, false);
    push_entry(&mut bytes, b"/generated/status7", &exec, true);

    let total_len = bytes.len() as u32;
    bytes[24..28].copy_from_slice(&total_len.to_le_bytes());
    let digest = artifact_digest(&bytes);
    bytes[DIGEST_OFFSET..DIGEST_OFFSET + 8].copy_from_slice(&digest.to_le_bytes());
    bytes
}

fn push_entry(bytes: &mut Vec<u8>, path: &[u8], contents: &[u8], executable: bool) {
    bytes.push(ENTRY_FILE);
    bytes.push(if executable { FLAG_EXECUTABLE } else { 0 });
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&(path.len() as u16).to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&(contents.len() as u32).to_le_bytes());
    debug_assert_eq!(ENTRY_HEADER_LEN, 12);
    bytes.extend_from_slice(path);
    bytes.extend_from_slice(contents);
}

fn artifact_digest(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for (index, byte) in bytes.iter().enumerate() {
        let byte = if (DIGEST_OFFSET..DIGEST_OFFSET + 8).contains(&index) {
            0
        } else {
            *byte
        };
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3) ^ byte as u64;
    }
    hash
}

fn build_exit_elf(exit_status: u64) -> [u8; EXEC_ELF_LEN] {
    const EHDR_LEN: usize = 64;
    const PHENT_LEN: usize = 56;
    const DATA_OFFSET: usize = 0x200;
    const TEXT_VADDR: u64 = 0x0000_0000_0001_0100;
    const DATA_VADDR: u64 = 0x0000_0000_0002_0200;
    const ENTRY: u64 = TEXT_VADDR;
    const PF_X: u32 = 1;
    const PF_W: u32 = 2;
    const PF_R: u32 = 4;
    const PAGE_ALIGN: u64 = 0x1000;

    let mut bytes = [0u8; EXEC_ELF_LEN];
    bytes[0] = 0x7f;
    bytes[1] = b'E';
    bytes[2] = b'L';
    bytes[3] = b'F';
    bytes[4] = 2;
    bytes[5] = 1;
    bytes[6] = 1;

    write_le_u16(&mut bytes, 16, 2);
    write_le_u16(&mut bytes, 18, 183);
    write_le_u32(&mut bytes, 20, 1);
    write_le_u64(&mut bytes, 24, ENTRY);
    write_le_u64(&mut bytes, 32, EHDR_LEN as u64);
    write_le_u16(&mut bytes, 52, EHDR_LEN as u16);
    write_le_u16(&mut bytes, 54, PHENT_LEN as u16);
    write_le_u16(&mut bytes, 56, 2);

    write_load_phdr(
        &mut bytes,
        EHDR_LEN,
        PF_R | PF_X,
        TEXT_OFFSET as u64,
        TEXT_VADDR,
        8,
        8,
        PAGE_ALIGN,
    );
    write_load_phdr(
        &mut bytes,
        EHDR_LEN + PHENT_LEN,
        PF_R | PF_W,
        DATA_OFFSET as u64,
        DATA_VADDR,
        4,
        0x1004,
        PAGE_ALIGN,
    );

    let exit_status = (exit_status & 0xffff) as u32;
    let movz_x0 = 0xd280_0000u32 | (exit_status << 5);
    bytes[TEXT_OFFSET] = movz_x0 as u8;
    bytes[TEXT_OFFSET + 1] = (movz_x0 >> 8) as u8;
    bytes[TEXT_OFFSET + 2] = (movz_x0 >> 16) as u8;
    bytes[TEXT_OFFSET + 3] = 0xd2;
    bytes[TEXT_OFFSET + 4] = 0x01;
    bytes[TEXT_OFFSET + 5] = 0x42;
    bytes[TEXT_OFFSET + 6] = 0x0f;
    bytes[TEXT_OFFSET + 7] = 0xd4;
    bytes[DATA_OFFSET..DATA_OFFSET + 4].copy_from_slice(b"DATA");
    bytes
}

fn write_load_phdr(
    bytes: &mut [u8; EXEC_ELF_LEN],
    offset: usize,
    flags: u32,
    file_offset: u64,
    virtual_address: u64,
    file_size: u64,
    memory_size: u64,
    alignment: u64,
) {
    write_le_u32(bytes, offset, 1);
    write_le_u32(bytes, offset + 4, flags);
    write_le_u64(bytes, offset + 8, file_offset);
    write_le_u64(bytes, offset + 16, virtual_address);
    write_le_u64(bytes, offset + 24, virtual_address);
    write_le_u64(bytes, offset + 32, file_size);
    write_le_u64(bytes, offset + 40, memory_size);
    write_le_u64(bytes, offset + 48, alignment);
}

fn write_le_u16(bytes: &mut [u8; EXEC_ELF_LEN], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_le_u32(bytes: &mut [u8; EXEC_ELF_LEN], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_le_u64(bytes: &mut [u8; EXEC_ELF_LEN], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}
